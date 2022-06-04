#![windows_subsystem = "windows"]
use std::fs::File;
use std::io::Read;

use iced::{Application, Settings, window};
use anyhow::Result;

mod gui;
mod api;
mod patch;
mod localize;
mod encrypt;
mod widgets;

use crate::gui::LoaderMainInterface;
use crate::localize::Language;

fn main() -> Result<()> {
    localize::set_language(Language::Auto)?;

    let setting = Settings {
        id: None,
        window: window::Settings {
            size: (800, 600),
            position: window::Position::Default, 
            min_size: None,
            max_size: None,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: None,
        },
        flags: (),
        default_font: {
            if cfg!(windows) {
                let mut buffer = Vec::new();
                File::open(r#"C:\Windows\Fonts\simhei.ttf"#)?.read_to_end(&mut buffer)?;
                Some(Box::leak(buffer.into_boxed_slice()))
            }
            else {
                None
            }
        },
        default_text_size: 16,
        text_multithreading: true,
        exit_on_close_request: true,
        antialiasing: false,
        try_opengles_first: false,
    };

    LoaderMainInterface::run(setting)?;
    Ok(())
    //crate::encrypt::test();
    //Ok(())
}
