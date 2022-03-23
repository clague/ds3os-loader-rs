use crate::api::MasterServerApi;

use {
    crate::api::Server,
    crate::gui::FailReason,
    iced::{
        Column, Length, Row, Space, Text, Scrollable, scrollable,
        Command, Element, button, Button, Alignment, Radio
    },
};

pub struct ServerRow {
    pub id: usize,
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
    pub fn new(server: Server, id: usize, is_manual: bool) -> Self {
        Self {
            id,
            server,
            is_manual,
            server_btn: button::State::new(),
            delete_btn: button::State::new(),
        }
    }

    pub fn view(&mut self, selected: &usize) -> Element<RowMessage> {
        Row::new()
            .push(
                Button::new(
                    &mut self.server_btn,
                    Row::new()
                        .align_items(Alignment::Center)
                        .push(Text::new(&self.server.name).width(Length::FillPortion(1)))
                        .push(Text::new(&self.server.hostname).width(Length::FillPortion(1)))
                        .push(Text::new(&self.server.player_count.to_string()).width(Length::FillPortion(1)))
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
    pub selected: usize,
    pub manual_server_offset: usize,

    scrollable: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum ListMessage {
    //SearchInputChanged(String),
    UpdateServerList,
    UpdateServerListComplete(Vec<Server>),
    ImportConfig(Vec<Server>),
    Fail(FailReason, String),
    RowMessage(usize, RowMessage),
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
            selected: 0,
            manual_server_offset: 0,

            scrollable: scrollable::State::new(),
        }
    }

    pub fn update(&mut self, message: ListMessage, api: &MasterServerApi) -> Command<ListMessage> {
        match message {
            ListMessage::UpdateServerList => {
                let api = api.clone();
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
                                ListMessage::Fail(FailReason::RefreshListFail, e.to_string())
                            }
                        }
                    }
                );
            },
            ListMessage::UpdateServerListComplete(servers) => {
                self.rebuild_list(servers);
            },
            ListMessage::Fail(_, _) => {},
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
            ListMessage::ImportConfig(servers) => {
                self.import(servers);
            }
        }
        Command::none()
    }

    pub fn view(&mut self, heads: [&str;3]) -> Element<ListMessage> {
        let head = Row::with_children(heads.iter().map(|head| Text::new(*head).width(Length::FillPortion(1)).into()).collect());
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
            .width(Length::Fill)
            .into()
    }

    pub fn find_by_id(&self, id: usize) -> Option<&ServerRow> {
        self.rows.iter().filter(|row| row.id == id).next()
    }

    pub fn find_selected(&self) -> Option<&ServerRow> {
        self.find_by_id(self.selected)
    }

    pub fn find_by_id_mut(&mut self, id: usize) -> Option<&mut ServerRow> {
        self.rows.iter_mut().filter(|row| row.id == id).next()
    }

    pub fn find_selected_mut(&mut self) -> Option<&mut ServerRow> {
        self.find_by_id_mut(self.selected)
    }

    fn rebuild_list(&mut self, servers: Vec<Server>) {
        self.rows.retain(|row| row.is_manual);
        self.rows
            .append(&mut
                servers.into_iter()
                    .enumerate()
                    .map(|(id, server)| ServerRow::new(server, id + 2016, false))
                    .collect()
            );
    }

    pub fn import(&mut self, mut servers: Vec<Server>) {
        servers.retain(|server| {
            self.rows
                .iter()
                .find(|row| row.server.hostname == server.hostname)
                .is_none()
        });
        let offset = self.manual_server_offset;
        self.manual_server_offset += servers.len();
        self.rows.splice(0..0, 
            servers.into_iter()
                .enumerate()
                .map(|(id, server)| {
                    ServerRow::new(server, id + offset, true)
                })
            );
    }
}