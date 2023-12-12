#![allow(non_snake_case)]
use dioxus::prelude::*;

#[inline_props]
pub fn YearSelect<'a>(
    cx: Scope, 
    button: String,
    hide: bool,
    options: &'a Vec<i32>,
    on_submit: EventHandler<'a, FormEvent>,
    ) -> Element {
    cx.render(rsx!(
        form {
        hidden: *hide,
        onsubmit: move |evt| on_submit.call(evt),
        p {
            class: "flex flex-row justify-center self-center mx-3",
            "Set Year"
        }
        p { 
            class: "flex flex-row justify-center self-center mx-3",
            input {
                class: button.as_str(),
                r#type: "submit",
            }
        }
            for opt in options {
                p {
                input {
                    class: "mx-3",
                    r#type: "checkbox",
                    value: *opt as i64,
                    name: *opt as i64,
                    // oninput: move |event| on_input.call(event),
                }
                label {
                    r#for: *opt as i64,
                    format!("{}", opt)
                }
                }
            }
        }
            ))
}
