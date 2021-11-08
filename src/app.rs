use eframe::{egui, epi};
use egui::*;
use rugui::framebuffer::{Framebuffer, Color, Orientation};
use rugui::geometry::{Coordinates};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    framebuffer: Framebuffer,
    scroll: u32,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: u32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2,
            scroll: 50,
            framebuffer: Framebuffer::new(160, 32, 8).unwrap()
        }
    }
}

impl epi::App for TemplateApp {
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
        //self.framebuffer.draw_line(&Coordinates::new((5, 5), (40, 5)), &Color::Black);
        //self.framebuffer.draw_line(&Coordinates::new((5, 10), (5, 30)), &Color::Black);
        //self.framebuffer.draw_rect(&Coordinates::new((5, 5), (100, 30)), &Color::Black);
        //self.framebuffer.draw_circle(100, 16, 8, &Color::Black);
        //self.framebuffer.progress_bar(5, 45, 5, 15, 50, &Color::Black);
        //self.framebuffer.table(&Coordinates::new((50, 0), (150, 30)), 3, 3, &Color::Black);

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
        let Self { label, value, scroll, framebuffer } = self;

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

            ui.add(egui::Slider::new(value, 0..=100).text("progress"));
            framebuffer.progress_bar(&Coordinates::new((15, 5),(90, 15)), *value, &Color::Black);

            ui.add(egui::Slider::new(scroll, 0..=100).text("scroll"));
            framebuffer.scroller(&Coordinates::new((0, 0), (10, 32)), *scroll as i32, 3, Orientation::Vertical, &Color::Black);


            if ui.button("Increment").clicked() {
                framebuffer.draw_pixel(*value as i32, *value as i32, &Color::Black);
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
            
            draw_display(ui.painter(), Vec2{x: 300.0, y: 100.0}, &framebuffer);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}


fn draw_display(painter: &Painter, start: Vec2, frame: &Framebuffer) {
    let scaling = 4.0;

    for x in 0..frame.get_width() {
        for y in 0..frame.get_height() {
            let pixel = frame.get_pixel(x, y);
            let color = match pixel {
                Color::Black => Color32::from_gray(255),
                _ => Color32::from_gray(0)
            };
            let net = Color32::from_gray(200);

            let px = start.x + (scaling * x as f32);
            let py = start.y + (scaling * y as f32);

            painter.rect(
                Rect::from_min_max(
                    Pos2{x: px, y: py}, 
                    Pos2{x: px + scaling - 1.0, y: py + scaling - 1.0}),
                1.0,
                color,
                Stroke::none(),
            );

            painter.rect(
                Rect::from_min_max(
                    Pos2{x: px + scaling - 1.0, y: py}, 
                    Pos2{x: px + scaling, y: py + scaling}),
                1.0,
                net,
                Stroke::none(),
            );

            painter.rect(
                Rect::from_min_max(
                    Pos2{x: px, y: py + scaling - 1.0}, 
                    Pos2{x: px + scaling, y: py + scaling}),
                1.0,
                net,
                Stroke::none(),
            );
        }
    }
}