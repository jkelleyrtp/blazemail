use dioxus::prelude::*;
use dioxus_desktop::use_window;

pub fn MessageListHeader(cx: Scope) -> Element {
    let window = use_window(cx);

    cx.render(rsx! {
        div {
            class: "p-2 bg-gray-100 border-b bg-gray-200 border-gray-200 flex flex-row justify-between items-center h-12 cursor-default",
            onmousedown: move |_| window.drag(),

            // Helpful display info on the left of the row
            div { class: "flex flex-col",
                h1 { class: "font-bold text-sm text-gray-800", "Inbox - Google " }
                h3 { class: "text-xs text-gray-500", "2,5438 messages, 100 unread" }
            }

            // Filters for Primary, Social, Promotions, Updates, Forums
            FilterGroup {}
        }
    })
}

pub fn FilterGroup(cx: Scope) -> Element {
    let filters = &[
        ("Primary", "primary"),
        ("Social", "social"),
        ("Promotions", "promotions"),
        ("Updates", "updates"),
        ("Forums", "forums"),
    ];

    cx.render(rsx! {
        ul { class: "flex flex-row",
            filters.iter().map(|(name, id)| rsx!(
                li { class: "flex flex-1 mx-1 text-xs",
                    input { class: "hidden peer", id: "filter-{id}", r#type: "radio", name: "hosting",  value: "filter-{id}", }
                    label {
                        class: "p-1 text-gray-500 bg-white rounded-lg border border-gray-200 cursor-pointer dark:hover:text-gray-300 dark:border-gray-700 dark:peer-checked:text-blue-500 peer-checked:border-blue-600 peer-checked:text-blue-600 hover:text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:bg-gray-800 dark:hover:bg-gray-700",
                        r#for: "filter-{id}",
                        div { class: "block", "{name}" }
                    }
                }
            ))
        }
    })
}
