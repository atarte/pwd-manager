
use eframe::egui;
// use serde::{Deserialize, Serialize};

use crate::app_data::AppData;
use super::Init;

// #[derive(Debug, Serialize, Deserialize)]
#[derive(Debug)]
pub struct UserCreation {
    name: String,
    pwd: String
}

impl Init for UserCreation {
    fn init() -> Self {
        Self {
            name: String::new(),
            pwd: String::new(),
        }
    }
}

pub fn display(app: &mut AppData, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("creating user");

        if ui.button("<- Back").clicked() {
            app.switch_to_user_selection();
            return;
        }

        let new_user: &mut UserCreation = app.get_new_user().unwrap();

        ui.horizontal(|ui| {
            ui.label("username: ");
            ui.text_edit_singleline(&mut new_user.name);
        });

        ui.horizontal(|ui| {
            ui.label("password: ");
            ui.text_edit_singleline(&mut new_user.pwd);
        });
    });
}
