use eframe::{egui, epi};
use rugui::framebuffer::{Framebuffer, Color};
use rugui::coordinates::bounding_box::BBox;

use super::edisplay::{EDisplay};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct DisplayEmulator {
    // Example stuff:
    label: String,
    framebuffer: Framebuffer,
    radius: u32,
    circle_thickness: u32,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: u32,
    #[cfg_attr(feature = "persistence", serde(skip))]
    display: EDisplay
}

impl Default for DisplayEmulator {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 0,
            radius: 0,
            circle_thickness: 1,

            display: EDisplay::default(),
            framebuffer: Framebuffer::new(160, 32, 8).unwrap()
        }
    }
}

impl epi::App for DisplayEmulator {
    fn name(&self) -> &str {
        "eframe template"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        //self.framebuffer.draw_line(&BBox::new((5, 5).into(), (40, 5).into()), &Color::Black);
        //self.framebuffer.draw_line(&BBox::new((5, 10).into(), (5, 30).into()), &Color::Black);
        //self.framebuffer.draw_line(&BBox::new((5, 5).into(), (80, 30).into()), &Color::Black);
        //self.framebuffer.table(&BBox::new((50, 0).into(), (150, 30).into()), 3, 3, &Color::Black);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { label, value, radius, framebuffer, circle_thickness, display } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            rugui::geometry::Rect::new_filled(BBox::new((90, 0).into(), (159, 31).into()), Color::White)
                .draw(framebuffer);
            ui.add(egui::Slider::new(value, 1..=100).text("progress"));
            framebuffer.progress_bar(BBox::new((15, 5).into(), (90, 15).into()), *value, Color::Black);

            ui.add(egui::Slider::new(radius, 0..=16).text("radius"));
            ui.add(egui::Slider::new(circle_thickness, 1..=*radius).text("thickness"));
            rugui::geometry::Circle::new((120, 16).into(), *radius, Color::Black)
                .thickness(*circle_thickness)
                .draw(framebuffer);


            if ui.button("Increment").clicked() {
                use rugui::geometry::Line;
                let mut cords = (*value as i32, *value as i32);
                Line::new(BBox::new(cords.into(), cords.into()), Color::Black)
                    .draw(framebuffer);

                cords.1 += 2;
                Line::new(BBox::new(cords.into(), cords.into()), Color::Black)
                    .draw(framebuffer);

                //framebuffer.draw_pixel(*value as i32, *value as i32, &Color::Black);
                *value += 1;
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
            ui.heading("LCD emulation");

            display.free_texture(frame);
            *display = EDisplay::new(&framebuffer, 4, frame);
            ui.add(display.clone());

            ui.label("Right-click on a display to copy coordinates to clip buffer");
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and radiusing if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
