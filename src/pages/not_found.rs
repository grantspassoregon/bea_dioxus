#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

#[inline_props]
pub fn NotFound(cx: Scope, segments: Vec<String>) -> Element {
    let theme_ref = use_shared_state::<Theme>(cx);
    match theme_ref {
        Some(value) => {
            let theme = *value.read();
            let class = theme.background();
            cx.render(rsx!(NotFoundContent {
                class: class,
                segments: segments.clone(),
            }))
        }
        None => {
            let class = "max-w-full h-10 bg-blue-300".to_string();
            cx.render(rsx!(NotFoundContent {
                class: class,
                segments: segments.clone(),
            }))
        }
    }
}

#[derive(Props, PartialEq)]
struct NotFoundContentProps {
    #[props(into)]
    class: String,
    segments: Vec<String>,
}

fn NotFoundContent(cx: Scope<NotFoundContentProps>) -> Element {
    cx.render(rsx!(
    div {
        class: cx.props.class.as_str(),
        MenuItems {}
        // Footer { "Page content not found." }
        "Page content not found."
    }
      ))
}
