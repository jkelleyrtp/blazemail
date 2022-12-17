#![allow(non_snake_case)]

use dioxus_desktop::{tao::platform::macos::WindowBuilderExtMacOS, LogicalSize, WindowBuilder};

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
    dioxus_desktop::launch_cfg(app::app, make_config());
}

fn make_config() -> dioxus_desktop::Config {
    dioxus_desktop::Config::default()
        .with_window(make_window())
        .with_custom_head(
            r#"
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
"#
            .to_owned(),
        )
}

fn make_window() -> WindowBuilder {
    WindowBuilder::new()
        .with_has_shadow(true)
        .with_transparent(true)
        .with_titlebar_buttons_hidden(true)
        .with_title_hidden(true)
        .with_titlebar_hidden(true)
        .with_min_inner_size(LogicalSize::new(600, 800))
}
