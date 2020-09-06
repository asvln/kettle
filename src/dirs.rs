//! Aliases to `dirs` crate.
//!
//! PathBufs are automatically unwrapped if dirs are infallible on all platforms.

use dirs;
use std::path::*;

pub fn home_dir() -> PathBuf {
    dirs::home_dir().unwrap()
}

pub fn cache_dir() -> PathBuf {
    dirs::cache_dir().unwrap()
}

pub fn config_dir() -> PathBuf {
    dirs::config_dir().unwrap()
}

pub fn data_dir() -> PathBuf {
    dirs::data_dir().unwrap()
}

pub fn data_local_dir() -> PathBuf {
    dirs::data_local_dir().unwrap()
}

pub fn executable_dir() -> Option<PathBuf> {
    dirs::executable_dir()
}

pub fn preference_dir() -> PathBuf {
    dirs::preference_dir().unwrap()
}

pub fn runtime_dir() -> Option<PathBuf> {
    dirs::runtime_dir()
}

pub fn audio_dir() -> Option<PathBuf> {
    dirs::audio_dir()
}

pub fn desktop_dir() -> Option<PathBuf> {
    dirs::desktop_dir()
}

pub fn document_dir() -> Option<PathBuf> {
    dirs::document_dir()
}

pub fn download_dir() -> Option<PathBuf> {
    dirs::download_dir()
}

pub fn font_dir() -> Option<PathBuf> {
    dirs::font_dir()
}

pub fn picture_dir() -> Option<PathBuf> {
    dirs::picture_dir()
}

pub fn public_dir() -> Option<PathBuf> {
    dirs::public_dir()
}

pub fn template_dir() -> Option<PathBuf> {
    dirs::template_dir()
}

pub fn video_dir() -> Option<PathBuf> {
    dirs::video_dir()
}
