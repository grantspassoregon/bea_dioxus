#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Props, PartialEq)]
pub struct HomeTileProps<'a> {
    #[props(default = "/home")]
    href: &'a str,
    #[props(default = "GP Spotlight")]
    title: &'a str,
}

pub fn HomeTile<'a>(cx: Scope<'a, HomeTileProps<'a>>) -> Element {
    cx.render(rsx!(
    Link { class: "bg-slate-300 p-2 m-3 rounded-full border-2 border-slate-100 shadow text-blue-800", to: Route::Home {}, cx.props.title }
                  ))
}
