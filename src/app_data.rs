// use crate::panels:user_selection::{UserSelection, USER_LIST_PATH};
use crate::views::user_selection;
use crate::views::user_creation;
use crate::views::{Init, export_json, import_json};

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
    Home,
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
            _ => Err(format!("State 'UserSelection' expect, buts state '{:?}' encounter", self.state)),
        }
    }

    pub fn get_new_user(&mut self) -> Result<&mut user_creation::UserCreation, String> {
        match self.state {
            AppState::UserCreation => Ok(&mut self.new_user),
            _ => Err(format!("State 'UserCreation' expect, buts state '{:?}' encounter", self.state)),
        }
    }

    pub fn switch_to_user_selection(&mut self) {
        self.state = AppState::UserSelection;
        self.users = import_json(user_selection::USER_LIST_PATH);
    }

    pub fn switch_to_user_creation(&mut self) {
        self.state = AppState::UserCreation;
        self.new_user = user_creation::UserCreation::init();
    }

    pub fn switch_to_login(&mut self) {
        self.state = AppState::Login
    }

    pub fn switch_to_home(&mut self) {
        self.state = AppState::Home
    }
}

impl eframe::App for AppData {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {

        match self.state {
            AppState::UserSelection => user_selection::display(self, ctx, frame),
            AppState::UserCreation => user_creation::display(self, ctx, frame),
            AppState::Login => todo!(),
            AppState::Home => todo!(),
        }
    }
}
