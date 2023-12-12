use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn MenuItems(cx: Scope) -> Element {
    tracing::trace!("Main body drawing.");
    cx.render(rsx!(
            menu {
                class: "flex flex-row justify-between sticky top-0",
                HomeTile {}
                Link { class: "self-center m-6", to: Route::Home {}, "Home" }
                DarkModeButton {
                    on_click: move |event| {
                        tracing::trace!("{:?}", &event);
                        let theme = use_shared_state::<Theme>(cx);
                        let mut msg = "".to_string();
                        match theme {
                            Some(value) => {
                                let t = value.read().clone();
                                *value.write() = t.next();
                            }
                            None => msg.push_str("No theme found."),
                        }
                    },
                }
    }))
}
