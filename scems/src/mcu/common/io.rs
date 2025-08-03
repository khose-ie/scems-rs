// ！ The definations for MCU peripheral GPIO_TypeDef.
// ！
// ！ The definations in this module provide a common interface to operate a GPIO_TypeDef pin.

use super::EventLaunch;
use crate::derive::EnumCastU32;

/// Trait providing operations of an GPIO_TypeDef pin.
pub trait IoCtrl
where
    Self: EventLaunch<dyn IoDeviceEventAgent>,
{
    type Pin;

    /// Get the GPIO_TypeDef pin level state.
    fn state(&self) -> IoState;

    /// Let pin output high or low level, only effective in output mode.
    fn set_state(&self, state: IoState);

    /// Toggle the outputing pin level, only effective in output mode.
    fn toggle(&self);
}

pub trait IoDeviceEventAgent
{
    fn on_io_state_change(&self) {}
}

/// Level state of a GPIO_TypeDef.
#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy, EnumCastU32)]
pub enum IoState
{
    /// Low level, the value is `0`.
    Reset = 0,

    /// High level, the value is `1`.
    Set = 1,
}
