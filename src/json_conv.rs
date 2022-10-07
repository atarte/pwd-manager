use serde::de;
// use serde_derive::{Serialize, Deserialize};
// use serde_json::de;

use std::{fs::{self, File}, path::Path};

// use crate::panel::Init;



// pub fn import_json<'a, T: de::Deserialize<'a>>(path: &str, data: &mut T) {
//     // let data: D;
//     
//     if !Path::new(path).exists() <LeftMouse<LeftMouse>
//         // let data: D = data.init();
//         // 
//         export_json(path, data);
//         // return data;
//     }
//     else {
//         let file_str: String = fs::read_to_string(path)
//             .expect("The file is not here so fuck it");
//
//         data = serde_json::from_str(&file_str)
//             .expect("Can't serialize to load_json");
//     }
// }
//
// pub fn export_json<T>(path: &str, data: T) {
//     let file: File = File::create(path)
//         .expect("Can't open the data file");
//
//     serde_json::to_writer_pretty(&file, data)
//         .expect("Can't write in the data file")
// }
