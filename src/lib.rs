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
//! kettle::app!("this_APP");
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

pub mod app;
mod config;
pub mod dirs;
mod error;

#[doc(hidden)]
pub mod __ {
    pub use paste::paste;
}
/// Initializes a `kettle::app::App`.
///
/// Simple usage
/// ```
/// kettle::app!("this_APP");
/// ```
///
/// Custom const
/// ```
/// kettle::app!("this_APP", THAT_APP);
/// ```
///
/// Custom default config filename
/// ```
/// kettle::app!("this_APP" => "config.ini");
/// ```
///
/// Fully custom
/// ```
/// kettle::app!("this_APP", THAT_APP => "config.ini");
///```
#[macro_export]
macro_rules! app {
    ($name:literal) => {
        $crate::__::paste! {
            pub const [<$name:upper>]: $crate::app::App = $crate::app::app($name, None);
        }
    };
    ($name:literal, $const_name:expr) => {
        $crate::__::paste! {
            pub const [<$const_name:upper>]: $crate::app::App = $crate::app::app($name, None);
        }
    };
    ($name:literal => $config_file:literal) => {
        $crate::__::paste! {
            pub const [<$name:upper>]: $crate::app::App = $crate::app::app($name, Some($config_file));
        }
    };
    ($name:literal, $const_name:expr => $config_file:literal) => {
        $crate::__::paste! {
            pub const [<$const_name:upper>]: $crate::app::App = $crate::app::app($name, Some($config_file));
        }
    };
}
