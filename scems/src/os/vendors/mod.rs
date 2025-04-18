#[cfg(feature = "os-cmsis-os")]
mod cmsis_os;

#[cfg(feature = "os-cmsis-os")]
pub use cmsis_os::*;
