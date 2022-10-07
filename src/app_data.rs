use crate::panel::user_selection::{UserSelection, USER_LIST_PATH};
use crate::panel::{export_json, import_json};

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

impl Default for AppData {
    fn default() -> Self {
        let user_list: UserSelection = import_json(USER_LIST_PATH);
        Self {
            state: AppState::UserSelection(user_list),
        }
    }
}

impl AppData {
    pub fn switch_panel<E>(&mut self, new_state: &E) {
        match &self.state {
            AppState::UserSelection(data) => {
                export_json(USER_LIST_PATH, &data)
            },
            _ => todo!(),
        }

        // self.state = E;
    }

    pub fn switch_to_user_selection(&mut self) {
        println!("user selection");
        // export_json(USER_LIST_PATH, );
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
