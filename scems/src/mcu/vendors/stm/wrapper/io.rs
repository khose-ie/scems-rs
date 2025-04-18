use crate::common::result::IResult;
use crate::derive::AsIoEventPtr;
use crate::derive::{EnumCastU16, EnumCount};
use crate::mcu::common::io::{Io, IoEvent, IoEventPtr, IoState};
use crate::mcu::common::{AsEventPtr, EventHandle};
use crate::mcu::vendors::stm::native::io::*;

#[repr(u16)]
#[derive(Clone, Copy, EnumCount, EnumCastU16)]
pub enum IoPin
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

#[derive(AsIoEventPtr)]
pub struct IoEventDefault;

static DEF_EVENT_HANDLE: IoEventDefault = IoEventDefault;
static mut EVENT_HANDLES: [Option<*mut dyn IoEvent>; IoPin::count()] = [None; IoPin::count()];

pub struct IoDevice
{
    handle: *mut GPIO,
    pin: IoPin,
}

impl IoDevice
{
    pub const fn new(handle: *mut GPIO, pin: IoPin) -> Self
    {
        IoDevice { handle, pin }
    }
}

impl EventHandle<dyn IoEventPtr> for IoDevice
{
    fn set_event_handle(&mut self, event_handle: &dyn IoEventPtr) -> IResult<()>
    {
        let pin: u16 = self.pin.into();
        unsafe { EVENT_HANDLES[pin.trailing_zeros() as usize] = Some(event_handle.as_event_ptr()) };
        Ok(())
    }

    fn clean_event_handle(&mut self)
    {
        let pin: u16 = self.pin.into();
        unsafe { EVENT_HANDLES[pin.trailing_zeros() as usize] = None };
    }
}

impl Io for IoDevice
{
    type Pin = IoPin;

    fn state(&self) -> IoState
    {
        IoState::from(unsafe { HAL_GPIO_ReadPin(self.handle, self.pin.into()) })
    }

    fn set_state(&self, state: IoState)
    {
        unsafe { HAL_GPIO_WritePin(self.handle, self.pin.into(), state.into()) };
    }

    fn toggle(&self)
    {
        unsafe { HAL_GPIO_TogglePin(self.handle, self.pin.into()) };
    }
}

#[no_mangle]
pub unsafe extern "C" fn HAL_GPIO_EXTI_Callback(pin: u16)
{
    let event_handle = &mut *EVENT_HANDLES[pin.trailing_zeros() as usize].unwrap_or(DEF_EVENT_HANDLE.as_event_ptr());
    event_handle.on_io_state_change();
}

#[no_mangle]
pub unsafe extern "C" fn HAL_GPIO_EXTI_Rising_Callback(pin: u16)
{
    let event_handle = &mut *EVENT_HANDLES[pin.trailing_zeros() as usize].unwrap_or(DEF_EVENT_HANDLE.as_event_ptr());
    event_handle.on_io_state_change();
}

#[no_mangle]
pub unsafe extern "C" fn HAL_GPIO_EXTI_Falling_Callback(pin: u16)
{
    let event_handle = &mut *EVENT_HANDLES[pin.trailing_zeros() as usize].unwrap_or(DEF_EVENT_HANDLE.as_event_ptr());
    event_handle.on_io_state_change();
}
