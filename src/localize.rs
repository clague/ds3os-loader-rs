use std::{collections::HashMap, hash::Hash, sync::Mutex};

use crate::gui::FailReason;

use lazy_static::lazy_static;
use anyhow::{Result, anyhow};
use sys_locale::get_locale;

#[derive(Eq, PartialEq, Hash)]
pub enum Language {
    Auto,
    English,
    SChinese,
}

lazy_static! {
    static ref _FailReasonString: HashMap<Language, HashMap<FailReason, &'static str>> = HashMap::from([
        (Language::English, HashMap::from([
            (FailReason::ChooseFileFail, "Invalid file choosen!"),
            (FailReason::RefreshListFail, "Can't refresh the server list!"),
            (FailReason::PatchFail, "Exception happened during the patch"),
            (FailReason::ListNoSelected, "Please select a server first!"),
            (FailReason::ProcessNotFound, "Game process not found, maybe you need open the game first."),
        ])),
        (Language::SChinese, HashMap::from([
            (FailReason::ChooseFileFail, "无效的配置文件！"),
            (FailReason::RefreshListFail, "无法刷新服务器列表！"),
            (FailReason::PatchFail, "修改内存过程中发生错误"),
            (FailReason::ListNoSelected, "请先选择一个服务器"),
            (FailReason::ProcessNotFound, "未找到游戏进程，也许你应该先打开游戏。"),
        ])),
    ]);

    pub static ref FailReasonString: Mutex<&'static HashMap<FailReason, &'static str>> = Mutex::new(&_FailReasonString[&Language::English]);
}
pub fn set_language(mut lang: Language) -> Result<()> {
    if lang == Language::Auto {
        let lang_str = get_locale().unwrap_or("en-US".into());
        if lang_str.starts_with("zh") {
            lang = Language::SChinese;
        }
        else {
            lang = Language::English;
        }
    }
    match FailReasonString.try_lock() {
        Ok(mut lock) => {
            *lock = &_FailReasonString[&lang];
        },
        Err(e) => {
            return Err(anyhow!(e.to_string()));
        }
    }
    Ok(())
}

