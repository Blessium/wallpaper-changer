mod ui;
mod wallpaper;

use ui::MyEguiApp;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Wallpaper changer",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}
