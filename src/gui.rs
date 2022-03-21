use iced::{Application, executor, Command, Row, scrollable, button, Button, Text, Column, Alignment, Length};

use crate::api::{Server, MasterServerApi};
use crate::launch::Launcher;
use crate::table::{ServerList, ListMessage};

pub struct LoaderMainInterface {
    // The counter value
    launcher: Launcher,

    // The local state of the two buttons
    update: button::State,
    launch: button::State,
    list: ServerList,
}

#[derive(Debug, Clone)]
pub enum Message {
    ListMessage(ListMessage),
    Launch,
}


impl Application for LoaderMainInterface {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let api = MasterServerApi::new(*crate::MASTER_SERVER_ADDR_DEF, 1).unwrap();
        (
            LoaderMainInterface{
                launcher: Launcher::new(),
                update: button::State::new(),
                launch: button::State::new(),
                list: ServerList::new(),
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
                        if let Some(row) = self.list.rows.get((self.list.selected - 1) as usize) {
                            let api = self.list.api.clone();
                            let server = row.server.clone();
                            Command::perform(Launcher::patch_game(pid, api, server, ""), 
                                |r| {
                                    match r {
                                        Ok(_) => {
                                        },
                                        Err(e) => {
                                            println!("{}", e);
                                        }
                                    }
                                    Message::ListMessage(ListMessage::Nothing)
                                })
                        }
                        else {
                            Command::none()
                        }
                    }, 
                    Err(e) => {
                        println!("{}", e);
                        Command::none()
                    }
                }
                
            },
            Message::ListMessage(m) => {
                self.list.update(m).map(|m| Message::ListMessage(m))
            },
        }
    }
    
    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let update_button = Button::new(&mut self.update, Text::new("Update"))
            .on_press(Message::ListMessage(ListMessage::UpdateServerList));
        let launch_button = Button::new(&mut self.launch, Text::new("Launch"))
            .on_press(Message::Launch);
        let button_row = Row::with_children(vec![update_button.into(), launch_button.into()]).spacing(60).align_items(Alignment::Center).height(Length::Units(100));

        let heads = ["Name", "Address", "Player Count", "Description"];
        let table = self.list.view(heads).map(|list_message| Message::ListMessage(list_message));

        Column::with_children(vec![table.into(), button_row.into()]).into()
    }
}
