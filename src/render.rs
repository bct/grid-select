use crate::{colour, config, grid, layout, text};
use cosmic_text;
use raqote::{DrawOptions, PathBuilder, Source};

pub type DrawTarget<'a> = raqote::DrawTarget<&'a mut [u32]>;

pub struct DrawArgs<'a> {
    pub layer_space: &'a layout::Space,
    pub config: &'a config::Config,
    pub font_system: &'a mut cosmic_text::FontSystem,
    pub swash_cache: &'a mut cosmic_text::SwashCache,
    pub scale: f32,
}

pub struct DrawableItem {
    text: text::Text,
    pub grid_position: grid::GridPosition,
    normal_bg_colour: colour::Colour,
}

impl DrawableItem {
    pub fn new(
        grid_position: grid::GridPosition,
        text: String,
        font_name: Option<String>,
        size: f32,
        normal_bg_colour: colour::Colour,
    ) -> DrawableItem {
        let t = text::Text::new(text, font_name, size);
        DrawableItem {
            text: t,
            grid_position,
            normal_bg_colour,
        }
    }
}

pub struct DrawableItems {
    items: Vec<DrawableItem>,
}

impl DrawableItems {
    pub fn from_grid(config: &config::Config, grid: &grid::Grid) -> DrawableItems {
        let mut cycle_colours = config.bg_colour.cycle();

        let dis = grid
            .items_iter()
            .map(|i| {
                let normal_bg_colour = cycle_colours.next().unwrap();
                DrawableItem::new(
                    i.position.clone(),
                    i.display.clone(),
                    config.font_name.clone(),
                    config.font_size,
                    normal_bg_colour.clone(),
                )
            })
            .collect();

        DrawableItems { items: dis }
    }

    pub fn at_positions(&self, positions: &[grid::GridPosition]) -> Vec<&DrawableItem> {
        // TODO: scanning every DI here is not efficient.
        // this is called every time the cursor moves.
        self.items
            .iter()
            .filter(|di| positions.contains(&di.grid_position))
            .collect()
    }
}

fn grid_item_rect(
    config: &config::Config,
    item_pos: &layout::ScreenPosition,
    item_space: &layout::Space,
    is_selected: bool,
    normal_bg_colour: &colour::Colour,
    dt: &mut DrawTarget,
) {
    let mut pb = PathBuilder::new();
    pb.rect(item_pos.x, item_pos.y, item_space.width, item_space.height);
    let path = pb.finish();

    let bg_colour = if is_selected {
        &config.active_bg_colour
    } else {
        normal_bg_colour
    };
    dt.fill(
        &path,
        &Source::Solid(bg_colour.as_source()),
        &DrawOptions::default(),
    );

    dt.stroke(
        &path,
        &Source::Solid(config.border_colour.as_source()),
        &raqote::StrokeStyle {
            width: config.border_width,
            ..Default::default()
        },
        &DrawOptions::default(),
    );
}

pub fn draw_grid_item(
    dt: &mut DrawTarget,
    di: &DrawableItem,
    cursor_position: &grid::GridPosition,
    draw_args: &mut DrawArgs,
) -> (layout::ScreenPosition, layout::Space) {
    let (item_pos, item_space) = layout::grid_position_to_screen(
        &draw_args.layer_space.scale(draw_args.scale),
        &di.grid_position,
        draw_args.config.item_width * draw_args.scale,
        draw_args.config.item_height * draw_args.scale,
        draw_args.config.item_margin * draw_args.scale,
    );

    let is_selected = cursor_position == &di.grid_position;

    // render the rectangle
    grid_item_rect(
        draw_args.config,
        &item_pos,
        &item_space,
        is_selected,
        &di.normal_bg_colour,
        dt,
    );

    // render the text
    let fg_colour = if is_selected {
        &draw_args.config.active_fg_colour
    } else {
        &draw_args.config.fg_colour
    };

    di.text.render_centred(
        dt,
        draw_args.font_system,
        draw_args.swash_cache,
        draw_args.scale,
        fg_colour,
        &item_space,
        &item_pos,
    );

    (item_pos, item_space)
}

pub fn grid(
    dt: &mut DrawTarget,
    drawable_items: &DrawableItems,
    cursor_position: &grid::GridPosition,
    draw_args: &mut DrawArgs,
) {
    for di in &drawable_items.items {
        draw_grid_item(dt, di, cursor_position, draw_args);
    }
}
