use crate::state::*;
use dioxus::prelude::*;
use fermi::use_atom_state;
use google_gmail1::api::Message;

pub fn MessageList(cx: Scope) -> Element {
    let messages = use_atom_state(cx, MESSAGES);

    cx.render(rsx! {
        div { class: "h-full flex flex-col items-stretch bg-white user-select-none",
            div { class: "flex flex-row flex-auto min-h-0",
                div { class: "flex flex-col items-stretch min-h-0 overflow-x-hidden", style: "flex: 0 0 100%;",
                    div { class: "text-bold font-cx flex flex-row border-b text-gray-700 border-gray-200",
                        div { class: "flex-1 overflow-hidden ml-4", "From" }
                        div { class: "flex-1 overflow-hidden ml-4", "Snippet" }
                        div { class: "flex-1 overflow-hidden ml-4", "Sent" }
                        div { class: "flex-1 overflow-hidden ml-4", "Received" }
                    }
                    div { class: "flex-initial min-h-0 overflow-y-auto px-2 pt-2 text-sm",
                        (0..messages.len()).map(|idx| rsx! (
                            MessageListItem { key: "{idx}", idx: idx }
                        ))
                    }
                }
            }
        }
    })
}

#[inline_props]
pub fn MessageListItem(cx: Scope, idx: usize) -> Element {
    let messages = use_atom_state(cx, MESSAGES);
    let selected = use_atom_state(cx, SELECTED);

    let message = &messages[*idx];

    let is_selected = selected.as_ref().map(|s| s == idx).unwrap_or_default();
    let select_color = if is_selected { "bg-blue-400" } else { "" };

    let ParsedEmail {
        name,
        snippet,
        email,
        sent,
        received,
        ..
    } = use_name_and_email(cx, message);

    cx.render(rsx! {
        div {
            class: "text-bold font-sm overflow-hidden truncate flex flex-row cursor-default rounded {select_color}",
            onclick: move |_| selected.set(Some(*idx)),
            div { class: "flex-1 overflow-hidden ml-4 user-select-none", "{name}" }
            div { class: "flex-1 overflow-hidden ml-4 user-select-none", "{snippet}" }
            div { class: "flex-1 overflow-hidden ml-4 user-select-none", "{sent}" }
            div { class: "flex-1 overflow-hidden ml-4 user-select-none", "{received}" }
        }
    })
}

struct ParsedEmail {
    name: String,
    email: String,
    sent: String,
    received: String,
    snippet: String,
    labels: Vec<String>,
}

fn use_name_and_email<'a>(cx: &'a ScopeState, message: &Message) -> &'a ParsedEmail {
    cx.use_hook(|| parse_message(message))
}

fn parse_message(message: &Message) -> ParsedEmail {
    let make = || {
        let headers = message.payload.as_ref()?.headers.as_ref()?;
        let value = headers.iter().find(|h| h.name.as_deref() == Some("From"))?;
        let raw = value.value.as_ref().cloned();

        match parse_email_from(raw.as_deref()) {
            Some((name, email)) => {
                let mut out = String::new();
                html_escape::decode_html_entities_to_string(name, &mut out);

                Some((out.to_string(), email.to_string()))
            }
            None => Some((raw.unwrap_or_default(), "".to_string())),
        }
    };

    let (name, email) = make().unwrap_or_default();
    let raw = message.snippet.as_deref().unwrap_or_default();
    let mut snippet = String::new();
    html_escape::decode_html_entities_to_string(raw, &mut snippet);
    let sent = "Jan 12".to_string();
    let received = "July 12".to_string();

    ParsedEmail {
        email,
        name,
        snippet,
        received,
        sent,
        labels: vec![],
    }
}

fn parse_email_from(raw: Option<&str>) -> Option<(&str, &str)> {
    // split the email from the name
    raw.and_then(|s| s.split_once('<'))
        .map(|(from, email)| (from.trim(), email.trim_end_matches('>')))
}

#[test]
fn messages_parses() {
    //
}
