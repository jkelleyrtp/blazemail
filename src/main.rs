#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_desktop::{tao::platform::macos::WindowBuilderExtMacOS, use_window, WindowBuilder};
use fermi::{use_atom_root, use_atom_state, use_init_atom_root, use_read, Atom};
use google_gmail1::api::{MessagePart, MessagePartHeader};

mod activites;
mod app;
mod list_header;
mod mail;
mod message;
mod message_list;
mod preview;
mod sidebar;
mod state;

fn main() {
    // include tailwind from cdn
    static CUSTOM_HEAD: &str = r#"
    <script src="https://cdn.tailwindcss.com"></script>
    <style type="text/css">
        html, body {
            height: 100%;
            margin: 0;
            overscroll-behavior-y: none;
            overscroll-behavior-x: none;
            overflow: hidden;
        }
        #main, #bodywrap {
            height: 100%;
            margin: 0;
            overscroll-behavior-x: none;
            overscroll-behavior-y: none;
        }
    </style>
"#;

    dioxus_desktop::launch_cfg(
        app::app,
        dioxus_desktop::Config::default()
            .with_custom_head(CUSTOM_HEAD.into())
            .with_window(
                WindowBuilder::new()
                    .with_has_shadow(true)
                    .with_transparent(true)
                    .with_titlebar_buttons_hidden(true)
                    .with_title_hidden(true)
                    .with_titlebar_hidden(true)
                    .with_maximized(true),
            ),
    );
}
