#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;
use spreadsheet::prelude::*;

#[derive(Props)]
pub struct LoadButtonProps<'a> {
    button: String,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn LoadButton<'a>(cx: Scope<'a, LoadButtonProps<'a>>) -> Element<'a> {
    tracing::trace!("Load button drawing.");
    cx.render(rsx!(button {
        class: cx.props.button.as_str(),
        onclick: move |event| cx.props.on_click.call(event),
        "Load Data"
    }))
}

pub fn Load(cx: Scope) -> Element {
    tracing::trace!("Load screen drawing.");
    let theme = use_shared_state::<Theme>(cx);
    let button = Theme::get(&theme, "button");
    let msg = use_state(cx, || "No data".to_string());
    cx.render(rsx!(
    div {
        class: "flex flex-row justify-center space-x-4",
    div {
        LoadButton {
            button: button,
            on_click: move |event| {
                tracing::trace!("{:#?}", &event);
                msg.set("Loading...".to_string());
                dotenv::dotenv().ok();
                let path = std::env::var("BEA_CAINC5N_CSV").unwrap();
                let records = BeaData::from_csv(path).unwrap();
                tracing::trace!("Records: {}", records.records_ref().len());
                use_shared_state_provider(cx, || records);
                msg.set("Loaded".to_string());
                tracing::info!("Records loaded.");
            }
        }
    }
    div {
        class: "my-1",
        format!("Status: {}", msg)
    }
    }
        ))
}
