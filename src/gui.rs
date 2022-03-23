use iced::{Application, executor, Command, Column, Font};
use iced_aw::{split, Split};
use native_dialog::{FileDialog, MessageDialog};
use std::io::BufReader;
use anyhow::Result;
use std::fs::File;

use crate::api::{Server, MasterServerApi};
use crate::patch::Patches;
use crate::widgets::list::{ServerList, ListMessage, RowMessage};
use crate::widgets::topbar::{TopBar, TopBarMessage};
use crate::widgets::detail_panel::DetailPanel;
use crate::localize::FailReasonLocalizedString;

pub static ICON_FONT: Font = Font::External { 
    name: "Icons",
    bytes: include_bytes!("../resources/icons/icons.ttf"),
};

pub struct LoaderMainInterface {
    api: MasterServerApi,
    patch: Patches,
    cur_passwd: String,
    // The local state of the two buttons
    topbar: TopBar,
    server_list: ServerList,
    detail_panel: DetailPanel,
    split_pane: split::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    ListMessage(ListMessage),
    TopBarMessage(TopBarMessage),
    PasswordInput(String),
    Patch,
    Fail(FailReason, String),
    OnResize(u16),
    Nothing,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum FailReason {
    ChooseFileFail,
    RefreshListFail,

    ListNoSelected,
    FetchPublicKeyFail,
    ProcessNotFound,
    PatchFail,
}


impl Application for LoaderMainInterface {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            LoaderMainInterface{
                api: MasterServerApi::new(*crate::api::MASTER_SERVER_ADDR_DEF, 1).unwrap(),
                patch: Patches::new(),
                cur_passwd: String::new(),

                topbar: TopBar::new(),
                server_list: ServerList::new(),
                detail_panel: DetailPanel::new(),
                split_pane: split::State::new(None, split::Axis::Vertical),
            },
            Command::perform(async {}, |_| Message::ListMessage(ListMessage::UpdateServerList)) // ugly
        )
    }

    fn title(&self) -> String {
        "Dark Souls III - Another Open Server Loader".to_string()
    }

    fn update(
        &mut self,
        message: Self::Message
    ) -> Command<Self::Message> {
        match message {
            Message::Patch => {
                if let Some(row) = self.server_list.find_selected_mut() {
                    row.server.passwd = self.cur_passwd.clone();

                    match self.patch.find_process() {
                        Ok(pid) => {
                            let api = self.api.clone();

                            let ip_addr = row.server.ip_addr.clone();
                            let mut pubkey = row.server.pubkey.clone();
                            let hostname = row.server.hostname.clone();
                            let passwd = row.server.passwd.clone();

                            Command::perform(async move {
                                    if pubkey.is_empty() {
                                        pubkey = api
                                            .get_pubkey(&ip_addr, &passwd)
                                            .await
                                            .map_err(|e| (FailReason::FetchPublicKeyFail, e.to_string()))?;
                                    }
                                    Patches::patch(pid, &hostname, &pubkey).map_err(|e| (FailReason::PatchFail, e.to_string())).map(|_| ())
                                }, 
                                |r| {
                                    match r {
                                        Ok(_) => {
                                            Message::Nothing
                                        },
                                        Err(e) => {
                                            Message::Fail(e.0, e.1)
                                        }
                                    }
                                })
                        },
                        Err(e) => {
                            self.update(Message::Fail(FailReason::ProcessNotFound, e.to_string()))
                        }
                    }
                }
                else {
                    self.update(Message::Fail(FailReason::ListNoSelected, "No row is selected".into()))
                }
            }, 
            
            Message::ListMessage(m) => {
                if let ListMessage::RowMessage(id, RowMessage::ToggleSelection) = m {
                    if let Some(row) = self.server_list.find_by_id(id) {
                        self.cur_passwd = row.server.passwd.clone();
                    }
                }
                self.server_list.update(m, &self.api).map(map_list_message)
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
                    _ => {
                        self.topbar.update(m).map(map_topbar_message)
                    }
                }
            },
            Message::Fail(reason, description) => {
                let text = format!("{}\nDetail: {}", FailReasonLocalizedString[&reason], &description);
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
            Message::OnResize(pos) => { 
                self.split_pane.set_divider_position(pos);
                Command::none()
            },
            Message::PasswordInput(s) => {
                self.cur_passwd = s;
                Command::none()
            }
            Message::Nothing => { Command::none() },
        }
    }
    
    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let topbar = self.topbar
            .view()
            .map(map_topbar_message);
        let mut col = Column::new()
            .push(topbar);

        let heads = ["Name", "Address", "Player Count"];

        if let Some(row) = self.server_list.rows.iter().filter(|row| row.id == self.server_list.selected).next() {
            let detail_panel = self.detail_panel.view(row.server.clone(), &self.cur_passwd);
            let split = Split::new(
                &mut self.split_pane, 
                self.server_list.view(heads).map(map_list_message),
                detail_panel,
                Message::OnResize
            );
            col = col.push(split);
        }
        else {
            col = col.push(self.server_list.view(heads).map(map_list_message));
        }
        
        col.into()
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

fn map_list_message(m: ListMessage) -> Message {
    if let ListMessage::Fail(r, s) = m {
        Message::Fail(r, s)
    }
    else {
        Message::ListMessage(m)
    }
}

fn map_topbar_message(m: TopBarMessage) -> Message {
    if let TopBarMessage::RefreshServerList = m {
        Message::ListMessage(ListMessage::UpdateServerList)
    }
    else {
        Message::TopBarMessage(m)
    }
}

pub enum Icon {
    Refresh,
    TrashBinLight,
    TrashBinDark,
    OpenFolder,
    PlayLight,
    PlayDark,
}

impl Into<String> for Icon {
    fn into(self) -> String {
        match self {
            Icon::Refresh => "\u{E800}".into(),
            Icon::TrashBinLight => "\u{E801}".into(),
            Icon::TrashBinDark => "\u{E802}".into(),
            Icon::OpenFolder => "\u{E804}".into(),
            Icon::PlayLight => "\u{E805}".into(),
            Icon::PlayDark => "\u{E806}".into(),
        }
    }
}