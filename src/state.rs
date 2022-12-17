// todo: incorporate more email types here
use fermi::Atom;
use std::fs;

use google_gmail1::api::Message;

pub static SELECTED: Atom<Option<usize>> = |_| None;

pub static MESSAGES: Atom<Vec<Message>> =
    |_| match fs::read_to_string("data/sensitive/index.json").map(|s| serde_json::from_str(&s)) {
        Ok(Ok(index)) => index,
        _ => Vec::default(),
    };

pub static TOOLBAR_CFG: Atom<ToolbarCfg> = |_| ToolbarCfg {};

pub struct ToolbarCfg {}

pub struct IndexedMessage {
    pub message: Message,
    pub index: usize,
    pub name: String,
    pub email: String,
    pub snippet: String,
}
