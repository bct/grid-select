use crate::{colour, layout, render};

use cosmic_text::{Align, Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use raqote::{DrawOptions, Source};

fn make_buffer(
    text: &str,
    font_system: &mut FontSystem,
    font_name: Option<&str>,
    size: f32,
    scale: f32,
    space: &layout::Space,
) -> Buffer {
    // Text metrics indicate the font size and line height of a buffer
    let metrics = Metrics::new(size, size).scale(scale);

    // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
    let mut buffer = Buffer::new(font_system, metrics);

    // Set a size for the text buffer, in pixels
    buffer.set_size(font_system, Some(space.width), Some(space.height));

    // Attributes indicate what font to choose
    let mut attrs = Attrs::new();
    if let Some(font_name) = font_name {
        attrs = attrs.family(cosmic_text::Family::Name(font_name));
    }

    // Add some text!
    buffer.set_text(
        font_system,
        text,
        &attrs,
        Shaping::Advanced,
        Some(Align::Center),
    );

    buffer
}

struct RenderedText {
    width: i32,
    height: i32,
    data: Vec<u32>,
}

impl RenderedText {
    fn as_image(&self) -> raqote::Image<'_> {
        raqote::Image {
            width: self.width,
            height: self.height,
            data: &self.data,
        }
    }
}

// render the text into a new image buffer.
fn render_text_centred(
    text: &str,
    font_name: Option<&str>,
    font_size: f32,
    font_system: &mut FontSystem,
    swash_cache: &mut SwashCache,
    scale: f32,
    colour: &colour::Colour,
    space: &layout::Space,
) -> RenderedText {
    let mut dt = raqote::DrawTarget::new(space.width as i32, space.height as i32);

    let buffer = make_buffer(text, font_system, font_name, font_size, scale, space);

    let run_count = buffer.layout_runs().count();
    let text_height = buffer.metrics().line_height * run_count as f32;

    // centre the text vertically.
    // (cosmic text already centres it horizontally)
    let x_offset = 0.;
    let y_offset = (space.height - text_height) / 2.;

    buffer.draw(
        font_system,
        swash_cache,
        colour.as_cosmic(),
        |x, y, w, h, colour| {
            let (r, g, b, a) = colour.as_rgba_tuple();
            let source = Source::Solid(raqote::SolidSource::from_unpremultiplied_argb(a, r, g, b));
            dt.fill_rect(
                x_offset + x as f32,
                y_offset + y as f32,
                w as f32,
                h as f32,
                &source,
                &DrawOptions::new(),
            );
        },
    );

    RenderedText {
        width: space.width as i32,
        height: space.height as i32,
        data: dt.into_vec(),
    }
}

pub struct Text {
    text: String,
    font_name: Option<String>,
    font_size: f32,
}

impl Text {
    pub fn new(text: String, font_name: Option<String>, size: f32) -> Text {
        Text {
            text,
            font_name,
            font_size: size,
        }
    }

    pub fn render_centred(
        &self,
        dt: &mut render::DrawTarget,
        font_system: &mut FontSystem,
        swash_cache: &mut SwashCache,
        scale: f32,
        colour: &colour::Colour,
        space: &layout::Space,
        position: &layout::ScreenPosition,
    ) {
        // empirically drawing to an image and then copying that image into the
        // final layer is significantly faster than drawing directly to the layer.
        //
        // (based on keyboard response time when redrawing 128 items)
        //
        // this could mabye make sense if the small image fits into cache better, but that's
        // total speculation.
        let rendered_text = render_text_centred(
            self.text.as_str(),
            self.font_name.as_deref(),
            self.font_size,
            font_system,
            swash_cache,
            scale,
            colour,
            space,
        );

        dt.draw_image_at(
            position.x,
            position.y,
            &rendered_text.as_image(),
            &DrawOptions::default(),
        );
    }
}
