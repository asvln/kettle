//! A library for building applications natively on Linux/Redox/macOS/Windows.
//! It currently provides...
//!
//! - app-specific `dirs`
//! - easy `ini` config files
//!
//! This crate utilizes the [`dirs`](https://crates.io/crates/dirs) crate and re-exports it for easy access.
//!
//! # Usage
//! ```
//! pub(crate) const THIS_APP: kettle::App = kettle::app("this_APP", None);
//!
//! fn main() {
//!     let config_dir = THIS_APP.config_dir(); //$HOME/.config/this_APP/
//!     let data_dir = THIS_APP.data_dir();     //$HOME/.local/share/this_APP/
//!
//!     // default config file
//!     THIS_APP.config()
//!         .set("view", Some("horizontal"));
//!
//!     let view = THIS_APP.config()
//!         .get("view");
//!
//!     assert_eq!(view, Some("horizontal".to_string()));
//!
//!     // named config file
//!     THIS_APP.config_file("admin_profiles")
//!         .section("dev") // setting `ini` sections is possible
//!         .set("view", None);
//!
//!     let admin_view = THIS_APP.config_file("admin_profiles")
//!         .section("dev")
//!         .get("view");
//!
//!     assert_eq!(admin_view, None);
//! }
//! ```
//!
pub mod dirs;

mod app;
pub use app::{app, App};
mod config;
mod error;
