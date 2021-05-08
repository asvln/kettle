use crate::{dirs, config::Config};
use std::path::PathBuf;

#[derive(Debug)]
pub struct App {
    name: &'static str,
    config_file: Option<&'static str>,
}

impl App {
    /// Handle to the default config file.
    pub fn config(&self) -> Config {
        let dir = dirs::config_dir().join(self.name);
        let file = self.config_file.unwrap_or("config");
        Config::from(dir, file)
    }

    /// Handle to a named config file.
    pub fn config_file(&self, file: &'static str) -> Config {
        let dir = dirs::config_dir().join(self.name);
        Config::from(dir, file)
    }

    /// PathBuf for your application's cache directory
    /// ## Example
    /// ```
    /// let cache_dir = APP.cache_dir();
    /// ```
    pub fn cache_dir(&self) -> PathBuf {
        dirs::cache_dir().join(&self.name)
    }

    /// PathBuf for your application's config directory
    /// ## Example
    /// ```
    /// let config_dir = APP.config_dir();
    /// ```
    pub fn config_dir(&self) -> PathBuf {
        dirs::config_dir().join(&self.name)
    }

    /// PathBuf for your application's data directory
    /// ## Example
    /// ```
    /// let data_dir = APP.data_dir();
    /// ```
    pub fn data_dir(&self) -> PathBuf {
        dirs::data_dir().join(&self.name)
    }

    /// PathBuf for your application's local data directory
    /// ## Example
    /// ```
    /// let data_local_dir = APP.data_local_dir();
    /// ```
    pub fn data_local_dir(&self) -> PathBuf {
        dirs::data_local_dir().join(&self.name)
    }

    /// PathBuf for your application's preference directory
    /// ## Example
    /// ```
    /// let preference_dir = APP.preference_dir();
    /// ```
    pub fn preference_dir(&self) -> PathBuf {
        dirs::preference_dir().join(&self.name)
    }

}


/// Initializes a `kettle::App`.
///
/// Defining this as a const with `pub(crate)` visibility will allow you to utilize `kettle`
/// across your code.
/// # Example
/// ```
/// pub(crate) const THIS_APP: kettle::App = kettle::app("this_APP", None)
///
/// // you can optionally define a custom default config filename
/// pub(crate) const THIS_APP: kettle::App = kettle::app("this_APP", Some("config.ini")
/// ```
pub const fn app(name: &'static str, config_file: Option<&'static str>) -> App {
    App { name, config_file }
}

