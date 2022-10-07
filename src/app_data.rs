// use crate::panel::user_selection::U;
//
// use crate::json_conv;
use crate::panel::user_selection::UserSelection;

// #[derive(Debug)]
pub struct AppData {
    state: AppState,
}

enum AppState {
    UserSelection(UserSelection),
    UserCreation,
    Login,
    Main,
}

impl AppData {
    pub fn switch_to_user_selection(&mut self) {
        println!("user selection");
        // json_conv::export_json("test");
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

impl Default for AppData {
    fn default() -> Self {
        let user_list: UserSelection = UserSelection::new();

        Self {
            state: AppState::UserSelection(user_list),
            // state: AppState::UserSelection(user_selection::load_user_list()),
        }
    }
}

impl eframe::App for AppData {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        // match self.state {
        //     AppState::UserSelection => panel::user_selection::display_user_selection(ctx, self),
        //     AppState::UserCreation => panel::user_creation::display_user_creation(ctx, self),
        //     AppState::Login => todo!(),
        //     AppState::Main => todo!(),
        // }
    }
}
