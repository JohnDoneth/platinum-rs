#[cfg(target_os = "windows")]
pub mod win32;

#[cfg(target_os = "windows")]
use win32 as platinum;

#[cfg(target_os = "macos")]
pub mod osx;

#[cfg(target_os = "macos")]
use osx as platinum;

pub use self::platinum::*;
