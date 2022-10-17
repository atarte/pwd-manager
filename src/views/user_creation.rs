
use eframe::egui;
// use serde::{Deserialize, Serialize};

use crate::app_data::Data;
use crate::models::user::User;

// use super::Init;

#[derive(Debug)]
pub struct UserCreation {
    pub name: String,
    pub pwd: String
}

impl UserCreation {
    pub fn new() -> Self {
        Self { 
            name: String::new(),
            pwd: String::new(),
        }
    }

    pub fn display(&mut self, data: &mut Data, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("creating user");

            if ui.button("<- Back").clicked() {
                data.switch_to_user_selection();
                return;
            }

            // let new_user: &mut User = app.get_new_user().unwrap();

            ui.horizontal(|ui| {
                ui.label("username: ");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.horizontal(|ui| {
                ui.label("password: ");
                ui.text_edit_singleline(&mut self.pwd);
                
                // new_user.hash_password(pwd); 
            });

            if ui.button("Save").clicked() {
                data.add_user(self);

                data.switch_to_user_selection();
                return;
            }
        });
    }
}
