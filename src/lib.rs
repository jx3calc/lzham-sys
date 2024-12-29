#[cfg(target_os = "windows")]
#[cfg(target_env = "msvc")]
mod windows_msvc;
#[cfg(target_os = "windows")]
#[cfg(target_env = "msvc")]
pub use windows_msvc::*;

#[cfg(target_os = "macos")]
mod darwin;
#[cfg(target_os = "macos")]
pub use darwin::*;

#[cfg(target_os = "linux")]
#[cfg(target_env = "gnu")]
mod linux_gnu;
#[cfg(target_os = "linux")]
#[cfg(target_env = "gnu")]
pub use linux_gnu::*;
