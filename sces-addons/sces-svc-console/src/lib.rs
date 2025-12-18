#![no_std]

extern crate alloc;

mod console;
mod native;
mod svc;

pub use console::Console;
pub use console::ConsoleCommands;
pub use console::ConsoleExecute;
pub use native::NativeConsole;
pub use svc::ConsoleService;
