use core::mem::transmute;

use crate::derive::{EnumCastU16, EnumCount};
use crate::mcu::common::io::{IoCtrl, IoDeviceEventAgent, IoState};
use crate::mcu::common::EventLaunch;
use crate::mcu::vendor::stm::native::io::*;

pub use crate::mcu::vendor::stm::native::io::GPIO_TypeDef;

#[repr(u16)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, EnumCount, EnumCastU16)]
pub enum GPIO_Pin
{
    P00 = 0x0001,
    P01 = 0x0002,
    P02 = 0x0004,
    P03 = 0x0008,
    P04 = 0x0010,
    P05 = 0x0020,
    P06 = 0x0040,
    P07 = 0x0080,
    P08 = 0x0100,
    P09 = 0x0200,
    P10 = 0x0400,
    P11 = 0x0800,
    P12 = 0x1000,
    P13 = 0x2000,
    P14 = 0x4000,
    P15 = 0x8000,
}

static mut IO_EVENT_QUEUE: [Option<*const dyn IoDeviceEventAgent>; GPIO_Pin::count()] = [None; GPIO_Pin::count()];

pub struct Io
{
    handle: *mut GPIO_TypeDef,
    pin: GPIO_Pin,
}

impl Io
{
    pub const fn new(handle: *mut GPIO_TypeDef, pin: GPIO_Pin) -> Self
    {
        Io { handle, pin }
    }
}

impl EventLaunch<dyn IoDeviceEventAgent> for Io
{
    fn set_event_agent(&mut self, event_handle: &'static dyn IoDeviceEventAgent)
    {
        let pin: u16 = self.pin.into();
        unsafe {
            IO_EVENT_QUEUE[pin.trailing_zeros() as usize] =
                Some(transmute(event_handle as *const dyn IoDeviceEventAgent))
        };
    }

    fn clean_event_agent(&mut self)
    {
        let pin: u16 = self.pin.into();
        unsafe { IO_EVENT_QUEUE[pin.trailing_zeros() as usize] = None };
    }
}

impl IoCtrl for Io
{
    type Pin = GPIO_Pin;

    fn state(&self) -> IoState
    {
        match unsafe { HAL_GPIO_ReadPin(self.handle, self.pin.into()) }
        {
            GPIO_PinState::GPIO_PIN_RESET => IoState::Reset,
            GPIO_PinState::GPIO_PIN_SET => IoState::Set,
        }
    }

    fn set_state(&self, state: IoState)
    {
        let native_state: GPIO_PinState = match state
        {
            IoState::Reset => GPIO_PinState::GPIO_PIN_RESET,
            IoState::Set => GPIO_PinState::GPIO_PIN_SET,
        };

        unsafe { HAL_GPIO_WritePin(self.handle, self.pin.into(), native_state) };
    }

    fn toggle(&self)
    {
        unsafe { HAL_GPIO_TogglePin(self.handle, self.pin.into()) };
    }
}

pub struct IoQueue;

impl IoQueue
{
    #[inline]
    #[allow(static_mut_refs)]
    pub fn allocate(sample_handle: *mut GPIO_TypeDef, pin: GPIO_Pin) -> Io
    {
        Io::new(sample_handle, pin)
    }
}

/////////////////////////////////////////////////////////////////////////////
// HAL interrupt callback function implementations
/////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern "C" fn HAL_GPIO_EXTI_Callback(pin: u16)
{
    if let Some(event_handle) = IO_EVENT_QUEUE[pin.trailing_zeros() as usize]
    {
        (*event_handle).on_io_state_change();
    }
}

#[no_mangle]
pub unsafe extern "C" fn HAL_GPIO_EXTI_Rising_Callback(pin: u16)
{
    if let Some(event_handle) = IO_EVENT_QUEUE[pin.trailing_zeros() as usize]
    {
        (*event_handle).on_io_state_change();
    }
}

#[no_mangle]
pub unsafe extern "C" fn HAL_GPIO_EXTI_Falling_Callback(pin: u16)
{
    if let Some(event_handle) = IO_EVENT_QUEUE[pin.trailing_zeros() as usize]
    {
        (*event_handle).on_io_state_change();
    }
}
