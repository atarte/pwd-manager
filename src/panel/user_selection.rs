use eframe::egui;
use serde::{Deserialize, Serialize};

use std::fs::File; 
use std::path::Path;
use std::io::Write;

use crate::App;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSelection {
    user_list: Vec<String>,
}

pub fn display_user_selection(ctx: &eframe::egui::Context, app: &mut App) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("selecting user");
       
        if ui.button("create_user").clicked() {
            // *state = AppState::UserCreation;
            app.switch_to_user_creation();
        }
        // if ui.bu
       
    });
}


pub fn load_user_list() -> UserSelection {
    let user_file_path = "./user_list.json";

    // Check if file exists
    if !Path::new(user_file_path).exists() {
        File::create(user_file_path)
            .expect("Can not create user list file")
            // .write(serde_json::to_string_pretty(UserSelection::new()).)
            .write(b"{\"user_list\": []}")
            .expect("Can not fill user list file");
    }

    let test = serde_json::to_string_pretty(UserSelection::new());
    // Get file content and convert it to struct
    let file_json = serde_json::from_reader::<File, UserSelection>(
        File::open(user_file_path)
            .expect("File should exist so fuck it")
    ).expect("Cannot read json");

    println!("{:?}", file_json);
    return file_json;
}