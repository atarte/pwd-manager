use std::{path::Path, fs::File};

// use eframe::egui;
use serde::{Deserialize, Serialize};

use super::Init;
//
// use std::fs::File; 
// use std::path::Path;
// use std::io::Write;
//
// use crate::App;
//
// use super::Json;

pub const USER_LIST_PATH: &str = "./user_list.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSelection {
    user_list: Vec<String>,
}

impl Init for UserSelection {
    fn init() -> Self {
        Self {
            user_list: vec!(),
        }
    }
}
// impl Json for UserSelection {
//     fn export_json(self: &UserSelection) {
//         let file: File = File::create(FILE_PATH)
//             .expect("cant create file");
//
//         serde_json::to_writer_pretty(&file, self)
//             .expect("cant write date in file")
//     }
//     
//     fn import_json() -> Self {
//        if !Path::new(FILE_PATH).exists() {
//            
//        } 
//     }
// }

impl UserSelection {
    // pub fn new() -> Self {
    //     Self {
    //         user_list: vec!(),
    //     }
    // }
}
//
// pub fn display_user_selection(ctx: &eframe::egui::Context, app: &mut App) {
//     egui::CentralPanel::default().show(ctx, |ui| {
//         ui.heading("selecting user");
//        
//         if ui.button("create_user").clicked() {
//             // *state = AppState::UserCreation;
//             app.switch_to_user_creation();
//         }
//         // if ui.bu
//        
//     });
// }
//
//
// pub fn load_user_list() -> UserSelection {
//     let user_file_path = "./user_list.json";
//
//     // Check if file exists
//     if !Path::new(user_file_path).exists() {
//         File::create(user_file_path)
//             .expect("Can not create user list file")
//             // .write(serde_json::to_string_pretty(UserSelection::new()).)
//             .write(b"{\"user_list\": []}")
//             .expect("Can not fill user list file");
//     }
//
//     let test = serde_json::to_string_pretty(UserSelection::new());
//     // Get file content and convert it to struct
//     let file_json = serde_json::from_reader::<File, UserSelection>(
//         File::open(user_file_path)
//             .expect("File should exist so fuck it")
//     ).expect("Cannot read json");
//
//     println!("{:?}", file_json);
//     return file_json;
// }
