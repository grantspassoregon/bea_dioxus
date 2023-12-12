#![allow(non_snake_case)]
use bea_dioxus::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use spreadsheet::prelude::*;

fn main() -> BeaResult<()> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init()
    {};
    tracing::info!("Subscriber initialized.");
    dioxus_desktop::launch(App);
    tracing::info!("🚀 Client started successfully");
    Ok(())
}

fn App(cx: Scope) -> Element {
    let theme = use_shared_state::<Theme>(cx);
    if theme.is_none() {
        use_shared_state_provider(cx, || Theme::Light);
    }
    tracing::info!("data loading...");
    dotenv::dotenv().ok();
    let path = std::env::var("BEA_CAINC5N_CSV").unwrap();
    let records = BeaData::from_csv(path).unwrap();
    tracing::info!("Records: {}", records.records_ref().len());
    use_shared_state_provider(cx, || records.clone());
    tracing::info!("Data added to state.");
    cx.render(rsx! {
        style { include_str!("../public/tailwind.css") }
        Router::<Route> {}
    })
}
