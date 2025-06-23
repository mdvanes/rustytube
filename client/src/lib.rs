#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

pub mod list_items;

pub mod list_items_eventsourced;

pub mod geo;