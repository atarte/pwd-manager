#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod views;
mod models;
mod app_data;

fn main() {
    let options = eframe::NativeOptions{
        ..Default::default()
    };

    eframe::run_native(
        "Pwd Manager", 
        options, 
        Box::new(|_cc| Box::new(app_data::AppData::default())),
    );
}
