use copypasta::{ClipboardContext, ClipboardProvider};
use eframe::{egui, epi};
use egui::*;
use rugui::coordinates::cvec::{Vector2};
use rugui::framebuffer::{Color, Framebuffer};

/// A widget to render a screen emulator
///
/// ```
/// ui.add(EDisplay::new(&framebuffer, 2, frame);
/// ```
#[derive(Clone, Copy)]
pub struct EDisplay {
    image: Image,
    sense: Sense,
    scaling: usize,
    texture_id: Option<TextureId>,
}

impl EDisplay {
    pub fn new(framebuffer: &Framebuffer<'_>, scaling: usize, frame: &mut epi::Frame<'_>) -> Self {
        let req =
            (framebuffer.get_width(), framebuffer.get_height()).scale(scaling as i32);
        let size = (req.0 as usize, req.1 as usize);

        let pixels = framebuffer_to_pixels(framebuffer, scaling);
        let texture = frame
            .tex_allocator()
            .alloc_srgba_premultiplied(size, &pixels);
        let image = Image::new(texture, emath::vec2(size.0 as f32,size.1 as f32));

        Self {
            image,
            sense: Sense::click(),
            scaling,
            texture_id: Some(texture),
        }
    }

    /// Create a dummy widget
    pub fn default() -> Self {
        Self {
            image: Image::new(TextureId::default(), [0.0, 0.0]),
            sense: Sense::click(),
            scaling: 1,
            texture_id: None,
        }
    }

    /// Free texture memory used by widget. Since egui can't change a texture after creation,
    /// we create new one on each redraw.
    ///
    /// ```
    /// display.free_texture(frame);
    /// *display = EDisplay::new(&framebuffer, 2, frame);
    /// ui.add(display.clone());
    ///
    pub fn free_texture(&mut self, frame: &mut epi::Frame<'_>) {
        if let Some(texture) = self.texture_id {
            frame.tex_allocator().free(texture);
            self.texture_id = None;
        }
    }
}

impl Widget for EDisplay {
    fn ui(self, ui: &mut Ui) -> Response {
        if self.texture_id.is_none() {
            // Do not draw default() widget
            return ui.allocate_response(self.image.size(), self.sense);
        }

        let (rect, response) = ui.allocate_exact_size(self.image.size(), self.sense);
        self.image.paint_at(ui, rect);

        let clicked = response.secondary_clicked();
        response
            .on_hover_ui_at_pointer(|ui| {
                let cursor = ui.input().pointer.hover_pos().unwrap();
                let x = (cursor.x - rect.left()) as usize / self.scaling;
                let y = (cursor.y - rect.top()) as usize / self.scaling;
                let text = format!("x = {}\ny = {}", x, y);
                ui.label(text);

                if clicked {
                    let mut clipboard = ClipboardContext::new().unwrap();
                    let text = format!("({}, {})", x, y);
                    clipboard.set_contents(text).unwrap();
                }
            })
            .on_hover_cursor(CursorIcon::Crosshair)
    }
}

/// Convert `rugui::Framebuffer` to pixel array, which can be used by `egui`.
fn framebuffer_to_pixels(framebuffer: &Framebuffer<'_>, scaling: usize) -> Vec<Color32> {
    let width  = framebuffer.get_width()  as usize * scaling;
    let height = framebuffer.get_height() as usize * scaling;

    let mut pixels = vec![Color32::from_gray(200); width * height];

    for y in 0..framebuffer.get_height() {
        for x in 0..framebuffer.get_width() {
            let pixel = framebuffer.get_pixel(x, y);
            let color = match pixel {
                Color::Black => Color32::from_gray(255),
                _            => Color32::from_gray(0),
            };
            let pos = (y as usize * scaling * width) + (x as usize * scaling);

            // TODO: Doesn't work for scaling == 1
            for dx in 0..(scaling - 1) {
                for dy in 0..(scaling - 1) {
                    pixels[pos + dx + (dy * width)] = color;
                }
            }
        }
    }

    pixels
}
