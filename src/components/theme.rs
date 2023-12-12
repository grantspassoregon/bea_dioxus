#![allow(non_snake_case)]
use dioxus::prelude::*;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn next(&self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }

    pub fn get(theme: &Option<&UseSharedState<Theme>>, aspect: &str) -> String {
        tracing::trace!("Getting theme.");
        let mut class = "".to_string();
        match theme {
            Some(value) => {
                let theme = *value.read();
                match aspect {
                    "background" => class.push_str(&theme.background()),
                    "button" => class.push_str(&theme.button()),
                    "center" => class.push_str(&theme.center()),
                    "dropdown" => class.push_str(&theme.dropdown()),
                    "input" => class.push_str(&theme.input()),
                    _ => {}
                }
                tracing::trace!("Theme set.");
            }
            None => {
                tracing::trace!("No theme found.");
            }
        }
        class
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Light => write!(f, "💡"),
            Self::Dark => write!(f, "🌙"),
        }
    }
}

pub trait Aspect {
    fn background(&self) -> String;
    fn button(&self) -> String;
    fn center(&self) -> String;
    fn dropdown(&self) -> String;
    fn input(&self) -> String;
}

impl Aspect for Theme {
    fn background(&self) -> String {
        match self {
            Self::Light => {
                "min-h-screen max-w-full bg-gradient-to-r from-slate-300 to-slate-400 text-zinc-800 divide-y divide-zinc-500".to_string()
            }
            Self::Dark => {
                "min-h-screen max-w-full bg-gradient-to-r from-zinc-700 via-zinc-900 to-zinc-800 text-slate-100 divide-y divide-zinc-500".to_string()
            }
        }
    }

    fn button(&self) -> String {
        match self {
            Self::Light => {
                "my-1 px-2 bg-gradient-to-r from-blue-300 via-blue-200 to-blue-300 rounded-full shadow shadow-blue-300 hover:from-blue-400 hover:via-blue-300 hover:to-blue-400 active:from-indigo-300 active:via-indigo-200 active:to-indigo-300"
                    .to_string()
            }
            Self::Dark => {
                "my-1 bg-gradient-to-r from-slate-300 via-slate-100 to-slate-300 px-2 rounded-full text-zinc-900 shadow shadow-slate-300 hover:from-slate-400 hover:via-slate-200 hover:to-slate-400 active:from-indigo-300 active:via-indigo-200 active:to-indigo-300"
                    .to_string()
            }
        }
    }

    fn center(&self) -> String {
        "p-3 flex flex-row justify-center".to_string()
    }

    fn dropdown(&self) -> String {
        match self {
            Self::Light => "bg-slate-300 p-1 m-3".to_string(),
            Self::Dark => "bg-zinc-600 p-1 m-3".to_string(),
        }

    }

    fn input(&self) -> String {
        match self {
            Self::Light => "mx-3 px-2 bg-slate-300".to_string(),
            Self::Dark => "mx-3 px-2 bg-zinc-600".to_string(),
        }
    }
}
