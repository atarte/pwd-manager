// use eframe::egui;

// pub struct Login {
//     user: String,
//     pwd: String,
//     show_pwd: bool
// }

// fn password_ui()

// impl Login {
//     fn clear_field(&mut self) {
//         self.user = String::new();
//         self.pwd = String::new();
//     }

//     // fn password_ui(&mut self, )
// }

// impl Default for Login {
//     fn default() -> Self {
//         Self {
//             user: String::new(),
//             pwd: String::new(),
//             show_pwd: false,
//         }
//     }
// }

// impl eframe::App for Login {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {

//             ui.horizontal(|ui| {
//                     ui.label("User :");
//                     ui.text_edit_singleline(&mut self.user);
//             });

//             ui.horizontal(|ui| {
//                 ui.label("password :");
//                 ui.text_edit_singleline(&mut self.pwd);

//                 if ui.button("hide").on_hover_text("hide/ show").clicked() {
//                     self.show_pwd = !self.show_pwd;
//                 }
//             });

//             ui.horizontal(|ui| {
//                 if ui.button("Cancel").clicked() {
//                     // self.user = String::new();
//                     // self.pwd = String::new();
//                     self.clear_field();
//                 }

//                 if ui.button("Login").clicked() {
//                     println!("login");
//                     println!("user : '{}'", self.user);
//                     println!("pwd : '{}'", self.pwd);
                    
//                     self.clear_field();

//                 }
//             });
//         });
//     }
// }