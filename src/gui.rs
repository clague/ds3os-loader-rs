use iced::{Application, executor, Command, Row, button, Button, Text, Column, Alignment, Length, Font};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use std::io::{BufReader};
use anyhow::{Result};
use std::fs::File;

use crate::api::{Server, MasterServerApi};
use crate::launch::Launcher;
use crate::table::{ServerList, ListMessage};
use crate::widgets::topbar::{TopBar, TopBarMessage};
use crate::localize::FailReasonString;

pub static ICON_FONT: Font = Font::External { 
    name: "Icons",
    bytes: include_bytes!("../resources/icons/icons.ttf"),
};

pub struct LoaderMainInterface {
    // The counter value
    launcher: Launcher,

    // The local state of the two buttons
    topbar: TopBar,
    launch: button::State,
    server_list: ServerList,
}

#[derive(Debug, Clone)]
pub enum Message {
    ListMessage(ListMessage),
    TopBarMessage(TopBarMessage),
    Launch,
    Fail(FailReason, String),
    Nothing,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum FailReason {
    ChooseFileFail,
    RefreshListFail,
    PatchFail,
    ListNoSelected,
    ProcessNotFound,
}


impl Application for LoaderMainInterface {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            LoaderMainInterface{
                launcher: Launcher::new(),

                topbar: TopBar::new(),
                launch: button::State::new(),
                server_list: ServerList::new(),
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        "Dark Souls III - Yet Another Open Server Loader".to_string()
    }

    fn update(
        &mut self,
        message: Self::Message
    ) -> Command<Self::Message> {
        match message {
            Message::Launch => {
                match self.launcher.find_process() {
                    Ok(pid) => {
                        if let Some(row) = self.server_list.rows.get((self.server_list.selected - 1) as usize) {
                            let api = self.server_list.api.clone();
                            let server = row.server.clone();
                            Command::perform(Launcher::patch_game(pid, api, server, ""), 
                                |r| {
                                    match r {
                                        Ok(_) => {
                                            Message::Nothing
                                        },
                                        Err(e) => {
                                            Message::Fail(FailReason::PatchFail, e.to_string())
                                        }
                                    }
                                })
                        }
                        else {
                            self.update(Message::Fail(FailReason::ListNoSelected, "No row is selected".into()))
                        }
                    }, 
                    Err(e) => {
                        self.update(Message::Fail(FailReason::ProcessNotFound, e.to_string()))
                    }
                }
                
            },
            Message::ListMessage(m) => {
                self.server_list.update(m).map(list_message_map)
            },
            Message::TopBarMessage(m) => {
                match m {
                    TopBarMessage::ChooseConfigFile => {
                        let mes = match choose_config_file() {
                            Ok(servers) => {
                                if servers.len() == 0 {
                                    Message::Fail(FailReason::ChooseFileFail, "".into())
                                }
                                else { Message::ListMessage(ListMessage::ImportConfig(servers)) }
                            },
                            Err(e) => {
                                Message::Fail(FailReason::ChooseFileFail, e.to_string())
                            }
                        };
                        self.update(mes)
                    },
                    TopBarMessage::RefreshServerList => {
                        self.server_list.update(ListMessage::UpdateServerList).map(|m| Message::ListMessage(m))
                    },
                    _ => {
                        self.topbar.update(m).map(|m| Message::TopBarMessage(m))
                    }
                }
            },
            Message::Fail(reason, description) => {
                let text = format!("{}\nDetail: {}", FailReasonString.lock().unwrap()[&reason], &description);
                if let Err(e) = MessageDialog::new()
                    .set_title("Error")
                    //.set_type(MessageType::Error)
                    .set_text(&text)
                    .show_alert()
                    {
                        println!("Error: {}", e);
                    }
                Command::none()
            },
            Message::Nothing => { Command::none() }
        }
    }
    
    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let topbar = self.topbar.view().map(|m| Message::TopBarMessage(m));

        let launch_button = Button::new(&mut self.launch, Text::new("Launch"))
            .on_press(Message::Launch).height(Length::Units(100));

        let heads = ["Name", "Address", "Player Count", "Description"];
        let table = self.server_list.view(heads).map(list_message_map);

        Column::with_children(vec![topbar.into(), table.into(), launch_button.into()]).into()
    }
}
fn choose_config_file() -> Result<Vec<Server>> {
    Ok(FileDialog::new()
        .add_filter("Server Config File", &["ds3osconfig"])
        .add_filter("All files", &["*"])
        .set_location("~/")
        .show_open_multiple_file()?
        .into_iter()
        .filter_map(|path| {
            match File::open(path.clone()) {
                Ok(file) => {
                    serde_json::from_reader::<_, Server>(BufReader::new(file)).ok()
                },
                Err(e) => {
                    println!("Import file '{}' failed! Reason: {}", path.to_string_lossy(), e.to_string());
                    None
                }
            }
        }).collect())
}

fn list_message_map(m: ListMessage) -> Message {
    if let ListMessage::Fail(r, s) = m {
        Message::Fail(r, s)
    }
    else {
        Message::ListMessage(m)
    }
}
