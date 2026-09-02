#[macro_use]
extern crate rust_i18n;

use rust_i18n::i18n;
i18n!("locales", fallback = "en");

mod app;
mod error;
mod turing;
mod ui;
mod utils;
mod api;

pub use app::App;
