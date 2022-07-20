use std::{collections::HashMap, hash::Hash};

use crate::gui::FailReason;

use lazy_static::lazy_static;
use anyhow::Result;
use sys_locale::get_locale;

#[derive(Eq, PartialEq, Hash)]
pub enum Language {
    Auto,
    English,
    SChinese,
}

lazy_static! {
    static ref FAIL_REASON_LOCALIZED_STRING_: HashMap<Language, HashMap<FailReason, &'static str>> = HashMap::from([
        (Language::English, HashMap::from([
            (FailReason::ChooseFileFail, "Invalid file choosen!"),
            (FailReason::RefreshListFail, "Can't refresh the server list!"),
            (FailReason::PatchFail, "Exception happened during the patch"),
            (FailReason::ListNoSelected, "Please select a server first!"),
            (FailReason::ProcessNotFound, "Game process not found, maybe you need open the game first."),
            (FailReason::FetchPublicKeyFail, "Can't fetch public key from the master server, most likely due to the incorrect password"),
        ])),
        (Language::SChinese, HashMap::from([
            (FailReason::ChooseFileFail, "无效的配置文件！"),
            (FailReason::RefreshListFail, "无法刷新服务器列表！"),
            (FailReason::PatchFail, "修改内存过程中发生错误"),
            (FailReason::ListNoSelected, "请先选择一个服务器"),
            (FailReason::ProcessNotFound, "未找到游戏进程，也许你应该先打开游戏。"),
            (FailReason::FetchPublicKeyFail, "从主服务器获取公钥失败，一般是由于密码错误"),
        ])),
    ]);

    static ref TEXT_LOCALIZED_STRING_: HashMap<Language, HashMap<TextType, &'static str>> = HashMap::from([
        (Language::English, HashMap::from([
            (TextType::PasswordRequired, "Need password"),
            (TextType::PasswordNotRequired, "No password"),
        ])),
        (Language::SChinese, HashMap::from([
            (TextType::PasswordRequired, "需要密码"),
            (TextType::PasswordNotRequired, "不需要密码"),
        ])),
    ]);

    pub static ref FAIL_REASON_LOCALIZED_STRING: &'static HashMap<FailReason, &'static str> = &FAIL_REASON_LOCALIZED_STRING_[&Language::English];

    pub static ref TEXT_LOCALIZED_STRING: &'static HashMap<TextType, &'static str> = &TEXT_LOCALIZED_STRING_[&Language::English];
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
    unsafe {
        type F = &'static HashMap<FailReason, &'static str>;
        type T = &'static HashMap<TextType, &'static str>;
        *(&(*FAIL_REASON_LOCALIZED_STRING) as *const F as *mut F) = &FAIL_REASON_LOCALIZED_STRING_[&lang];
        *(&(*TEXT_LOCALIZED_STRING) as *const T as *mut T) = &TEXT_LOCALIZED_STRING_[&lang];
    };
    Ok(())
}

#[derive(Eq, PartialEq, Hash)]
pub enum TextType {
    PasswordRequired,
    PasswordNotRequired,
}