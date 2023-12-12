#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

#[derive(Props)]
pub struct DropDownItemProps<'a> {
    #[props(into)]
    style: String,
    on_select: EventHandler<'a, SelectionEvent>,
    #[props(into)]
    selection: &'a UseState<String>,
    #[props(into)]
    message: String,
}

fn DropDownItem<'a>(cx: Scope<'a, DropDownItemProps<'a>>) -> Element {
    cx.render(rsx!(
    option {
        class: cx.props.style.as_str(),
        value: cx.props.message.as_str(),
        oncut: move |_| tracing::info!("on cut"),
        onblur: move |_| tracing::info!("on blur"),
        oncopy: move |_| tracing::info!("on copy"),
        ondrag: move |_| tracing::info!("on drag"),
        ondrop: move |_| tracing::info!("on drop"),
        onload: move |_| tracing::info!("on load"),
        onplay: move |_| tracing::info!("on play"),
        onabort: move |_| tracing::info!("on abort"),
        onclick: move |_| tracing::info!("on click"),
        onended: move |_| tracing::info!("on ended"),
        onerror: move |_| tracing::info!("on error"),
        onfocus: move |_| tracing::info!("on focus"),
        oninput: move |_| tracing::info!("on input"),
        onkeyup: move |_| tracing::info!("on keyup"),
        onpaste: move |_| tracing::info!("on paste"),
        onpause: move |_| tracing::info!("on pause"),
        cx.props.message.clone()
    }
            ))

}

#[derive(Props)]
pub struct DropDownInnerProps<'a> {
    #[props(into)]
    style: String,
    on_input: EventHandler<'a, FormEvent>,
    options: Vec<&'a str>,
}

pub fn DropDownInner<'a>(cx: Scope<'a, DropDownInnerProps<'a>>) -> Element {
    cx.render(rsx!(
        div { "Select" }
        select {
            class: cx.props.style.as_str(),
            name: "Ted",
            oninput: move |event| cx.props.on_input.call(event),
            for opt in cx.props.options.clone() {
                option {
                    value: opt,
                    opt
                }
            }
        }
            ))
}

#[inline_props]
pub fn DropDown<'a>(
    cx: Scope, 
    selection: &'a UseState<String>,
    options: Vec<&'a str>,
    ) -> Element {
    let theme = use_shared_state::<Theme>(cx);
    let style = Theme::get(&theme, "dropdown");
    cx.render(rsx!(
        DropDownInner { 
            style: style,
            on_input: move |event: FormEvent| {
                selection.set(event.inner().value.clone());
            },
            options: options.clone(),
        }
            ))
}

#[derive(Props)]
pub struct ButtonProps<'a> {
    style: String,
    on_click: EventHandler<'a, MouseEvent>,
    msg: String,
}

fn DropDownButton<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    cx.render(rsx!(
        button {
            class: cx.props.style.as_str(),
            onclick: move |event| cx.props.on_click.call(event),
            cx.props.msg.clone()
        }
            ))

}

#[inline_props]
pub fn DropPanel<'a>(cx: Scope, on_click: EventHandler<'a, MouseEvent>, options: Vec<&'a str>, selection: &'a UseState<String>) -> Element {
    let theme = use_shared_state::<Theme>(cx);
    let style = Theme::get(&theme, "button");
    cx.render(rsx!(
        div {
            DropDown { selection: selection, options: options.clone() }
            DropDownButton {
                style: style,
                on_click: move |evt| on_click.call(evt),
                msg: "Select".to_string(),
            }
        }
            ))
}
