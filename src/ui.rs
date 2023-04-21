use crate::wallpaper::WallpaperManager;
use eframe::egui;
use eframe::egui::{menu, ComboBox, Grid, ScrollArea, TextEdit};

#[derive(Default)]
pub struct MyEguiApp {
    wallpaper_search_text: String,
    wallpaper_manager: WallpaperManager,
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
        egui::TopBottomPanel::top("Wallpaper changer").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Import wallpaper").clicked() {
                        if let Some(wallpapers_path) = rfd::FileDialog::new().pick_files() {
                            self.wallpaper_manager
                                .import_wallpapers(wallpapers_path)
                                .unwrap();
                        }
                    }
                    if ui.button("Import folder").clicked() {
                        if let Some(folders_path) = rfd::FileDialog::new().pick_folders() {
                            println!("{:?}", folders_path);
                        }
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Wallpaper switcher");
            ui.vertical(|ui| {
                ui.add(
                    TextEdit::singleline(&mut self.wallpaper_search_text)
                        .hint_text("Search wallpaper"),
                );
                ScrollArea::vertical()
                    .min_scrolled_height(192.0)
                    .show(ui, |ui| {
                        Grid::new("Wallpapers:").show(ui, |ui| {
                            if let Some(wallpapers) = self.wallpaper_manager.load_wallpapers() {
                                let _idx = 0;
                                let _: _ = wallpapers
                                    .into_iter()
                                    .enumerate()
                                    .map(|(idx, mut wallpaper)| {
                                        if ((idx + 1) % 2) == 0 {
                                            wallpaper.display(ui);
                                            ui.end_row();
                                        } else {
                                            wallpaper.display(ui);
                                        }
                                    })
                                    .collect::<Vec<_>>();
                            } else {
                                ui.label("No wallpaper found");
                            }
                        });
                    });
            });
        });
    }
}
