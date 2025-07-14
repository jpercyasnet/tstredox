use std::path::Path;

pub fn rc_rotatepress (dir_value: String, mergescrol_value: String) -> (u32, String) {
     let mut errcode: u32 = 0;
     let mut errstring: String = "xx".to_string();
     let mut numrow = 0;
     let mut bolok = true;
     if Path::new(&dir_value).exists() {
         let mergelistvec: Vec<&str> = mergescrol_value[0..].split("\n").collect();
         let mut lenmg1 = mergelistvec.len();
         if lenmg1 < 2 {
             errstring = "no images to rotate".to_string();
             errcode = 1;
         } else {
             lenmg1 = lenmg1 - 1;
             for indl in 0..lenmg1 {
                let str_cur_dirfrom = dir_value.clone();
                let linestr = mergelistvec[indl];
                let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
                let filefromx = lineparse[0].to_string();
                let fullfrom = str_cur_dirfrom.clone() + "/" + &filefromx[1..];
                if !Path::new(&fullfrom).exists() {
                    errstring = format!("********* convert Copy: ERROR {} does not exist **********",fullfrom);
                    bolok = false;
                    errcode = 2;
                    break;
                } else {
                    let strval = lineparse[1].to_string();
                    let locind = strval.find("orientation");
                    if locind != None {
                        let start = locind.unwrap();
                        let start = start + 13;
                        let end = start + 1;
                        let getorient = strval.get(start..end);
                        let orient_int: i32 = getorient.unwrap().parse().unwrap_or(-99);
                        if orient_int > 0 {
                            if (orient_int == 3) | 
                               (orient_int == 6) |
                               (orient_int == 8) {
                                numrow = numrow + 1;
                            }
                        }
                    }
                }
             }
             if bolok {
                 if numrow > 0 {
                     errstring = "Rotating in Progress".to_string();
                     errcode = 0;
                 } else {
                     errstring = "no images to rotate".to_string();
                     errcode = 3;
                 }
             }
         }
     } else {
         errstring = "the directory does not exist".to_string();
         errcode = 4;
     }
     (errcode, errstring)
}

