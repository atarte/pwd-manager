use eframe::egui;

use  crate::App;

pub fn display_user_creation(ctx: &eframe::egui::Context, app: &mut App) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("user creation");

        if ui.button("back").clicked() {
            app.switch_to_user_selection();
        }
    });
}