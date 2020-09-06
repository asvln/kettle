# kettle
[![Crates.io](https://img.shields.io/crates/v/kettle)](https://crates.io/crates/configparser) [![Released API docs](https://docs.rs/kettle/badge.svg)](https://docs.rs/kettle)

`kettle` is a boiler for platform-specific applications

It is a small library which provides simple cross-platform access to project specific `config`, `cache`, `data`, etc. directories for applications running on Linux/macOS/Windows. It also includes an `ini` based config file which can be easily used for application settings.

This crate utilizes the [`dirs`](https://crates.io/crates/dirs) crate and re-exports it for easy access to non-application
specific directories.

## Usage
```rust
use kettle::Project;

pub const APP: Project = Project::init("app_name", None);

fn main() {
    let config_dir = APP.config_dir();
    let cache_dir = APP.cache_dir();
    let data_dir = APP.data_dir();
    
    // Example output of `config_dir`
    // Linux = /home/Alice/.config/app_name
    // macOS = /Users/Alice/Library/Application Support/app_name
    // Windows C:\Users\Alice\AppData\Roaming\app_name

    APP.config_set("default_view", Some("vertical"));
    APP.config_set("key with spaces", None);

    assert_eq!(APP.config_get("default_view"), Some("vertical"));
    assert_eq!(APP.config_get("key with spaces") None,

    // Default config file name is 'config' and is saved in the respective config directory.
    // Config file name can be changed during initialization.
    // e.g. `Project::init("app_name", "settings.ini");
}
```

## License
MIT
