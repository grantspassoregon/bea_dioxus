#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

#[inline_props]
pub fn Loading(cx: Scope) -> Element {
    let theme_ref = use_shared_state::<Theme>(cx);
    match theme_ref {
        Some(value) => {
            let theme = *value.read();
            let class = theme.background();
            cx.render(rsx!(LoadingContent {
                class: class,
            }))
        }
        None => {
            let class = "max-w-full h-10 bg-blue-300".to_string();
            cx.render(rsx!(LoadingContent {
                class: class,
            }))
        }
    }
}

#[derive(Props, PartialEq)]
struct LoadingContentProps {
    #[props(into)]
    class: String,
}

fn LoadingContent(cx: Scope<LoadingContentProps>) -> Element {
    cx.render(rsx!(
    div {
        class: cx.props.class.as_str(),
        // Footer { "Page content not found." }
        "Loading..."
    }
      ))
}
