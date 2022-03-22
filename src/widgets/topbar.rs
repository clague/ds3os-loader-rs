use iced::{button, Button, Command, Element, Length, Text, Row, Alignment};

pub struct TopBar {
    refresh_btn: button::State,
    import_btn: button::State,

    about_btn: button::State,
}

#[derive(Debug, Clone)]
pub enum TopBarMessage {
    RefreshServerList,
    ChooseConfigFile,
    ShowAbout,
}

impl TopBar {
    pub fn new() -> Self{
        Self {
            refresh_btn: button::State::new(),
            import_btn: button::State::new(),
            about_btn: button::State::new(),
        }
    }

    pub fn update(&mut self, message: TopBarMessage) -> Command<TopBarMessage> {
        match message {
            TopBarMessage::ChooseConfigFile => {
                
            },
            TopBarMessage::ShowAbout => todo!(),
            _ => {},
        }
        Command::none()
    }

    pub fn view(&mut self) -> Element<TopBarMessage> {
        let refresh_btn = Button::new(
            &mut self.refresh_btn,
            Text::new("\u{E800}").font(crate::gui::ICON_FONT)
        )
            .height(Length::Units(50))
            .width(Length::Units(50))
            .on_press(TopBarMessage::RefreshServerList);
        let import_btn = Button::new(
            &mut self.import_btn,
            Text::new("\u{E804}").font(crate::gui::ICON_FONT)
        )
            .height(Length::Units(50))
            .width(Length::Units(50))
            .on_press(TopBarMessage::ChooseConfigFile);
        let about_btn = Button::new(
            &mut self.about_btn,
            Text::new("About")
        )
            .height(Length::Units(50))
            .width(Length::Units(50))
            .on_press(TopBarMessage::ShowAbout);

        Row::new()
            .push(refresh_btn)
            .push(import_btn)
            .push(iced::Space::with_width(Length::Fill))
            .push(about_btn)
            .spacing(10)
            .align_items(Alignment::Center)
            .height(Length::Units(100))
            .into()
    }
}
