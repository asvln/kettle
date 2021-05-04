use crate::error::*;
use ini::Ini;
use std::fs;
use std::path::PathBuf;

pub struct Config {
    path: (PathBuf, &'static str),
    section: Option<&'static str>,
}
impl Config {
    pub fn from(dir: PathBuf, file: &'static str) -> Self {
        Self {
            path: (dir, file),
            section: None,
        }
    }

    // Gets value from config. Will return `None` if key or config file does not exist.
    pub fn get(&self, key: &str) -> Option<String> {
        if let Ok(config) = self.load() {
            if let Some(value) = config.get_from(self.section, key) {
                Some(value.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }
    // Sets value to config. Keys with `None` values are removed.
    pub fn set<S: Into<String>>(&self, key: &'static str, value: Option<S>) -> Result<()> {
        if let Ok(config) = self.load() {
            self.save(self.set_or_delete(config, key, value)?)
        } else {
            self.create_empty()?;
            self.save(self.set_or_delete(self.load()?, key, value)?)
        }
    }
    fn set_or_delete<S: Into<String>>(
        &self,
        mut config: Ini,
        key: &'static str,
        value: Option<S>,
    ) -> Result<Ini> {
        if let Some(v) = value {
            config.set_to(self.section, key.to_string(), v.into());
            Ok(config)
        } else {
            config.delete_from(self.section, key);
            Ok(config)
        }
    }

    // Adds a section to a config query.
    pub fn section(mut self, section: &'static str) -> Self {
        self.section = Some(section);
        self
    }

    // std::fs
    fn path(&self) -> PathBuf {
        self.path.0.join(self.path.1)
    }
    fn load(&self) -> Result<Ini> {
        let file_str = fs::read_to_string(self.path())?;
        let config = Ini::load_from_str(&file_str)?;
        Ok(config)
    }
    fn save(&self, config: Ini) -> Result<()> {
        config.write_to_file(self.path())?;
        Ok(())
    }
    fn create_empty(&self) -> Result<()> {
        std::fs::create_dir_all(self.path.0.clone())?;
        std::fs::write(self.path(), b"")?;
        Ok(())
    }
}
