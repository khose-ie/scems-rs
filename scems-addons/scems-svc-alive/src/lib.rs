#![no_std]

extern crate alloc;

mod alive;
mod native;
mod svc;

pub use alive::AliveWatch;
pub use alive::AliveWatchHandle;
pub use native::NativeAliveWatch;
pub use svc::AliveWatchService;
