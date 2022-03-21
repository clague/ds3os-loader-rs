use crate::api::MasterServerApi;

use {
    crate::{
        localize::localized_string,
        api::Server,
    },
    iced::{
        Column, Length, Row, Space, Text, Scrollable, scrollable,
        Command, Element, button, Button, Alignment, Radio
    },
};

pub struct ServerRow {
    pub id: u32,
    pub server: Server,
    pub is_manual: bool,

    server_btn: button::State,
    delete_btn: button::State,
}

#[derive(Clone, Debug)]
pub enum RowMessage {
    Delete,
    ToggleSelection,
}

impl ServerRow {
    pub fn new(server: Server, id: u32, is_manual: bool) -> Self {
        Self {
            id,
            server,
            is_manual,
            server_btn: button::State::new(),
            delete_btn: button::State::new(),
        }
    }

    pub fn view(&mut self, selected: &u32) -> Element<RowMessage> {
        let selection_radio = Radio::new(self.id, "", Some(*selected),  |_| RowMessage::ToggleSelection);
        
        Row::new()
            .push(
                Button::new(
                    &mut self.server_btn,
                    Row::new()
                        .align_items(Alignment::Center)
                        .push(selection_radio)
                        .push(Text::new(&self.server.name).width(Length::FillPortion(1)))
                        .push(Text::new(&self.server.hostname).width(Length::FillPortion(1)))
                        .push(Text::new(&self.server.player_count.to_string()).width(Length::FillPortion(1)))
                        .push(Text::new(&self.server.description).width(Length::FillPortion(1)))
                )
                .padding(8)
                .width(Length::Fill)
                .on_press(RowMessage::ToggleSelection),
            )
            .push(Space::with_width(Length::Units(15)))
            .align_items(Alignment::Center)
            .into()
    }
}

pub struct ServerList {
    pub rows: Vec<ServerRow>,
    pub api: MasterServerApi,
    pub selected: u32,
    pub manual_import_list: Vec<Server>,

    head_btn_name: button::State,
    head_btn_hostname: button::State,
    head_btn_player_count: button::State,
    head_btn_description: button::State,

    scrollable: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum ListMessage {
    //SearchInputChanged(String),
    UpdateServerList,
    UpdateServerListComplete(Vec<Server>),
    UpdateServerListFail(String),
    RowMessage(u32, RowMessage),
    Nothing,
}

impl ServerList {

    pub fn new() -> Self {
        Self::with_servers(Vec::new())
    }
    pub fn with_servers(servers: Vec<Server>) -> Self {
        let mut id = 0;
        Self {
            rows: servers.into_iter().map(|server| {
                id += 1;
                ServerRow::new(server, id, false)
            }).collect(),
            api: MasterServerApi::new(*crate::MASTER_SERVER_ADDR_DEF, 1).unwrap(),
            selected: 0,
            manual_import_list: Vec::new(),

            head_btn_name: button::State::new(),
            head_btn_hostname: button::State::new(),
            head_btn_player_count: button::State::new(),
            head_btn_description: button::State::new(),

            scrollable: scrollable::State::new(),
        }
    }

    pub fn update(&mut self, message: ListMessage) -> Command<ListMessage> {
        match message {
            ListMessage::UpdateServerList => {
                let api = self.api.clone();
                return Command::perform(
                    async move {
                        api.list_servers().await
                    },
                    move |res| {
                        match res {
                            Ok(servers) => {
                                ListMessage::UpdateServerListComplete(servers)
                            },
                            Err(e) => {
                                ListMessage::UpdateServerListFail(e.to_string())
                            }
                        }
                    }
                );
            },
            ListMessage::UpdateServerListComplete(servers) => {
                self.rebuild_list(servers);
            },
            ListMessage::UpdateServerListFail(e) => {},
            ListMessage::RowMessage(id, row_message) => {
                match row_message {
                    RowMessage::Delete => {
                        self.rows.retain(|row| row.id != id);
                    }
                    RowMessage::ToggleSelection => {
                        match self.rows.iter_mut().filter(|row| row.id == id).next() {
                            Some(row) => {
                                self.selected = id
                            }, 
                            None => {}
                        }
                    },
                }
            }
            ListMessage::Nothing => {}
        }
        Command::none()
    }

    pub fn view(&mut self, heads: [&str;4]) -> Element<ListMessage> {
        let head = Row::with_children(heads.iter().map(|head| Text::new(*head).into()).collect());
        let scrollable = Scrollable::new(&mut self.scrollable)
            .push(
                Column::with_children(
                    self.rows.iter_mut().map(
                        |row| {
                            let id = row.id;
                            row.view(&self.selected).map(
                                move |row_message| ListMessage::RowMessage(id, row_message)
                            ).into()
                        }
                    ).collect()
                )
            );

        Column::new()
            .push(head)
            .push(scrollable)
            .height(Length::Fill)
            .into()
    }

    fn rebuild_list(&mut self, servers: Vec<Server>) {
        let mut id: u32 = 0;
        self.rows = servers.into_iter().map(|server| {
            id += 1;
            ServerRow::new(server, id, false)
        }).collect();
        self.rows.append(&mut 
            self.manual_import_list
                .iter()
                .map( |server| {
                    id += 1;
                    ServerRow::new(server.clone(), id, true)
                }
            ).collect()
        );
    }

    pub fn import(&mut self, mut servers: Vec<Server>) {
        self.manual_import_list.append(&mut servers);
        let mut id = self.rows.len() as u32;
        self.rows.append(&mut 
            servers.into_iter()
                .map( |server| {
                    id += 1;
                    ServerRow::new(server, id, true)
                }
            ).collect()
        )
    }
}