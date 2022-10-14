// use crate::panels:user_selection::{UserSelection, USER_LIST_PATH};
use crate::views::user_selection;
use crate::views::user_creation;
use crate::models::{Init, export_json, import_json};
use crate::models::user::{User, UsersList};


const USERS_LIST_PATH: &str = "./users_list.json";

#[derive(Debug)]
pub struct AppData {
    state: AppState,
    users_list: UsersList,
    new_user: User,
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
        let users_selected: UsersList = import_json(USERS_LIST_PATH);
        let user_created: User = User::new();

        Self {
            state: AppState::UserSelection,
            users_list: users_selected,
            new_user: user_created,
        }
    }
}

impl AppData {
    pub fn get_users(&self) -> Result<&UsersList, String> {
        match self.state {
            AppState::UserSelection => Ok(&self.users_list),
            _ => Err(format!("State 'UserSelection' expect, buts state '{:?}' encounter", self.state)),
        }
    }

    pub fn get_new_user(&mut self) -> Result<&mut User, String> {
        match self.state {
            AppState::UserCreation => Ok(&mut self.new_user),
            _ => Err(format!("State 'UserCreation' expect, buts state '{:?}' encounter", self.state)),
        }
    }

    pub fn add_user(&mut self) {
        self.users_list.users.push(self.new_user.clone());
        export_json(USERS_LIST_PATH, &self.users_list);
        
        self.new_user = User::new();
    }

    pub fn switch_to_user_selection(&mut self) {
        self.state = AppState::UserSelection;
        self.users_list = import_json(USERS_LIST_PATH);
    }

    pub fn switch_to_user_creation(&mut self) {
        self.state = AppState::UserCreation;
        self.new_user = User::new();
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
