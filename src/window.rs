use crate::config;
use crate::grid;
use crate::layout;
use crate::render;
use crate::state;

use smithay_client_toolkit::reexports::calloop::EventLoop;
use smithay_client_toolkit::reexports::calloop_wayland_source::WaylandSource;
use smithay_client_toolkit::{
    compositor::CompositorState,
    shell::wlr_layer::{KeyboardInteractivity, Layer, LayerShell},
};
use smithay_client_toolkit::{
    delegate_compositor, delegate_keyboard, delegate_layer, delegate_output, delegate_registry,
    delegate_seat, delegate_shm,
    output::OutputState,
    reexports::client::protocol,
    registry::ProvidesRegistryState,
    registry::RegistryState,
    registry_handlers,
    seat::SeatState,
    shell::{WaylandSurface, wlr_layer::LayerSurface},
    shm::{Shm, slot::Buffer, slot::SlotPool},
};
use wayland_client::protocol::wl_keyboard;
use wayland_client::{Connection, globals::registry_queue_init};

mod compositor;
mod keyboard;
mod layer_shell;
mod output;
mod seat;
mod shm;

const DEFAULT_SCALE: u16 = 1;

pub struct Window {
    config: config::Config,
    state: state::State,
    drawable_items: render::DrawableItems,

    registry_state: RegistryState,
    seat_state: SeatState,
    output_state: OutputState,

    layer: LayerSurface,
    keyboard: Option<wl_keyboard::WlKeyboard>,

    buffer: Option<Buffer>,
    pool: SlotPool,
    shm: Shm,

    width: u32,
    height: u32,
    scale: u16,

    font_system: cosmic_text::FontSystem,
    swash_cache: cosmic_text::SwashCache,
}

impl Window {
    fn width(&self) -> u32 {
        self.width * u32::from(self.scale)
    }

    fn height(&self) -> u32 {
        self.height * u32::from(self.scale)
    }

    pub fn new(
        config: config::Config,
        options: &[String],
    ) -> anyhow::Result<(Self, EventLoop<Window>)> {
        // All Wayland apps start by connecting the compositor (server).
        let conn = Connection::connect_to_env().unwrap();

        // Enumerate the list of globals to get the protocols the server implements.
        let (globals, event_queue) = registry_queue_init(&conn)?;
        let qh = event_queue.handle();

        let event_loop: EventLoop<Window> =
            EventLoop::try_new().expect("Failed to initialize the event loop!");
        let loop_handle = event_loop.handle();
        WaylandSource::new(conn.clone(), event_queue)
            .insert(loop_handle)
            .unwrap();

        // The compositor (not to be confused with the server which is commonly called the compositor) allows
        // configuring surfaces to be presented.
        let compositor =
            CompositorState::bind(&globals, &qh).expect("wl_compositor is not available");
        // This app uses the wlr layer shell, which may not be available with every compositor.
        let layer_shell = LayerShell::bind(&globals, &qh).expect("layer shell is not available");
        // Since we are not using the GPU in this example, we use wl_shm to allow software rendering to a buffer
        // we share with the compositor process.
        let shm = Shm::bind(&globals, &qh).expect("wl_shm is not available");

        // A layer surface is created from a surface.
        let surface = compositor.create_surface(&qh);

        // And then we create the layer shell.
        let layer = layer_shell.create_layer_surface(
            &qh,
            surface,
            Layer::Top,
            Some(crate::prog_name!()),
            None,
        );

        let grid = grid::Grid::new(options)?;

        // The logical dimensions of our layer.
        let width: u32 = grid.width as u32
            * (config.item_width + config.item_margin + 2. * config.border_width) as u32;
        let height: u32 = grid.height as u32
            * (config.item_height + config.item_margin + 2. * config.border_width) as u32;

        // Configure the layer surface, providing things like the anchor on screen, desired size and the keyboard
        // interactivity
        layer.set_keyboard_interactivity(KeyboardInteractivity::Exclusive);
        layer.set_size(width, height);

        // In order for the layer surface to be mapped, we need to perform an initial commit with no attached
        // buffer. For more info, see WaylandSurface::commit
        //
        // The compositor will respond with an initial configure that we can then use to present to the layer
        // surface with the correct options.
        layer.commit();

        // Initially we don't know the real scale. The compositor will tell us later
        let scale = DEFAULT_SCALE;
        let buffer_len = width * scale as u32 * height * scale as u32 * 4;
        let pool = SlotPool::new(buffer_len as usize, &shm).expect("Failed to create pool");

        let drawable_items = render::DrawableItems::from_grid(&config, &grid);

        let window = Window {
            config,
            state: state::State::new(grid),
            drawable_items,

            // Seats and outputs may be hotplugged at runtime, therefore we need to setup a registry state to
            // listen for seats and outputs.
            registry_state: RegistryState::new(&globals),
            seat_state: SeatState::new(&globals, &qh),
            output_state: OutputState::new(&globals, &qh),

            buffer: None,
            pool,
            shm,

            scale,
            width,
            height,
            layer,
            keyboard: None,

            // cosmic text
            font_system: cosmic_text::FontSystem::new(),
            swash_cache: cosmic_text::SwashCache::new(),
        };

        Ok((window, event_loop))
    }

    // much of this implementation is borrowed from yofi under the MIT license
    // copyright 2018 kitsu
    pub fn draw(&mut self) {
        let width = self.width().try_into().expect("width overflow");
        let height = self.height().try_into().expect("height overflow");
        let stride = width * 4;

        self.layer.wl_surface().set_buffer_scale(self.scale.into());

        if self
            .buffer
            .as_ref()
            .filter(|b| b.height() != height || b.stride() != stride)
            .is_some()
        {
            // we can't use this buffer any more.
            self.buffer.take();
        }

        const FORMAT: protocol::wl_shm::Format = protocol::wl_shm::Format::Argb8888;
        let mut buffer = self.buffer.take().unwrap_or_else(|| {
            self.pool
                .create_buffer(width, height, stride, FORMAT)
                .expect("create buffer")
                .0
        });

        let canvas = match self.pool.canvas(&buffer) {
            Some(canvas) => canvas,
            None => {
                // This should be rare, but if the compositor has not released the previous
                // buffer, we need double-buffering.
                let (second_buffer, canvas) = self
                    .pool
                    .create_buffer(width, height, stride, FORMAT)
                    .expect("create buffer");
                buffer = second_buffer;

                // create_buffer sometimes returns a larger buffer than we requested.
                // raqote DrawTarget::from_backing doesn't like this, so we'll truncate our
                // buffer to the desired length.
                //
                // https://github.com/Smithay/client-toolkit/issues/488
                let corrected_len = (width * height * 4) as usize;
                &mut canvas[..corrected_len]
            }
        };

        // Draw to the window:
        let mut dt = {
            let canvas = bytemuck::cast_slice_mut(canvas);

            render::DrawTarget::from_backing(width, height, canvas)
        };

        if self.state.needs_redraw {
            // something fundamental changed (e.g. scale factor)
            // we'll redraw the full screen.
            dt.clear(raqote::SolidSource {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            });

            render::grid(
                &layout::Space {
                    width: self.width as f32,
                    height: self.height as f32,
                },
                &self.config,
                &self.drawable_items,
                &self.state.cursor_position,
                &mut dt,
                &mut self.font_system,
                &mut self.swash_cache,
                self.scale as f32,
            );

            // Damage the entire window
            self.layer.wl_surface().damage_buffer(0, 0, width, height);

            self.state.needs_redraw = false;
        } else if self.state.cursor_needs_rerender() {
            // the cursor moved
            // we only need to redraw the old & new cursor positions
            // TODO: are these clones necessary?
            let cursor_position = self.state.cursor_position.clone();
            let rendered_position = self.state.rendered_cursor_position.clone();
            let items_to_redraw = self
                .drawable_items
                .at_positions(&[cursor_position, rendered_position]);
            for item in items_to_redraw {
                let (item_pos, item_space) = render::draw_grid_item(
                    item,
                    &layout::Space {
                        width: self.width as f32,
                        height: self.height as f32,
                    },
                    &self.config,
                    &self.state.cursor_position,
                    &mut dt,
                    &mut self.font_system,
                    &mut self.swash_cache,
                    self.scale as f32,
                );

                // Damage just the area we drew
                self.layer.wl_surface().damage_buffer(
                    item_pos.x as i32,
                    item_pos.y as i32,
                    item_space.width as i32,
                    item_space.height as i32,
                );
            }
        }

        self.state.rendered_cursor_position = self.state.cursor_position.clone();

        // Tell the compositor that we're done.
        buffer
            .attach_to(self.layer.wl_surface())
            .expect("buffer attach");
        self.layer.commit();

        self.buffer = Some(buffer);
    }

    pub fn should_exit(&self) -> bool {
        self.state.should_exit
    }
}

delegate_compositor!(Window);
delegate_output!(Window);
delegate_shm!(Window);

delegate_seat!(Window);
delegate_keyboard!(Window);

delegate_layer!(Window);

delegate_registry!(Window);

impl ProvidesRegistryState for Window {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState, SeatState];
}
