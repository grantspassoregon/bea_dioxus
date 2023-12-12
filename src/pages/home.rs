#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;
use spreadsheet::prelude::*;

pub fn Home(cx: Scope) -> Element {
    let theme = use_shared_state::<Theme>(cx);
    let class = Theme::get(&theme, "background");
    cx.render(rsx!(HomeContent { class: class }))
}

#[derive(Props, PartialEq)]
struct HomeContentProps {
    #[props(into)]
    class: String,
}

fn HomeContent(cx: Scope<HomeContentProps>) -> Element {
    tracing::info!("Home page drawing.");
    let data = use_shared_state::<BeaData>(cx);
    // tracing::info!("data accessed: {:#?}.", &data.is_some());
    if let Some(d) = data {
        let mut times = d.read().time_period_keys();
        times.reverse();
        let codes = d.read().linecode_hash();
        let fips = d.read().geofips_hash();
        cx.render(rsx!(
        div {
            class: cx.props.class.as_str(),
            MenuItems {}
            QueryPanel { data: d.read(), times: times, codes: codes, fips: fips }
        }
      ))
    } else {
        cx.render(rsx!(
        div {
            class: cx.props.class.as_str(),
            MenuItems {}
            "Data is not loaded."
        }
        ))
    }
}
