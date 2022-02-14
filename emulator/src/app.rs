use super::edisplay::EDisplay;
use eframe::{egui, epi};
use rugui::coordinates::bounding_box::BBox;
use rugui::framebuffer::{Color, Framebuffer};
use rugui::geometry::{circle::Circle, ellipse::Ellipse, rect::Rect, Drawable};

pub struct DisplayEmulator {
    label: String,
    radius: u32,
    circle_thickness: u32,
    progress: u8,
    resolution: (i32, i32),
    ellipse_width: u32,
    ellipse_height: u32,
    ellipse_thickness: u32,
}

impl Default for DisplayEmulator {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            radius: 5,
            circle_thickness: 1,
            progress: 0,
            resolution: (160, 32),
            ellipse_width: 5,
            ellipse_height: 3,
            ellipse_thickness: 1,
        }
    }
}

impl<'a> epi::App for DisplayEmulator {
    fn name(&self) -> &str {
        "rugui display emulator"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        let Self {
            label,
            radius,
            circle_thickness,
            progress,
            resolution,
            ellipse_width,
            ellipse_height,
            ellipse_thickness,
        } = self;

        let size = resolution.0 * (resolution.1 as f32 / 8.0).ceil() as i32;
        let mut buffer = vec![0; size as usize];
        let mut framebuffer = Framebuffer::new(resolution.0, resolution.1, &mut buffer).unwrap();

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            Rect::new_filled(BBox::new((90, 0), (159, 31)), Color::White).draw(&mut framebuffer);
            ui.add(egui::Slider::new(progress, 1..=100).text("progress"));
            rugui::widgets::progress_bar::ProgressBar::new(BBox::new((15, 5), (90, 15)), *progress, Color::Black)
                .draw(&mut framebuffer);

            ui.add(egui::Slider::new(radius, 0..=16).text("radius"));
            ui.add(egui::Slider::new(circle_thickness, 1..=*radius).text("thickness"));
            Circle::new((120, 16), *radius, Color::Black)
                .thickness(*circle_thickness)
                .draw(&mut framebuffer);

            let max_thickness: u32 = if ellipse_height >= ellipse_width {
                *ellipse_width
            } else {
                *ellipse_height
            };

            ui.add(egui::Slider::new(ellipse_width, 1..=50).text("ellipse width"));
            ui.add(egui::Slider::new(ellipse_height, 1..=50).text("ellipse height"));
            ui.add(
                egui::Slider::new(ellipse_thickness, 1..=max_thickness).text("ellipse thockness"),
            );
            Ellipse::new(*ellipse_width, *ellipse_height, (25, 25), Color::Black)
                .thickness(*ellipse_thickness)
                .draw(&mut framebuffer);

            if ui.button("Increment").clicked() {
                use rugui::geometry::line::Line;
                let mut cords = (*progress as i32, *progress as i32);
                Line::new(BBox::new(cords, cords), Color::Black).draw(&mut framebuffer);

                cords.1 += 2;
                Line::new(BBox::new(cords, cords), Color::Black).draw(&mut framebuffer);
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("LCD emulation control");

            ui.add(egui::Slider::new(&mut resolution.0, 1..=640).text("width"));
            ui.add(egui::Slider::new(&mut resolution.1, 1..=480).text("height"));
            ui.add(EDisplay::new(&framebuffer, 4, ctx));

            ui.label("Right-click on a display to copy coordinates to clip buffer");
        });
    }
}
