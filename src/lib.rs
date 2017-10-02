#[cfg(target_os = "windows")]
pub mod win32;

#[cfg(target_os = "windows")]
use win32 as platinum;

pub use self::platinum::*;
