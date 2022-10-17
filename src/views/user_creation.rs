
use eframe::egui;
// use serde::{Deserialize, Serialize};

use crate::app_data::Data;
use crate::models::user::User;

// use super::Init;

#[derive(Debug)]
pub struct UserCreation {
    name: String,
    pwd: String
}

impl UserCreation {
    pub fn new() -> Self {
        Self { 
            name: String::new(),
            pwd: String::new(),
        }
    }

    pub fn display(&mut self, app: &mut Data, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("creating user");

            if ui.button("<- Back").clicked() {
                app.switch_to_user_selection();
                return;
            }

            let new_user: &mut User = app.get_new_user().unwrap();

            ui.horizontal(|ui| {
                ui.label("username: ");
                ui.text_edit_singleline(&mut new_user.name);
            });

            ui.horizontal(|ui| {
                let mut pwd_string: String = String::new();
                ui.label("password: ");
                ui.text_edit_singleline(&mut pwd_string);
                
                new_user.hash_password(pwd_string); 
            });

            if ui.button("Save").clicked() {
                app.add_user();

                app.switch_to_user_selection();
                return;
            }
        });
    }
}
