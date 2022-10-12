use crate::panel::Init;
// use crate::panel::user_selection::{UserSelection, USER_LIST_PATH};
use crate::panel::user_selection;
use crate::panel::user_creation;
use crate::panel::{export_json, import_json};

#[derive(Debug)]
pub struct AppData {
    state: AppState,
    users: user_selection::UserSelection,
    new_user: user_creation::UserCreation,
}

#[derive(Debug)]
enum AppState {
    UserSelection,
    UserCreation,
    Login,
    Main,
}

impl Default for AppData {
    fn default() -> Self {
        let users_selected: user_selection::UserSelection = import_json(user_selection::USER_LIST_PATH);
        let user_created: user_creation::UserCreation = user_creation::UserCreation::init();

        Self {
            state: AppState::UserSelection,
            users: users_selected,
            new_user: user_created,
        }
    }
}

impl AppData {
    pub fn get_users(&self) -> Result<&user_selection::UserSelection, String> {
        match self.state {
            AppState::UserSelection => Ok(&self.users),
            _ => Err(format!("State 'UserSelection' expect, buts state '{:?}' encounter", self.state).to_string()),
        }
    }

    pub fn get_new_user(&mut self) -> Result<&mut user_creation::UserCreation, String> {
        match self.state {
            AppState::UserCreation => Ok(&mut self.new_user),
            _ => Err(format!("State 'UserCreation' expect, buts state '{:?}' encounter", self.state).to_string()),
        }
    }

    pub fn switch_to_user_selection(&mut self) {
        // println!("user selection");
        self.state = AppState::UserSelection;
    }

    pub fn switch_to_user_creation(&mut self) {
        // println!("user creation");
        self.state = AppState::UserCreation
    }

    pub fn switch_to_login(&mut self) {
        // println!("login");
        self.state = AppState::Login
    }

    pub fn switch_to_main(&mut self) {
        // println!("main");
        self.state = AppState::Main
    }
}

impl eframe::App for AppData {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        println!("{:?}", self.state);

        match self.state {
            AppState::UserSelection => user_selection::display(self, ctx, frame),
            AppState::UserCreation => user_creation::display(self, ctx, frame),
            AppState::Login => todo!(),
            AppState::Main => todo!(),
        }
    }
}
