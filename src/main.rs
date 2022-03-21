use iced::{Application, Settings};
use lazy_static::lazy_static;

mod gui;
mod api;
mod launch;
mod localize;
mod table;
mod encrypt;

use crate::gui::LoaderMainInterface;

lazy_static! {
    pub static ref MASTER_SERVER_ADDR_DEF: &'static str = "http://timleonard.uk:50020";
}

fn main() -> iced::Result {
    LoaderMainInterface::run(Settings::default())
    //crate::encrypt::test();
    //Ok(())
}

