use crate::activites;
use crate::list_header::*;
use crate::message_list::MessageList;
use crate::preview::Preview;
use crate::sidebar::SideBar;
use dioxus::prelude::*;
use dioxus_desktop::{tao::platform::macos::WindowBuilderExtMacOS, use_window, WindowBuilder};
use dioxus_router::*;
use fermi::{use_atom_root, use_atom_state, use_init_atom_root, use_read, Atom};
use google_gmail1::api::{MessagePart, MessagePartHeader};

pub fn app(cx: Scope) -> Element {
    use_init_atom_root(cx);

    let root = use_atom_root(cx).clone();

    use_coroutine(cx, |handle| activites::main_loop(handle, root));

    cx.render(rsx! {
        div { class: "flex flex-row rounded-lg overflow-hidden border border-gray-200 dark:border-gray-800", id: "bodywrap",
            SideBar {}
            div { class: "flex-col flex-grow w-1/3",
                div { class: "flex-grow h-full",
                    MessageListHeader {}
                    MessageList {}
                }
            }
            Preview {}
        }
    })
}
