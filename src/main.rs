use iced::{Application, Settings};
use lazy_static::lazy_static;
use anyhow::Result;

mod gui;
mod api;
mod launch;
mod localize;
mod table;
mod encrypt;
mod widgets;

use crate::gui::LoaderMainInterface;
use crate::localize::Language;

lazy_static! {
    pub static ref MASTER_SERVER_ADDR_DEF: &'static str = "http://timleonard.uk:50020";
}

fn main() -> Result<()> {
    localize::set_language(Language::Auto)?;
    LoaderMainInterface::run(Settings::default())?;
    Ok(())
    //crate::encrypt::test();
    //Ok(())
}

