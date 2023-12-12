#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;
use itertools::Itertools;
use spreadsheet::prelude::*;
use std::collections::HashMap;
use std::collections::hash_map::Keys;
use std::fmt::Display;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Props)]
pub struct QueryButtonProps<'a> {
    #[props(into)]
    msg: String,
    button: String,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn QueryButton<'a>(cx: Scope<'a, QueryButtonProps<'a>>) -> Element<'a> {
    tracing::trace!("Query button drawing.");
    cx.render(rsx!(button {
        class: cx.props.button.as_str(),
        onclick: move |event| cx.props.on_click.call(event),
        cx.props.msg.as_str()
    }))
}

#[inline_props]
pub fn QueryPanel<'a>(cx: Scope, data: Ref<'a, BeaData>, times: Vec<i32>, codes: HashMap<String, String>, fips: HashMap<i32, String>) -> Element {
    tracing::info!("Query panel drawing.");
    let year = use_state(cx, || HashMap::new());
    let code = use_state(cx, || HashMap::new());
    let geofips = use_state(cx, || HashMap::new());

    let see_year = use_state(cx, || true);
    let see_code = use_state(cx, || true);
    let see_fips = use_state(cx, || true);
    let results: &UseState<BeaData> = use_state(cx, || BeaData::from(Vec::new()));

    let field = use_state(cx, || HashMap::new());
    let field_2 = use_state(cx, || HashMap::new());
    let time_keys = use_state(cx, || Vec::new());
    tracing::info!("field vars created");

    let opts = vec!["Linecodes", "Geofips", "Time Period"];
    let selection = use_state(cx, || "".to_string());
    let theme = use_shared_state::<Theme>(cx);
    let button = Theme::get(&theme, "button");
    cx.render(rsx!(
    div {
        DropPanel {
            on_click: move |_| {
                tracing::info!("Button pressed.");
                tracing::info!("Selection is {}", selection.get());
                match selection.get().as_str() {
                    "Linecodes" => {
                        tracing::info!("Loading linecode keys.");
                        field.set(data.linecode_hash());
                        tracing::info!("Keys set to field.");
                    },
                    "Geofips" => {
                        tracing::info!("Loading geofips keys.");
                        field_2.set(data.geofips_hash());
                        tracing::info!("Keys set to field.");
                    },
                    "Time Period" => {
                        tracing::info!("Loading time periods.");
                        time_keys.set(data.time_period_keys());
                        tracing::info!("Keys set to field.");
                    },
                    _ => {
                        tracing::info!("No selection to load.");
                        tracing::info!("{:#?}", year.get().keys());
                        tracing::info!("{:#?}", code.get().keys());
                        tracing::info!("{:#?}", geofips.get().keys());
                        let res = search(data, year.get().keys(), code.get().keys(), geofips.get().keys());
                        results.set(res);

                    }
                }
            },
            options: opts,
            selection: selection,
        }
        div {
            input {
                class: "mx-3",
                r#type: "checkbox",
                name: "Year",
                oninput: move |_| see_year.set(!see_year.get()),
                value: "Year",
            }
            label {
                r#for: "Year",
                "Year"
            }
            input {
                class: "mx-3",
                r#type: "checkbox",
                name: "Code",
                oninput: move |_| see_code.set(!see_code.get()),
                value: "Code",
            }
            label {
                r#for: "Code",
                "Code"
            }
            input {
                class: "mx-3",
                r#type: "checkbox",
                name: "Fips",
                oninput: move |_| see_fips.set(!see_fips.get()),
                value: "Fips",
            }
            label {
                r#for: "Fips",
                "Fips"
            }
        }
        div {
            class: "flex flex-row divide-x divide-zinc-500 space-x-5",
            YearSelect { button: button.clone(), hide: *see_year.get(), options: &times,
                on_submit: move |evt: FormEvent| {
                    year.set(evt.data.values.clone());
                }, 
            }
            CodeSelect { button: button.clone(), hide: *see_code.get(), options: &codes,
                on_submit: move |evt: FormEvent| {
                    code.set(evt.data.values.clone());
                },
            }
            FipsSelect { button: button.clone(), hide: *see_fips.get(), options: &fips,
                on_submit: move |evt: FormEvent| {
                    geofips.set(evt.data.values.clone());
                },
            }
        }
        ResultWindow { results: results }
    }
        ))
}

#[derive(Props)]
pub struct QueryResultProps<'a, K, V> {
    #[props(into)]
    field: HashMap<K, V>,
    #[props(into)]
    phantom: PhantomData<&'a K>,
}

fn QueryResult<'a, K: Display + Clone + Ord + Hash, V: Display + Clone>(
    cx: Scope<'a, QueryResultProps<'a, K, V>>,
) -> Element<'a> {
    let keys = cx.props.field.keys().sorted();
    let hash = cx.props.field.clone();
    cx.render(rsx!(
    div {
        "Results"
    }
    div {
        class: "flex flex-col justify-center",
        for key in keys {
            div {
                format!("{}: {}", key, hash[&key])
            }
        }
    }
        ))
}

#[inline_props]
fn ResultWindow<'a>(cx: Scope<'a>, results: &'a BeaData) -> Element<'a> {
    if results.records_ref().is_empty() {
        cx.render(rsx!("Missing parameters for search."))
    } else {

    cx.render(rsx!(
        div {
            for result in results.records_ref() {
                p { format!("{:#?}", result) }
            }
        }
            ))
    }

}

fn search(data: &Ref<BeaData>, 
          years: Keys<String, Vec<String>>, 
          codes: Keys<String, Vec<String>>, 
          fips: Keys<String, Vec<String>>) -> BeaData {
    // let mut records = Vec::new();
    let mut rec = Vec::new();
    for year in years {
        tracing::info!("key is {}", year);
        rec.append(&mut data.filter("year", year).records());
    }
    let bea = BeaData::from(rec);
    let mut rec = Vec::new();
    for code in codes {
        tracing::info!("key is {}", code);
        rec.append(&mut bea.filter("code", code).records());
    }
    let bea = BeaData::from(rec);
    let mut rec = Vec::new();
    for place in fips {
        tracing::info!("key is {}", place);
        rec.append(&mut bea.filter("fips", place).records());
    }

    BeaData::from(rec)
}
