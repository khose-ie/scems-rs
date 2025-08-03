// ！ The definations for MCU peripheral GPIO_TypeDef.
// ！
// ！ The definations in this module provide a common interface to operate a GPIO_TypeDef pin.

use super::EventLaunch;
use crate::derive::EnumCastU32;

#[cfg(feature = "mcu-stm")]
use crate::mcu::vendor::stm::io::{Io as VendorIo, GPIO_Pin as VendorPin};

pub struct IoDevice
{
    sample: VendorIo,
}

impl IoDevice
{
    pub fn new(sample: VendorIo) -> IoDevice
    {
        IoDevice { sample }
    }
}

impl EventLaunch<dyn IoCtrlEvent> for IoDevice
{
    #[inline]
    fn set_event_agent(&mut self, event_handle: &'static dyn IoCtrlEvent)
    {
        self.sample.set_event_agent(event_handle);
    }

    #[inline]
    fn clean_event_agent(&mut self)
    {
        self.sample.clean_event_agent();
    }
}

impl IoCtrl for IoDevice
{
    type Pin = VendorPin;

    #[inline]
    fn state(&self) -> IoState
    {
        self.sample.state()
    }

    #[inline]
    fn set_state(&self, state: IoState)
    {
        self.sample.set_state(state);
    }

    #[inline]
    fn toggle(&self)
    {
        self.sample.toggle();
    }
}

/// Trait providing operations of an GPIO_TypeDef pin.
pub trait IoCtrl
where
    Self: EventLaunch<dyn IoCtrlEvent>,
{
    type Pin;

    /// Get the GPIO_TypeDef pin level state.
    fn state(&self) -> IoState;

    /// Let pin output high or low level, only effective in output mode.
    fn set_state(&self, state: IoState);

    /// Toggle the outputing pin level, only effective in output mode.
    fn toggle(&self);
}

pub trait IoCtrlEvent
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
