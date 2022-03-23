#![feature(never_type)]
use iced::{Application, Settings};
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
    LoaderMainInterface::run(Settings::default())?;
    Ok(())
    //crate::encrypt::test();
    //Ok(())
}

