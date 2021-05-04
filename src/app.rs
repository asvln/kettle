use paste::paste;
use crate::{dirs, config::Config};

#[doc(hidden)]
macro_rules! gen_dirs {
    ($($dir_name:expr),*) => {
        $(
            paste! {
                #[doc = " PathBuf for App's `" $dir_name "` directory."]
                pub fn [<$dir_name _dir>](&self) -> std::path::PathBuf {
                    dirs::[<$dir_name _dir>]().join(&self.name)
                }
            }
        )*
    }
}

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

    gen_dirs! {"cache", "config", "data", "data_local", "preference"}
}

pub const fn app(name: &'static str, config_file: Option<&'static str>) -> App {
    App { name, config_file }
}

