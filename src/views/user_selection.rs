use eframe::egui;
// use serde::{Deserialize, Serialize};

use crate::app_data::AppData;
use crate::models::user::{User, UsersList};
// use super::Init;

// pub const USER_LIST_PATH: &str = "./user_list.json";

// #[derive(Debug, Serialize, Deserialize)]
// pub struct UserSelection {
//     users_list: Vec<User>,
// }
//
// impl Init for UserSelection {
//     fn init() -> Self {
//         Self {
//             users_list: vec!(),
//         }
//     }
// }
//
// impl UserSelection {
//     // pub fn new() -> Self {
//     //     Self {
//     //         user_list: vec!(),
//     //     }
//     // }
// }

pub fn display(app: &mut AppData, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("selecting user");

        for user in app.get_users().unwrap().users.iter() {
            if ui.button(&user.name).clicked() {

            }
        }

         
        if ui.button("create_user").clicked() {
            app.switch_to_user_creation();
            return;
        }
    });
}

