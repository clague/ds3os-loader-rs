use iced::{button, Button, Command, Element, Length, Text, Alignment, Column, Scrollable, scrollable, TextInput, text_input};
use iced_aw::FloatingButton;
use iced_native::{text, Widget};

use crate::api::Server;
use crate::gui::{ICON_FONT, Icon};
use crate::localize::{TextLocalizedString, TextType::*};
pub struct DetailPanel {
    srcollable: scrollable::State,
    patch_btn: button::State,
    passwd_input: text_input::State,
}

impl DetailPanel {
    pub fn new() -> Self{
        Self {
            srcollable: scrollable::State::new(),
            patch_btn: button::State::new(),
            passwd_input: text_input::State::new(),
        }
    }

    pub fn update(&mut self, message: !) -> Command<!> {
        Command::none()
    }

    pub fn view(&mut self, server: Server, passwd: &str) -> Element<crate::gui::Message> {
        let name_text = Text::new(&format!("{}: {}", "Name", server.name));

        let hostname_text = Text::new(&format!("{}: {}", "Hostname", server.hostname));

        let private_hostname_text = Text::new(&format!("{}: {}", "Private Hostname", server.private_hostname));

        let player_count_text = Text::new(&format!("{}: {}", "Player Count", server.player_count));

        let password_required_text = Text::new(&format!("{}: {}", "Password: ", 
            if server.password_required {
                TextLocalizedString[&PasswordRequired]
            }
            else {
                TextLocalizedString[&PasswordNotRequired]
            }
        ));

        let description_text = Text::new(&format!("{}: {}", "Description", server.description));

        let col = Column::new()
            .push(name_text)
            .push(hostname_text)
            .push(private_hostname_text)
            .push(player_count_text)
            .push(password_required_text)
            .push(description_text)
            .spacing(10)
            .align_items(Alignment::Start);

        let scrollable = Scrollable::new(&mut self.srcollable)
            .push(col)
            .height(Length::Fill)
            .width(Length::Fill);

        let passwd_input = TextInput::new(&mut self.passwd_input,
            "Password",
            passwd,
            |s| crate::gui::Message::PasswordInput(s)
        ).size(32);

        let underlay = Column::new()
            .push(scrollable)
            .push(passwd_input);

        FloatingButton::new(&mut self.patch_btn, underlay, |state| {
                Button::new(
                    state,
                    Text::new(Icon::PlayLight)
                        .width(Length::Shrink)
                        .height(Length::Shrink)
                        .font(ICON_FONT)
                        .size(39),
                )
                //.style(iced_aw::style::button::Primary),
                .on_press(crate::gui::Message::Patch)
                .padding(5)
            }).into()
    }
}
