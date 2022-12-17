// todo: incorporate more email types here
use fermi::Atom;
use std::fs;

mod account;
mod index;
mod message;
mod toolbar;

use self::{
    account::{AccountProvider, LoggedInAccount},
    toolbar::ToolbarCfg,
};
use google_gmail1::api::Message;

pub static SELECTED: Atom<Option<usize>> = |_| None;
pub static MESSAGES: Atom<Vec<Message>> =
    |_| match fs::read_to_string("data/sensitive/index.json").map(|s| serde_json::from_str(&s)) {
        Ok(Ok(index)) => index,
        _ => Vec::default(),
    };

pub static TOOLBAR_CFG: Atom<ToolbarCfg> = |_| ToolbarCfg {};
pub static ACCOUNTS: Atom<Vec<LoggedInAccount>> = |_| {
    vec![
        LoggedInAccount {
            name: "Jon Kelley".into(),
            email: "jkelleyrtp@gmail.com".into(),
            avatar: "https://avatars.githubusercontent.com/u/1024544?v=4".into(),
            provider: AccountProvider::Gmail,
        },
        LoggedInAccount {
            name: "Jon Kelley".into(),
            email: "jkelleyrtp@gmail.com".into(),
            avatar: "https://avatars.githubusercontent.com/u/1024544?v=4".into(),
            provider: AccountProvider::Gmail,
        },
    ]
};

pub static SIMILAR_MESSAGES: Atom<Vec<usize>> = |_| vec![];
