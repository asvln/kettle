[![crates.io](https://img.shields.io/crates/v/kettle.svg)](https://crates.io/crates/kettle)
[![API documentation](https://docs.rs/kettle/badge.svg)](https://docs.rs/kettle/)
![actively developed](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-orange.svg)

# kettle
`kettle` is a small library which provides simple cross-platform access to project specific `config`, `cache`, `data`, etc. directories for applications running on Linux/macOS/Windows. It also includes an `ini` based config file which can be easily used for application settings.

This crate utilizes the [`dirs`](https://crates.io/crates/dirs) crate and re-exports it for easy access to non-application specific directories.

## Usage
```rust
use kettle::Project;

pub const APP: Project = Project::init("app_name", None);

fn main() {
    let config_dir = APP.config_dir();
    let cache_dir = APP.cache_dir();
    let data_dir = APP.data_dir();

    APP.config_set("default_view", Some("vertical"));
    APP.config_section_set("admin", "default_view", Some("horizontal"));
    APP.config_set("key with spaces", None);

    let default_view = APP.config_get("default_view").unwrap();
    let admin_view = APP.config_section_get("admin", "default_view").unwrap();
    let spaces_key = APP.config_get("key with spaces").unwrap();

    assert_eq!(default_view, Some("vertical".to_string()));
    assert_eq!(admin_view, Some("horizontal".to_string()));
    assert_eq!(spaces_key, None);

    // Default config file name is 'config' and is saved in the respective config directory.
    // Config file name can be changed during initialization.
    // e.g. `Project::init("app_name", "settings.ini");
}
```

## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option. 
