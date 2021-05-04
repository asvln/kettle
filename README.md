[![crates.io](https://img.shields.io/crates/v/kettle.svg)](https://crates.io/crates/kettle)
[![API documentation](https://docs.rs/kettle/badge.svg)](https://docs.rs/kettle/)
![actively developed](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-orange.svg)

# kettle
A library for building applications natively on Linux/Redox/macOS/Windows.
It currently provides...

- app-specific `dirs`
- easy `ini` config files

This crate utilizes the [`dirs`](https://crates.io/crates/dirs) crate and re-exports it for easy access.

## Usage
```rust
kettle::app!("this_APP")

fn main() {
    let config_dir = THIS_APP.config_dir(); //$HOME/.config/this_APP/
    let data_dir = THIS_APP.data_dir();     //$HOME/.local/share/this_APP/

    // default config file
    THIS_APP.config()
        .set("view", Some("horizontal"));

    let view = THIS_APP.config()
        .get("view");

    assert_eq!(view.unwrap(), Some("horizontal".to_string()));

    // named config file
    THIS_APP.config_file("admin_profiles")
        .section("dev") // setting `ini` sections is possible
        .set("view", None);

    let admin_view = THIS_APP.config_file("admin_profiles")
        .section("dev");
        .get("view");

    assert_eq!(admin_view.unwrap(), None);
}
```

## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.
