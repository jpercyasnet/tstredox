mod get_dirlistc;
mod get_winsize;

use get_dirlistc::get_dirlistc;
use get_winsize::get_winsize;

use iced::widget::{Column, text, column, button, Row, row, Radio, horizontal_space, Space, container, scrollable, checkbox};
use iced::{Element, Task, Length, Alignment, Color, Center};

use iced::theme::Theme;

use std::path::Path;

fn main() -> iced::Result {
     let mut widthxx: f32 = 1350.0;
     let mut heightxx: f32 = 750.0;
     let (errcode, _errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho as f32 - 20.0;
         heightxx = heighto as f32 - 75.0;
     }
     iced::application(FileDialogX::new, FileDialogX::update, FileDialogX::view)
        .window_size((widthxx, heightxx))
        .theme(FileDialogX::theme)
        .title(FileDialogX::title)
        .run()

}

struct FileDialogX {
    dir_value: String,
    mess_color: Color,
    msg_value: String,
    pagechoice_value: PageChoice,
    outdir_value: String,
    filterf: Filterf,
    files: Vec<File>,
    getdiritems: bool,

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
    PageRadioSelected(PageChoice),
    RCListPressed,
    C8SetDirPressed,
    C8ListPressed,
    C8ChgDirPressed,
    FilterChangedf(Filterf),
    FileMessage(usize, FileMessage),
    GetDirItemsChk(bool),
}

impl FileDialogX {
    fn new() -> (Self, iced::Task<Message>) {
        (  FileDialogX {
                dir_value: "no directory".to_string(),
                mess_color: Color::from([0.5, 0.5, 1.0]),
                msg_value: "no message".to_string(),
                pagechoice_value: PageChoice::ROT,
                outdir_value: String::new(),
                filterf:Filterf::All,
                files:Vec::<File>::new(),
                getdiritems: false,
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
                if !Path::new(&self.dir_value).exists() {
                    self.msg_value = format!("directory does not exist: {}", self.dir_value);
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                } else {
                    self.msg_value = format!("directory exist: {}", self.dir_value);
                    self.mess_color = Color::from([0.0, 1.0, 0.0]);
                }
                Task::none()
            }
            Message::C8ListPressed => {
                let (errcd, errstr, newdir, listitems) = get_dirlistc(self.dir_value.clone(), self.getdiritems.clone());
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
                } else {
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                }
                Task::none()
            }
            Message::C8SetDirPressed => {
                let a_dir: String = self.outdir_value.clone();
                if Path::new(&a_dir).exists() {
                    self.dir_value = a_dir;
                    self.msg_value = format!("directory has been set with {}", self.dir_value);
                    self.mess_color = Color::from([0.0, 1.0, 0.0]);
                } else {
                    self.msg_value = format!("directory {} does not exist", a_dir);
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                }
                Task::none()
            }
            Message::GetDirItemsChk(checkx) => {
                self.getdiritems = checkx;
                Task::none()
            } 
            Message::C8ChgDirPressed => {
                let files_selected = self.files.iter().filter(|fileitem| fileitem.completed).count();
                if files_selected < 1 {
                    self.msg_value = "no item selected".to_string();
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                } else if files_selected > 1 {
                    self.msg_value = "more than 1 item selected".to_string();
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                } else {
                    let mut itemstr: String = " ".to_string();
                    for filesy in self.files.iter() {
                         if filesy.completed {
                             itemstr = filesy.description.clone();
                         }
                    }
                    let lineparse: Vec<&str> = itemstr[0..].split(" | ").collect();
                    if lineparse[0] != "DIR" {
                        self.msg_value = format!("{} is not a directory", itemstr);
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                    } else {
                        let newdirx: String;
                        if lineparse[2] == "..parent" {
                            newdirx = lineparse[1].to_string();
                        } else {
                            newdirx = format!("{}/{}", self.outdir_value, lineparse[1]);
                        }
                        let (errcd, errstr, newdir, listitems) = get_dirlistc(newdirx.clone(), self.getdiritems.clone());
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
                         } else {
                            self.mess_color = Color::from([1.0, 0.0, 0.0]);
                         }
                    }
                }
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
            topshow = topshow.push(container(row![horizontal_space(), ua, horizontal_space(),
                                              ub, horizontal_space(), 
                                              ].align_y(Alignment::Center).spacing(5).padding(10),
            ));

            let mut subshow = Column::new().spacing(10).align_x(Alignment::Center);

            match self.pagechoice_value  {
                PageChoice::CON => {
                    subshow = subshow.push(container(row![horizontal_space(),
                                                                         button("Set Directory Button").on_press(Message::C8SetDirPressed),
                                                                         text(&self.outdir_value).size(20), 
                                                                         horizontal_space(),
                                              ].align_y(Alignment::Center).spacing(10).padding(10),
                        ));
                    subshow = subshow.push(container(row![horizontal_space(),
                                                          checkbox("Get Directory Items", self.getdiritems).on_toggle(Message::GetDirItemsChk),
                                                          horizontal_space(),
                                                          button("List Directory Button").on_press(Message::C8ListPressed),
                                                          horizontal_space(),
                                                          button("Change Directory Button").on_press(Message::C8ChgDirPressed), 
                                                          horizontal_space(),
                                              ].align_y(Alignment::Center).spacing(10).padding(10),
                         ));
                    subshow = subshow.push(container(controlsf
                        ),
                           );
                    subshow = subshow.push(container(scrollable_contentf
                        ),
                           );
                },
                PageChoice::ROT => {
                    subshow = subshow.push(container(row![horizontal_space(),
                                                                         button("List Orientation Button").on_press(Message::RCListPressed),
                                                                         horizontal_space(),
                                                                         ].align_y(Alignment::Center).spacing(10).padding(10),
                         ));
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
pub enum Error {
//    APIError,
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
