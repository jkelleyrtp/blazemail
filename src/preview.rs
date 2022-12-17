use dioxus::prelude::*;
use dioxus_desktop::use_window;
use fermi::use_read;
use google_gmail1::api::{MessagePart, MessagePartHeader};

use crate::state::*;

pub fn Preview(cx: Scope) -> Element {
    let selected = use_read(cx, SELECTED);

    cx.render(rsx! (
        div { class: "flex flex-col bg-white flex-grow w-1/2 border-l border-gray-200",
            Toolbar {}
            match *selected {
                Some(idx) => rsx!( RenderMessage { idx: idx } ),
                None => rsx!(div { class: "m-auto", "no message selected" }),
            }
        }
    ))
}

fn Toolbar(cx: Scope) -> Element {
    let window = use_window(cx);
    let toolbar_cfg = use_read(cx, TOOLBAR_CFG);

    cx.render(rsx! {
        div {
            class: "flex bg-gray-100 border-b border-gray-200 h-12",
            onmousedown: move |_| window.drag(),

            div {
                button {}
            }
            div {
                button {}
                button {}
                button {}
                button {}
                button {}
                button {}
            }
        },
    })
}

#[inline_props]
fn RenderMessage(cx: Scope<'a>, idx: usize) -> Element {
    let messages = use_read(cx, MESSAGES);
    let message = messages.get(*idx).unwrap();

    let payload = message.payload.as_ref().unwrap();
    let headers = payload.headers.as_ref().unwrap();
    let parts = payload.parts.as_ref();

    let from = extract_from(headers).unwrap_or_default();
    let to = extract_to(headers).unwrap_or_default();

    let body = parts
        .and_then(|parts| {
            decode_first_mime(parts, "text/html").or_else(|| decode_first_mime(parts, "text/plain"))
        })
        .unwrap_or_else(|| "no body".to_string());

    cx.render(rsx! {
        div { class: "p-4 bg-white h-full overflow-y-auto",
            div { class: "p-4 bg-white rounded-lg border border-gray-200 shadow-2xl",
                // from, to, subject, date
                h1 {}
                h2 { "{from}" }
                h2 { "{to}" }

                // body
                iframe {
                    class: "w-full h-full",
                    "onload": "this.style.height=(this.contentWindow.document.body.scrollHeight+20)+'px';",
                    "sandbox": "allow-same-origin",
                    srcdoc: "{body}"
                }
            }
        }
    })
}

fn extract_to(headers: &[MessagePartHeader]) -> Option<&str> {
    headers
        .iter()
        .find(|h| h.name.as_deref() == Some("To") || h.name.as_deref() == Some("to"))?
        .value
        .as_deref()
}
fn extract_from(headers: &[MessagePartHeader]) -> Option<&str> {
    headers
        .iter()
        .find(|h| h.name.as_deref() == Some("From") || h.name.as_deref() == Some("from"))?
        .value
        .as_deref()
}

fn decode_first_mime(parts: &[MessagePart], mime: &str) -> Option<String> {
    parts
        .iter()
        .find(|p| p.mime_type.as_deref() == Some(mime))
        .and_then(|body| {
            body.body.as_ref().map(|body| {
                let bytes =
                    base64::decode_config(body.data.as_ref().unwrap().as_bytes(), base64::URL_SAFE)
                        .unwrap();
                let body = String::from_utf8_lossy(&bytes);

                body.to_string()
            })
        })
}
