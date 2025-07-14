extern crate exif;
use std::fs;
use std::path::Path;
use std::env;

pub fn get_dirlistc (current_dir: String) -> (u32, String, String, Vec<String>) {
    let errcode: u32;
    let errstring: String;
    let mut strpath: String = current_dir;
//    let mut new_dirlist: String = " ".to_string();
    let mut orient;
    let mut listitems: Vec<String> = Vec::new();

    let mut new_path = Path::new(&strpath);
    if !new_path.exists() {
//        let res = env::current_dir();
//        match res {
//            Ok(path) => path.into_os_string().into_string().unwrap(),
//            Err(_) => "FAILED".to_string()
//        }
        strpath = env::current_dir().expect("REASON").into_os_string().into_string().unwrap();
        new_path = Path::new(&strpath);
    }

//                    self.msg_value = format!("directory does not exist: {}", self.dir_value);
//                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
//                } else {
//                    let dir_path = Path::new(&self.dir_value);
    let mut numentry = 0;
    let parentval = new_path.parent().expect("REASON").to_str().unwrap();
    let listin = format!("{} | dir", parentval);
    listitems.push(listin);
    for entry1 in fs::read_dir(&new_path).unwrap() {
         let entry = entry1.unwrap();
         if let Ok(metadata) = entry.metadata() {
             if let Ok(file_name) = entry.file_name().into_string() {
                 if metadata.is_file() {
                     orient = format!("file");
                 } else if metadata.is_dir() {
                     orient = format!("dir");
                 } else {
                     orient = format!("{:?}", metadata.file_type());
                 }
                 let listival = file_name + " | " + &orient;
                 listitems.push(listival);
                 numentry = numentry + 1;
             }
         }
    }
    if numentry > 0 {
//        listitems.sort();
        errstring = format!("{} items in directory ", numentry);
        errcode = 0;
    } else {
        errstring = "********* Directory 1: directory has no items **********".to_string();
        errcode = 1;
    }
    (errcode, errstring, strpath, listitems)
}

