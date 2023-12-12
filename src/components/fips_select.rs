#![allow(non_snake_case)]
use dioxus::prelude::*;
use itertools::Itertools;
use std::collections::HashMap;

#[inline_props]
pub fn FipsSelect<'a>(
    cx: Scope, 
    button: String,
    hide: bool,
    options: &'a HashMap<i32, String>,
    on_submit: EventHandler<'a, FormEvent>,
    ) -> Element {
    let keys = options.keys().sorted();
    cx.render(rsx!(
        form {
        hidden: *hide,
        onsubmit: move |evt| on_submit.call(evt),
        p {
            class: "flex flex-row justify-center self-center mx-3",
            "Set FIPS"
        }
        p { 
            class: "flex flex-row justify-center self-center mx-3",
            input {
                class: button.as_str(),
                r#type: "submit",
            }
        }
            for key in keys {
                p {
                input {
                    class: "mx-3",
                    r#type: "checkbox",
                    value: options[key].as_str(),
                    name: *key as i64,
                }
                label {
                    r#for: options[key].as_str(),
                    format!("{}: {}", key, options[key])
                }
                }
            }
        }
            ))
}
