mod components;
mod error;
mod pages;
mod query;

pub mod prelude {
    pub use crate::components::{Aspect, DarkModeButton, HomeTile, Load, MenuItems, Theme, DropDown, DropPanel, YearSelect, CodeSelect, FipsSelect};
    pub use crate::error::{BeaError, BeaResult};
    pub use crate::pages::{Home, NotFound, Route};
    pub use crate::query::{QueryButton, QueryPanel};
}
