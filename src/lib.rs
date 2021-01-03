//! `kettle` is a small library which provides simple cross-platform access to project specific
//! `config`, `cache`, `data`, etc. directories for applications running on Linux/macOS/Windows.
//! It also includes an `ini` based config file which can be easily used for application settings.
//!
//! This crate utilizes the [`dirs`](https://crates.io/crates/dirs) crate and re-exports it for
//! easy access to non-application directories.
//!
//! # Usage
//! ```
//! use kettle::Project;
//!
//! pub const APP: Project = Project::init("app_name", None);
//!
//! fn main() {
//!     let config_dir = APP.config_dir();
//!     let cache_dir = APP.cache_dir();
//!     let data_dir = APP.data_dir();
//!
//!     APP.config_set("default_view", Some("vertical"));
//!     APP.config_section_set("admin", "default_view", Some("horizontal"));
//!     APP.config_set("key with spaces", None);
//!
//!     let default_view = APP.config_get("default_view").unwrap();
//!     let admin_view = APP.config_section_get("admin", "default_view").unwrap();
//!     let spaces_key = APP.config_get("key with spaces").unwrap();
//!
//!     assert_eq!(default_view, Some("vertical".to_string()));
//!     assert_eq!(admin_view, Some("horizontal".to_string()));
//!     assert_eq!(spaces_key, None);
//!
//!     // Default config file name is 'config' and is saved in the respective config directory.
//!     // Config file name can be changed during initialization.
//!     // e.g. `Project::init("app_name", "settings.ini");
//! }

use error::*;
use ini::Ini;
use std::path::PathBuf;

pub mod dirs;
mod error;

/// The `Project` struct is used to access your directories and config throughout your project.
/// It allows you to set the project name and an optional config file name. See `init()` for more
/// info.
#[derive(Debug)]
pub struct Project {
    project_name: &'static str,
    config_name: Option<&'static str>,
}

impl Project {
    /// Initializes your project for use throughout your app. Easily set as a constant so you only
    /// have to declare it once.
    /// ## Example
    /// ```
    /// use kettle::Project;
    ///
    /// pub const APP: Project = Project::init("app_name", None);
    ///
    /// // or...
    ///
    /// pub const fn app() {
    ///     Project::init("app_name", Some("settings.ini"));
    /// }
    /// ```
    pub const fn init(project_name: &'static str, config_name: Option<&'static str>) -> Self {
        Self {
            project_name,
            config_name,
        }
    }

    // Project directories

    /// PathBuf for project's cache directory
    /// ## Example
    /// ```
    /// let cache_dir = APP.cache_dir();
    /// ```
    pub fn cache_dir(&self) -> PathBuf {
        dirs::cache_dir().join(&self.project_name)
    }

    /// PathBuf for project's config directory
    /// ## Example
    /// ```
    /// let config_dir = APP.config_dir();
    /// ```
    pub fn config_dir(&self) -> PathBuf {
        dirs::config_dir().join(&self.project_name)
    }

    /// PathBuf for project's data directory
    /// ## Example
    /// ```
    /// let data_dir = APP.data_dir();
    /// ```
    pub fn data_dir(&self) -> PathBuf {
        dirs::data_dir().join(&self.project_name)
    }

    /// PathBuf for project's local data directory
    /// ## Example
    /// ```
    /// let data_local_dir = APP.data_local_dir();
    /// ```
    pub fn data_local_dir(&self) -> PathBuf {
        dirs::data_local_dir().join(&self.project_name)
    }

    /// PathBuf for project's preference directory
    /// ## Example
    /// ```
    /// let preference_dir = APP.preference_dir();
    /// ```
    pub fn preference_dir(&self) -> PathBuf {
        dirs::preference_dir().join(&self.project_name)
    }

    // Project config

    /// PathBuf for project's config file. If no name is provided during init, it will default to
    /// 'config'
    /// ## Example
    /// ```
    /// let config_file = APP.config_file();
    /// ```

    pub fn config_file(&self) -> PathBuf {
        if let Some(n) = self.config_name {
            self.config_dir().join(n)
        } else {
            self.config_dir().join("config")
        }
    }

    // Load config file
    fn load(&self) -> Result<Ini> {
        let file = self.config_file();
        let config = Ini::load_from_file(file)?;
        Ok(config)
    }

    // Create config file
    fn create(&self) -> Result<()> {
        std::fs::create_dir_all(self.config_dir())?;
        std::fs::write(self.config_file(), b"")?;
        Ok(())
    }

    // Save updated config file to disk
    fn save(&self, config: Ini) -> Result<()> {
        let file = self.config_file();
        config.write_to_file(file)?;
        Ok(())
    }

    fn set_or_delete<S: Into<String>>(
        &self,
        mut config: Ini,
        section: Option<S>,
        key: &'static str,
        value: Option<&'static str>,
    ) -> Result<Ini> {
        if let Some(v) = value {
            config.with_section(section).set(key, v);
            Ok(config)
        } else {
            config.delete_from(section, key);
            Ok(config)
        }
    }

    /// Set config key and value. Creates config file if it does not exist.
    /// ## Example
    /// ```
    /// APP.config_set("default-view", Some("vertical"))
    /// APP.config_set("default-template", None)
    /// ```
    /// If value is set to `None`, then key field is deleted from the file.
    /// Returns `Err` if unable to write config file.
    pub fn config_set(&self, key: &'static str, value: Option<&'static str>) -> Result<()> {
        let section = None::<String>;
        if let Ok(config) = self.load() {
            let c = self.set_or_delete(config, section, key, value)?;
            self.save(c)?;
            Ok(())
        } else {
            self.create()?;
            let config = self.load()?;
            let c = self.set_or_delete(config, section, key, value)?;
            self.save(c)?;
            Ok(())
        }
    }

    /// Set config key and value for a specific section. Creates config file if it does not exist.
    /// ## Example
    /// ```
    /// APP.config_section_set("admin", "default-view", Some("vertical"))
    /// APP.config_section_set("user.bob", "default-template", None)
    /// ```
    /// If value is set to `None`, then key field is deleted from the file.
    /// If section is empty upon deletion, the section is deleted as well.
    /// Returns `Err` if unable to write config file.
    pub fn config_section_set<S: Into<String>>(
        &self,
        section: S,
        key: &'static str,
        value: Option<&'static str>,
    ) -> Result<()> {
        if let Ok(config) = self.load() {
            let c = self.set_or_delete(config, Some(section), key, value)?;
            self.save(c)?;
            Ok(())
        } else {
            self.create()?;
            let config = self.load()?;
            let c = self.set_or_delete(config, Some(section), key, value)?;
            self.save(c)?;
            Ok(())
        }
    }

    /// Get config key's value and read it as a String.
    /// ## Example
    /// ```
    /// APP.config_get("default-view");
    /// ```
    /// Returns `Ok(None)` if key does not exist or if field is blank.
    /// Returns `Err` if unable to read config file.
    pub fn config_get(&self, key: &str) -> Result<Option<String>> {
        let config = self.load()?;
        let section = None::<String>;
        let value = config.get_from(section, key);
        match value {
            Some(v) => Ok(Some(v.to_string())),
            None => Ok(None),
        }
    }

    /// Get config key's value from a specific section and read it as a String.
    /// ## Example
    /// ```
    /// APP.config_section_get("user.bob", "default-view");
    /// ```
    /// Returns `Ok(None)` if key does not exist or if field is blank.
    /// Returns `Err` if unable to read config file.
    pub fn config_section_get<S: Into<String>>(
        &self,
        section: S,
        key: &str,
    ) -> Result<Option<String>> {
        let config = self.load()?;
        let value = config.get_from(Some(section), key);
        match value {
            Some(v) => Ok(Some(v.to_string())),
            None => Ok(None),
        }
    }
}
