mod app;
mod panel;
mod filemanager;
mod launcher;
mod taskbar;
mod dock;
mod wallpaper;
mod settings;
mod theme;
mod widgets;
mod notifications;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("NganjoGUI")
            .with_fullscreen(true)
            .with_decorations(false),
        ..Default::default()
    };

    eframe::run_native(
        "NganjoGUI",
        native_options,
        Box::new(|cc| Box::new(app::NganjoGUI::new(cc))),
    )
}
