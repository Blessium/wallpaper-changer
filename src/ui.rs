use eframe::egui;
use eframe::egui::{FontId, Color32, ComboBox, ScrollArea, TextEdit};

#[derive(Default)]
pub struct MyEguiApp {
    wallpaper_search_text: String,
    wallpaper_mode: Mode,
}

#[derive(Debug, PartialEq, Default)]
enum Mode {
    #[default]
    Center,
    Crop,
    Fit,
    Span,
    Stretch,
    Tile,
}

impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Wallpaper switcher");
            
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {

                        ui.add(TextEdit::singleline(&mut self.wallpaper_search_text).hint_text("Search wallpaper"));

                        if ui.button("+").clicked() {
                            if let Some(path) = rfd::FileDialog::new().set_title("Select file or directory").pick_file() {
                                println!("{}", path.display().to_string());
                            }
                        }
                    });
                    ScrollArea::vertical().show(ui, |ui| { ui.label("works"); });
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Positioning: ");
                    ComboBox::from_label("")
                        .selected_text(format!("{:?}", self.wallpaper_mode))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.wallpaper_mode, Mode::Center, "Center");
                            ui.selectable_value(&mut self.wallpaper_mode, Mode::Crop, "Crop");
                            ui.selectable_value(&mut self.wallpaper_mode, Mode::Fit, "Fit");
                            ui.selectable_value(&mut self.wallpaper_mode, Mode::Span, "Span");
                            ui.selectable_value(&mut self.wallpaper_mode, Mode::Stretch, "Stretch");
                            ui.selectable_value(&mut self.wallpaper_mode, Mode::Tile, "Tile");
                        });
                });
            });
        });
    }
}
