// ！ The definations for MCU peripheral GPIO.
// ！
// ！ The definations in this module provide a common interface to operate a GPIO pin.

use crate::derive::EnumCastU32;

use super::{AsEventPtr, EventHandle};

/// Trait providing operations of an GPIO pin.
pub trait Io
where
    Self: EventHandle<dyn IoEventPtr>,
{
    type Pin;

    /// Get the GPIO pin level state.
    fn state(&self) -> IoState;

    /// Let pin output high or low level, only effective in output mode.
    fn set_state(&self, state: IoState);

    /// Toggle the outputing pin level, only effective in output mode.
    fn toggle(&self);
}

pub trait IoEvent
{
    fn on_io_state_change(&mut self) {}
}

pub trait IoEventPtr
where
    Self: IoEvent + AsEventPtr<dyn IoEvent>,
{
}

/// Level state of a GPIO.
#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy, EnumCastU32)]
pub enum IoState
{
    /// Low level, the value is `0`.
    Reset = 0,

    /// High level, the value is `1`.
    Set = 1,
}
