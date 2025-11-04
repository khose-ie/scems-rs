#![no_std]

extern crate alloc;

mod native;

pub use native::NativeConsoleCommandsExecute;
pub use native::NativeConsoleCommandsParser;
pub use native::NativeConsoleService;
