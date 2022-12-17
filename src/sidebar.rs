use dioxus::prelude::*;

pub fn SideBar(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "bg-gray-200 p-4 w-40 border-r border-gray-300", opacity: "0.98",
            // mimic some traffic lights
            div { class: "flex flex-row items-center py-2",
                div { class: "w-3 h-3 mx-2 rounded-full bg-red-500" }
                div { class: "w-3 h-3 mx-2 rounded-full bg-yellow-500" }
                div { class: "w-3 h-3 mx-2 rounded-full bg-green-500" }
            }
            h1 { "Sidebar" }
            ul { class: "list-disc truncate",
                li { "Inbox" }
                li { "Sent" }
                li { "Drafts" }
                li { "Trash" }
            }
        }
    })
}
