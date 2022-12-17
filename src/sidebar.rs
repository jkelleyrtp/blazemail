use dioxus::prelude::*;
use dioxus_desktop::use_window;
use fermi::use_read;

use crate::state::ACCOUNTS;

pub fn SideBar(cx: Scope) -> Element {
    let desktop = use_window(cx);
    let accounts = use_read(cx, ACCOUNTS);

    cx.render(rsx! {
        div { class: "bg-gray-200 dark:bg-zinc-800 w-40 border-r border-gray-300", opacity: "0.98",
            // mimic some traffic lights
            TrafficLights {}

            div { class: "pl-4",
                SideBarGroup { name: "Favorites"
                    Entry { icon: SidebarIcon::Inbox, name: "Inbox" }
                    Entry { icon: SidebarIcon::Inbox, name: "Inbox" }
                    Entry { icon: SidebarIcon::Inbox, name: "Inbox" }
                    Entry { icon: SidebarIcon::Inbox, name: "Inbox" }
                }

                SideBarGroup { name: "Analytics"
                    Entry { icon: SidebarIcon::Gear, name: "Inbox" }
                    Entry { icon: SidebarIcon::Gear, name: "Inbox" }
                    Entry { icon: SidebarIcon::Gear, name: "Inbox" }
                }

                SideBarGroup { name: "Smart Mailboxes"
                    Entry { icon: SidebarIcon::Gear, name: "Inbox" }
                }

                SideBarGroup { name: "Google"
                    Entry { icon: SidebarIcon::Inbox, name: "Inbox" }
                    Entry { icon: SidebarIcon::Inbox, name: "Inbox" }
                    Entry { icon: SidebarIcon::Inbox, name: "Inbox" }
                    Entry { icon: SidebarIcon::Inbox, name: "Inbox" }
                    Entry { icon: SidebarIcon::Inbox, name: "Inbox" }
                    Entry { icon: SidebarIcon::Inbox, name: "Inbox" }
                    Entry { icon: SidebarIcon::Inbox, name: "Inbox" }
                    Entry { icon: SidebarIcon::Inbox, name: "Inbox" }
                }
            }
        }
    })
}

#[inline_props]
pub fn SideBarGroup<'a>(cx: Scope<'a>, name: &'static str, children: Element<'a>) -> Element {
    cx.render(rsx! {
        div { class: "pb-8 text-xs",
            h3 { class: "text-gray-500 tracking-wide", "{name}" }
            ul { class: "mt-2 space-y-2 dark:text-white", children }
        }
    })
}

#[derive(PartialEq)]
enum SidebarIcon {
    Inbox,
    Gear,
    Folder,
    Flagged,
    Draft,
    Snoozed,
    Sent,
    Trash,
}

#[inline_props]
fn Entry(cx: Scope, icon: SidebarIcon, name: &'static str) -> Element {
    cx.render(rsx! {
        li { class: "flex flex-row items-center",
            span { class: "ml-2", "{name}" }
        }
    })
}

fn TrafficLights(cx: Scope) -> Element {
    let desktop = use_window(cx);

    cx.render(rsx! {
        div { class: "flex flex-row items-center p-2 w-full py-4", onmousedown: move |_| desktop.drag(),
            button {
                class: "w-3 h-3 mx-1 rounded-full bg-red-500",
                onclick: move |_| desktop.close(),
            }
            button { class: "w-3 h-3 mx-1 rounded-full bg-yellow-500" }
            button { class: "w-3 h-3 mx-1 rounded-full bg-green-500" }
        }
    })
}
