// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// use user_creation::display_user_creation;

mod panel;

enum AppState {
    UserSelection(panel::user_selection::UserSelection),
    UserCreation,
    Login,
    Main,
}

fn main() {
    let options = eframe::NativeOptions::default();
    
    eframe::run_native(
        "Login", 
        options, 
        Box::new(|_cc| Box::new(App::default())),
    );
}

pub struct App {
    state: AppState,
}

impl App {
    pub fn switch_to_user_selection(&mut self) {
        println!("user selection");
        // self.state = AppState::UserSelection(_)
    }

    pub fn switch_to_user_creation(&mut self) {
        println!("user creation");
        self.state = AppState::UserCreation
    }

    pub fn switch_to_login(&mut self) {
        println!("login");
        self.state = AppState::Login
    }

    pub fn switch_to_main(&mut self) {
        println!("main");
        self.state = AppState::Main
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: AppState::UserSelection(panel::user_selection::load_user_list()),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        // match self.state {
        //     AppState::UserSelection => panel::user_selection::display_user_selection(ctx, self),
        //     AppState::UserCreation => panel::user_creation::display_user_creation(ctx, self),
        //     AppState::Login => todo!(),
        //     AppState::Main => todo!(),
        // }
    }
}
