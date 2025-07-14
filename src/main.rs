mod get_dirlistr;
mod get_dirlistc;
mod rc_rotatepress;
mod c8_copypress;
mod get_winsize;

use get_dirlistr::get_dirlistr;
use get_dirlistc::get_dirlistc;
use get_winsize::get_winsize;

// use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, text, column, button, Row, row, Radio, progress_bar, horizontal_space, Space, container, Scrollable, scrollable, Text, checkbox};
use iced::{Element, Task, Length, Alignment, Color, Center};

use iced::theme::Theme;

// use serde::{Deserialize, Serialize};

use std::path::Path;
use std::process::Command as stdCommand;
use std::time::{Duration, Instant};
use std::thread::sleep;
use iced::futures;
use futures::channel::mpsc;
use crate::rc_rotatepress::rc_rotatepress;
use c8_copypress::c8_copypress;

fn main() -> iced::Result {
     let mut widthxx: f32 = 1350.0;
     let mut heightxx: f32 = 750.0;
     let (errcode, _errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho as f32 - 20.0;
         heightxx = heighto as f32 - 75.0;
//         println!("{}", errstring);
//     } else {
//         println!("**ERROR {} get_winsize: {}", errcode, errstring);
     }
     iced::application(PhotoRot1080::update, PhotoRot1080::view)
        .window_size((widthxx, heightxx))
        .theme(PhotoRot1080::theme)
//        .subscription(ImageList::subscription)
        .run()

}

struct PhotoRot1080 {
    dir_value: String,
    mess_color: Color,
    msg_value: String,
    do_progress: bool,
    progval: f32,
    tx_send: mpsc::UnboundedSender<String>,
    rx_receive: mpsc::UnboundedReceiver<String>,
     pagechoice_value: PageChoice,
    c8scrol_value: String,
    rcscrol_value: String,
    outdir_value: String,
    filterf: Filterf,
    files: Vec<File>,

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PageChoice {
    ROT,
    CON,
 }

impl Default for PageChoice {
    fn default() -> Self {
        PageChoice::ROT
    }
}

#[derive(Clone, Debug)]
enum Message {
    ProgressPressed,
    ProgRtn(Result<Progstart, Error>),
    RotatexFound(Result<Rotatex, Error>),
    CopyxFound(Result<Copyx, Error>),
    PageRadioSelected(PageChoice),
    RCListPressed,
    RCRotallPressed,
    C8OutDirPressed,
    C8ListPressed,
    C8CopyPressed,
    FilterChangedf(Filterf),
    FileMessage(usize, FileMessage),
}

impl PhotoRot1080 {
    fn new() -> (Self, iced::Task<Message>) {
        let (tx_send, rx_receive) = mpsc::unbounded();
        (  PhotoRot1080 {
                dir_value: "no directory".to_string(),
                mess_color: Color::from([0.5, 0.5, 1.0]),
                msg_value: "no message".to_string(),
                do_progress: false,
                pagechoice_value: PageChoice::ROT,
                progval: 0.0,
                tx_send,
                rx_receive,
                c8scrol_value: " nothing to process ".to_string(),
                rcscrol_value: " nothing to process ".to_string(),
                outdir_value: String::new(),
                filterf:Filterf::All,
                files:Vec::<File>::new(),
           },
            Task::none(),
        )

    }

    fn title(&self) -> String {
        String::from("File dialog test")
    }

    fn update(&mut self, message: Message) -> Task<Message>  {
        match message {

            Message::PageRadioSelected(xchoice) => {
                let strx = match xchoice {
                PageChoice::ROT => "page choice rotate correction selected",
                PageChoice::CON => "page choice file dialog selected",};
                self.pagechoice_value = xchoice;
                self.mess_color = Color::from([0.0, 1.0, 0.0]);
                self.msg_value = strx.to_string();
                Task::none()
            }
            Message::FilterChangedf(filterf) => {
                self.filterf = filterf;
                Task::none()
            }
            Message::FileMessage(i, file_message) => {
                if let Some(file) = self.files.get_mut(i) {
                    file.update(file_message);
                    Task::none()
                } else {
                    Task::none()
                }
            }

            Message::RCListPressed => {
                self.rcscrol_value = " nothing to process ".to_string();
                if !Path::new(&self.dir_value).exists() {
                    self.msg_value = format!("directory does not exist: {}", self.dir_value);
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                } else {
                    let dir_path = Path::new(&self.dir_value);
                    let (errcd, errstr, newliststr) = get_dirlistr(dir_path.to_path_buf());
                    if errcd == 0 {
                        self.rcscrol_value  = newliststr.to_string();
                        self.msg_value = format!("directory entries for: {}", self.dir_value);
                        self.mess_color = Color::from([0.0, 1.0, 0.0]);
                    } else {
                        self.msg_value = errstr.to_string();
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                    }
                }
                Task::none()
            }
            Message::C8ListPressed => {
//                self.c8scrol_value = " nothing to process ".to_string();
//                if !Path::new(&self.dir_value).exists() {
//                    self.msg_value = format!("directory does not exist: {}", self.dir_value);
//                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
//                } else {
//                    let dir_path = Path::new(&self.dir_value);
                let (errcd, errstr, newdir, listitems) = get_dirlistc(self.dir_value.clone());
                self.msg_value = errstr.to_string();
                if errcd == 0 {
                    self.files.clear();                         
                    self.outdir_value = newdir.to_string();
                    let listitemlen = listitems.len();
                    let newtoi = listitemlen as i32 ;
                    for indexi in 0..newtoi {
                         self.files.push(File::new(listitems[indexi as usize].clone()));
                    } 
                    self.mess_color = Color::from([0.0, 1.0, 0.0]);


//                    self.c8scrol_value  = newliststr.to_string();
//                    self.msg_value = format!("directory entries for: {}", self.dir_value);
//                    self.mess_color = Color::from([0.0, 1.0, 0.0]);
                } else {
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                }
//                }
                Task::none()
            }
            Message::C8OutDirPressed => {
                let mut a_dir: String = self.dir_value.clone().to_string();
                if !Path::new(&a_dir).exists() {
                    a_dir = self.outdir_value.clone().to_string();
                }
//                let (errcode, errstr, newdir) = c8_diroutpress(a_dir);
//                self.msg_value = errstr.to_string();
//                if errcode == 0 {
//                    self.outdir_value = newdir.to_string();
                    self.mess_color = Color::from([0.0, 1.0, 0.0]);
//                } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
//                }
                Task::none()
            }
            Message::C8CopyPressed => {
                let (errcode, errstr) = c8_copypress(self.dir_value.clone(), self.outdir_value.clone(), self.c8scrol_value.clone());
                self.msg_value = errstr.to_string();
                if errcode == 0 {
                    self.mess_color = Color::from([0.0, 1.0, 0.0]);
                    Task::perform(Copyx::copyit(self.dir_value.clone(), self.outdir_value.clone(), self.c8scrol_value.clone(), self.tx_send.clone()), Message::CopyxFound)
                } else {
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                    Task::none()
                }
            }
            Message::RCRotallPressed => {
                if !Path::new(&self.dir_value).exists() {
                    self.msg_value = format!("directory does not exist: {}", self.dir_value);
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                    Task::none()
                } else {
                    let dir_path = Path::new(&self.dir_value);
                    let (errcd, errstr, newliststr) = get_dirlistr(dir_path.to_path_buf());
                    if errcd == 0 {
                        let (errrc, errstrrc) = rc_rotatepress(self.dir_value.clone(), newliststr.clone());
                        self.msg_value = errstrrc.to_string();
                        if errrc == 0 {
                            self.mess_color = Color::from([0.0, 1.0, 0.0]);
                            Task::perform(Rotatex::rotateit(self.dir_value.clone(), newliststr.clone(), self.tx_send.clone()), Message::RotatexFound)
                        } else {
                            self.mess_color = Color::from([1.0, 0.0, 0.0]);
                            Task::none()
                        }
                    } else {
                        self.msg_value = errstr.to_string();
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                        Task::none()
                    }
                }
            }


            Message::ProgressPressed => {
                   self.do_progress = true;
                   Task::perform(Progstart::pstart(), Message::ProgRtn)
            }
            Message::CopyxFound(Ok(copyx)) => {
                self.msg_value = copyx.errval.clone();
                self.mess_color = copyx.errcolor.clone();
                self.do_progress = false;
                self.progval = 0.0;
                Task::none()
            }
            Message::CopyxFound(Err(_error)) => {
                self.msg_value = "error in copyx copyit routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
                Task::none()
            }
            Message::RotatexFound(Ok(copyx)) => {
                self.msg_value = copyx.errval.clone();
                self.mess_color = copyx.errcolor.clone();
                self.do_progress = false;
                self.progval = 0.0;
               Task::none()
            }
            Message::RotatexFound(Err(_error)) => {
                self.msg_value = "error in copyx copyit routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Task::none()
            }
            Message::ProgRtn(Ok(_prx)) => {
              if self.do_progress {
                let mut inputval  = " ".to_string();
                let mut bgotmesg = false;
                while let Ok(Some(input)) = self.rx_receive.try_next() {
                   inputval = input;
                   bgotmesg = true;
                }
                if bgotmesg {
                    let progvec: Vec<&str> = inputval[0..].split("|").collect();
                    let lenpg1 = progvec.len();
                    if lenpg1 == 3 {
                        let prog1 = progvec[0].to_string();
                        if prog1 == "Progress" {
                            let num_int: i32 = progvec[1].parse().unwrap_or(-9999);
                            if num_int == -9999 {
                                self.msg_value = format!("progress numeric not numeric: {}", inputval);
                                self.mess_color = Color::from([1.0, 0.0, 0.0]);
                            } else {
                                let dem_int: i32 = progvec[2].parse().unwrap_or(-9999);
                                if dem_int == -9999 {
                                    self.msg_value = format!("progress numeric not numeric: {}", inputval);
                                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                                } else {
                                    self.progval = 100.0 * (num_int as f32 / dem_int as f32);
                                    self.msg_value = format!("Convert progress: {} of {}", num_int, dem_int);
                                    self.mess_color = Color::from([0.5, 0.5, 1.0]);
                                }
                            }
                        } else {
                            self.msg_value = format!("message not progress: {}", inputval);
                            self.mess_color = Color::from([1.0, 0.0, 0.0]);
                        }
                    } else {
                        self.msg_value = format!("message not progress: {}", inputval);
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                    }
                }             
                Task::perform(Progstart::pstart(), Message::ProgRtn)
              } else {
                Task::none()
              }
            }
            Message::ProgRtn(Err(_error)) => {
                self.msg_value = "error in Progstart::pstart routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
            let selected_pagechoice = Some(self.pagechoice_value);
            let ua = Radio::new(
                     "Rotate Correction",
                     PageChoice::ROT,
                     selected_pagechoice,
                     Message::PageRadioSelected,
            ).size(15);
            let ub = Radio::new(
                     "File Dialog",
                     PageChoice::CON,
                     selected_pagechoice,
                     Message::PageRadioSelected,
            ).size(15);



                let controlsf = view_controlsf(&self.files, *&self.filterf);
                let filtered_files =
                    self.files.iter().filter(|file| self.filterf.matches(file));

                let mut filescol1 = Column::new().spacing(10);
                let mut n = 0;
                if filtered_files.clone().count() == 0 {
                    filescol1 = filescol1.push(container(row![empty_message(match self.filterf {
                        Filterf::All => "No directory selected or no files in directory",
                        Filterf::Active => "All files have been selected",
                        Filterf::Completed => "No files have been selected" 
                    })]));
                } else {
                    for filesy in self.files.iter() {
                         if filesy.completed {
                             if (self.filterf == Filterf::All) || (self.filterf == Filterf::Completed) {
                                 filescol1 = filescol1.push(container(row![filesy.view(n).map(move |message| {
                                    Message::FileMessage(n, message)
                                   })]));
                             }
                         } else {
                             if (self.filterf == Filterf::All) || (self.filterf == Filterf::Active) {
                                 filescol1 = filescol1.push(container(row![filesy.view(n).map(move |message| {
                                    Message::FileMessage(n, message)
                                   })]));
                             }
                         }
                         n = n + 1;
                    }
                }
                let mut filesrow = Row::new().spacing(20);
                filesrow = filesrow.push(container(filescol1).padding(10).width(Length::Fixed(500.0)));

                let scrollable_contentf: Element<Message> =
                  Element::from(scrollable(
                    filesrow
                )
                .height(Length::Fill)
                .width(Length::Fixed(500.0))
                .direction({
                    let scrollbar = scrollable::Scrollbar::new()
                        .width(10)
                        .margin(10)
                        .scroller_width(10);
//                        .anchor(self.anchor);

                    scrollable::Direction::Both {
                        horizontal: scrollbar,
                        vertical: scrollbar,
                    }
                 })
                ); 








       
            let mut topshow = Column::new().spacing(10);
            topshow = topshow.push(container(row![text("Message:").size(20),
                                              text(&self.msg_value).size(20).color(*&self.mess_color),
                                              ].align_y(Alignment::Center).spacing(10).padding(10),
            ));
            topshow = topshow.push(container(row![
                                              text(&self.dir_value).size(20),
                                              ].align_y(Alignment::Center).spacing(10).padding(5),
            ));
            topshow = topshow.push(container(row![button("Start Progress Button").on_press(Message::ProgressPressed),
                                              progress_bar(0.0..=100.0,self.progval),
                                              text(format!("{:.1}%", &self.progval)).size(30),
                                              ].align_y(Alignment::Center).spacing(5).padding(10),
            ));
            topshow = topshow.push(container(row![horizontal_space(), ua, horizontal_space(),
                                              ub, horizontal_space(), 
                                              ].align_y(Alignment::Center).spacing(5).padding(10),
            ));

            let mut subshow = Column::new().spacing(10).align_x(Alignment::Center);

            match self.pagechoice_value  {
                PageChoice::CON => {
                    subshow = subshow.push(container(row![horizontal_space(),
                                                                         button("Out Directory Button").on_press(Message::C8OutDirPressed),
                                                                         text(&self.outdir_value).size(20), 
                                                                         horizontal_space(),
                                              ].align_y(Alignment::Center).spacing(10).padding(10),
                        ));
                    subshow = subshow.push(container(row![horizontal_space(),
                                                                         button("List Directory Button").on_press(Message::C8ListPressed),
                                                                         horizontal_space(),
                                                                         button("Copy Button").on_press(Message::C8CopyPressed), 
                                                                         horizontal_space(),
                                              ].align_y(Alignment::Center).spacing(10).padding(10),
                         ));
                    subshow = subshow.push(container(controlsf
                        ),
                           );
                    subshow = subshow.push(container(scrollable_contentf
                        ),
                           );
/*
                    subshow = subshow.push(container(Scrollable::new(
                        Column::new()
                           .width(Length::Fill)
                           .align_x(Alignment::Center)
                           .push(
                              Text::new(format!("{}",&self.c8scrol_value)),
                           )
                           ).height(Length::Fill),
                        ),
                           );
*/
                },
                PageChoice::ROT => {
                    subshow = subshow.push(container(row![horizontal_space(),
                                                                         button("List Orientation Button").on_press(Message::RCListPressed),
                                                                         horizontal_space(),
                                                                         button("Rotate All Button").on_press(Message::RCRotallPressed), 
                                                                         horizontal_space(),
                                              ].align_y(Alignment::Center).spacing(10).padding(10),
                         ));
                    subshow = subshow.push(container(Scrollable::new(
                        Column::new()
                           .width(Length::Fill)
                           .align_x(Alignment::Center)
                           .push(
                              Text::new(format!("{}",&self.rcscrol_value)),
                           )
                           ).height(Length::Fill),
                        ),
                           );

                },
           }
            
        column![

         topshow,
         subshow,
         ]
         .padding(1)
        .into()

    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }
 
}

#[derive(Debug, Clone)]
pub struct Rotatex {
    errcolor: Color,
    errval: String,
}

impl Rotatex {

    pub async fn rotateit(dir_value: String, mergescrol_value: String, tx_send: mpsc::UnboundedSender<String>,) -> Result<Rotatex, Error> {
     let mut errstring  = " ".to_string();
     let mut colorx = Color::from([0.0, 1.0, 0.0]);
     let mut bolok = true;
     let mut numrow = 0;
     let mut numprocess = 0;
     let mergelistvec: Vec<&str> = mergescrol_value[0..].split("\n").collect();
     let mut lenmg1 = mergelistvec.len();
     lenmg1 = lenmg1 -1;
     let start_time = Instant::now();
     for indl in 0..lenmg1 {
          let str_cur_dirfrom = dir_value.clone();
          let linestr = mergelistvec[indl];
          let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
          let filefromx = lineparse[0].to_string();
          let fullfrom = str_cur_dirfrom.clone() + "/" + &filefromx[1..];
          if !Path::new(&fullfrom).exists() {
              errstring = format!("********* convert Copy: ERROR {} does not exist **********",fullfrom);
              colorx = Color::from([1.0, 0.0, 0.0]);
              bolok = false;
              break;
          }
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
                      if numprocess < 4 {
                          stdCommand::new("/home/jp/gimp.sh")
                             .arg(&fullfrom)
                             .spawn()
                             .expect("failed to execute process");
                          numprocess = numprocess + 1;
                      } else {
                          let _output = stdCommand::new("/home/jp/gimp.sh")
                               .arg(&fullfrom)
                               .output()
                               .expect("failed to execute process");
                          numprocess = 0;
                          let msgx = format!("Progress|{}|{}", numrow, lenmg1);
                          tx_send.unbounded_send(msgx).unwrap();
                      }
                  }
              }
          }
     }
     if bolok {
         let diffx = start_time.elapsed();     
         errstring = format!("rotated {} files in {} seconds", lenmg1, diffx.as_secs());
         colorx = Color::from([0.0, 1.0, 0.0]);
     }
     Ok(Rotatex {
            errcolor: colorx,
            errval: errstring,
        })
    }
}

#[derive(Debug, Clone)]
struct Copyx {
    errcolor: Color,
    errval: String,
}

impl Copyx {

    async fn copyit(dir_value: String, outdir_value: String, mergescrol_value: String, tx_send: mpsc::UnboundedSender<String>,) -> Result<Copyx, Error> {
     let mut errstring  = " ".to_string();
     let mut colorx = Color::from([0.0, 1.0, 0.0]);
     let mut bolok = true;
     let mut numrow = 0;
     let mut numprocess = 0;
     let mergelistvec: Vec<&str> = mergescrol_value[0..].split("\n").collect();
     let mut lenmg1 = mergelistvec.len();
     lenmg1 = lenmg1 -1;
     let start_time = Instant::now();
     for indl in 0..lenmg1 {
          let str_cur_dirfrom = dir_value.clone();
          let linestr = mergelistvec[indl];
          let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
          let filefromx = lineparse[0].to_string();
          let fullfrom = str_cur_dirfrom.clone() + "/" + &filefromx[1..];
          if !Path::new(&fullfrom).exists() {
              errstring = format!("********* convert Copy: ERROR {} does not exist **********",fullfrom);
              colorx = Color::from([1.0, 0.0, 0.0]);
              bolok = false;
              break;
          }
          let str_cur_dirout = outdir_value.clone();
          let fullto = str_cur_dirout.clone() + "/" + &filefromx;
          if Path::new(&fullto).exists() {
              errstring = format!("********* convert Copy: ERROR {} already exists **********", fullto);
              colorx = Color::from([1.0, 0.0, 0.0]);
              bolok = false;
              break;
          }
          if numprocess < 4 {
              stdCommand::new("magick")
                           .arg(&fullfrom)
                           .arg("-resize")
                           .arg("1920x1080")
                           .arg("-background")
                           .arg("black")
                           .arg("-gravity")
                           .arg("center")
                           .arg("-extent")
                           .arg("1920x1080")
                           .arg(&fullto)
                           .spawn()
                           .expect("failed to execute process");
              numprocess = numprocess + 1;
          } else {
              let _output = stdCommand::new("magick")
                           .arg(&fullfrom)
                           .arg("-resize")
                           .arg("1920x1080")
                           .arg("-background")
                           .arg("black")
                           .arg("-gravity")
                           .arg("center")
                           .arg("-extent")
                           .arg("1920x1080")
                           .arg(&fullto)
                           .output()
                           .expect("failed to execute process");
              numprocess = 0;
              let msgx = format!("Progress|{}|{}", numrow, lenmg1);
              tx_send.unbounded_send(msgx).unwrap();

          }

          numrow = numrow + 1;
     }
     if bolok {
         let diffx = start_time.elapsed();     
         errstring = format!("converted copied {} files in {} seconds", lenmg1, diffx.as_secs());
         colorx = Color::from([0.0, 1.0, 0.0]);
     }
     Ok(Copyx {
            errcolor: colorx,
            errval: errstring,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Error {
//    APIError,
}
// loop thru by sleeping for 5 seconds
#[derive(Debug, Clone)]
pub struct Progstart {
//    errcolor: Color,
//    errval: String,
}

impl Progstart {

    pub async fn pstart() -> Result<Progstart, Error> {
//     let errstring  = " ".to_string();
//     let colorx = Color::from([0.0, 0.0, 0.0]);
     sleep(Duration::from_secs(5));
     Ok(Progstart {
//            errcolor: colorx,
//            errval: errstring,
        })
    }
}


// #[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Debug, Clone)]
struct File {
    description: String,
    completed: bool,

}

#[derive(Debug, Clone)]
pub enum FileMessage {
    Completed(bool),
}

impl File {

    fn new(description: String) -> Self {
        File {
            description,
            completed: false,
        }
    }

    fn update(&mut self, message: FileMessage) {
        match message {
            FileMessage::Completed(completed) => {
                self.completed = completed;
            }

        }
    }

    fn view(&self, _i: usize) -> Element<FileMessage> {
                let checkbox = checkbox(
                    &self.description,
                    self.completed).on_toggle(FileMessage::Completed).width(Length::Fixed(500.0));

                row![
                    checkbox,

                ]
                .spacing(20)
                .align_y(Alignment::Center)
                .into()

    }
}
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filterf {
    All,
    Active,
    Completed,
}

impl Default for Filterf {
    fn default() -> Self {
        Filterf::All
    }
}

impl Filterf {
    fn matches(&self, file: &File) -> bool {
        match self {
            Filterf::All => true,
            Filterf::Active => !file.completed,
            Filterf::Completed => file.completed,
        }
    }
}

fn view_controlsf(files: &[File], current_filter: Filterf) -> Element<Message> {
    let files_left = files.iter().filter(|file| file.completed).count();

    let filter_button = |label, filterf, current_filter| {
        let label = text(label).size(16);

        let button = button(label).style(if filterf == current_filter {
            button::primary
        } else {
            button::text
        });

        button.on_press(Message::FilterChangedf(filterf)).padding(8)
    };

        row![Space::with_width(Length::Fixed(20.0)),
            text(format!(
            "{} {} selected",
            files_left,
            if files_left == 1 { "file" } else { "files" }
        ))
        .size(16),

            filter_button("All", Filterf::All, current_filter),
            filter_button("Not Selected", Filterf::Active, current_filter),
            filter_button("Selected", Filterf::Completed, current_filter,),
        ]
        .width(Length::Shrink)
        .spacing(10)
    .align_y(Alignment::Center)
    .into()
}


fn empty_message(message: &str) -> Element<'_, Message> {
    container(
        text(message)
            .width(Length::Fill)
            .size(25)
            .align_x(Center)
            .color([0.7, 0.7, 0.7]),
    )
    .width(Length::Fill)
    .height(Length::Fixed(200.0))
    .into()
}
