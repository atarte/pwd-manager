// use crate::panels:user_selection::{UserSelection, USER_LIST_PATH};
use crate::views::users_selection::UsersSelection;
use crate::views::user_creation::UserCreation;
use crate::models::{Init, export_json, import_json};
use crate::models::user::{User, UsersList};


const USERS_LIST_PATH: &str = "./data/users_list.json";

#[derive(Debug)]
pub struct AppData {
    views: ViewsData,
    // state: appstate,
    data: Data,
    // users_list: UsersList,
    // new_user: User,
}

#[derive(Debug)]
enum AppState {
    UserSelection,
    UserCreation,
    Login,
    Home,
}

#[derive(Debug)]
pub struct Data {
    users_list: UsersList,
    user: User,
    state: AppState,
}

#[derive(Debug)]
struct ViewsData {
    users_selection: UsersSelection,
    user_creation: UserCreation,
}

impl Default for AppData {
    fn default() -> Self {
        let users_selected: UsersList = import_json(USERS_LIST_PATH);
        let user_created: User = User::new();

        Self {
            views: ViewsData {
                users_selection: UsersSelection::new(),
                user_creation: UserCreation::new(),
            },
            // state: AppState::UserSelection,
            data: Data {
                state: AppState::UserSelection,
                users_list: users_selected,
                user: user_created,
            }
            // users_list: users_selected,
            // new_user: user_created,
        }
    }
}

impl Data {
    pub fn get_users(&self) -> Result<&UsersList, String> {
        match self.state {
            AppState::UserSelection => Ok(&self.users_list),
            _ => Err(format!("State 'UserSelection' expect, buts state '{:?}' encounter", self.state)),
        }
    }

    pub fn get_user(&mut self) -> Result<&mut User, String> {
        match self.state {
            AppState::UserCreation => Ok(&mut self.user),
            _ => Err(format!("State 'UserCreation' expect, buts state '{:?}' encounter", self.state)),
        }
    }

    pub fn add_user(&mut self, new_user: &UserCreation) {
        // self.users_list.users.push(self.user.clone());
        self.user.name = new_user.name.clone();
        self.user.hash_password(new_user.pwd.clone());

        self.users_list.users.push(self.user.clone()); 
        export_json(USERS_LIST_PATH, &self.users_list);
        
        self.user = User::new();
    }

    pub fn switch_to_user_selection(&mut self) {
        self.state = AppState::UserSelection;
        self.users_list = import_json(USERS_LIST_PATH);
    }

    pub fn switch_to_user_creation(&mut self) {
        self.state = AppState::UserCreation;
        self.user = User::new();
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

        match self.data.state {
            AppState::UserSelection => self.views.users_selection.display(&mut self.data, ctx, frame),
            AppState::UserCreation => self.views.user_creation.display(&mut self.data, ctx, frame),
            AppState::Login => todo!(),
            AppState::Home => todo!(),
        }
    }
}
