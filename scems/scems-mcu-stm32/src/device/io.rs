use core::ptr::NonNull;

use scems::value::{ErrValue, RetValue};
use scems_derive::{EnumAsU16, EnumCount};
use scems_mcu::io::{IoCtrl, IoCtrlEvent, IoState};
use scems_mcu::EventLaunch;

use crate::device::Handle;
use crate::native::io::*;
use crate::sample_queue::SampleQueue;
use crate::IO_COUNT;

pub use crate::native::io::GPIO_TypeDef;

/////////////////////////////////////////////////////////////////////////////
// IO Class
/////////////////////////////////////////////////////////////////////////////

#[repr(u16)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, EnumCount, EnumAsU16)]
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

#[derive(Clone, Copy)]
pub struct Io
{
    handle: NonNull<GPIO_TypeDef>,
    event_handle: Option<&'static dyn IoCtrlEvent>,
    pin: GPIO_Pin,
}

impl Io
{
    pub fn new(handle: *mut GPIO_TypeDef, pin: GPIO_Pin) -> RetValue<Self>
    {
        Ok(Io { handle: NonNull::new(handle).ok_or(ErrValue::Param)?, event_handle: None, pin })
    }
}

impl Handle<GPIO_TypeDef> for Io
{
    fn handle_value(&self) -> *mut GPIO_TypeDef
    {
        self.handle.as_ptr()
    }
}

impl EventLaunch<dyn IoCtrlEvent> for Io
{
    #[allow(static_mut_refs)]
    fn set_event_agent(&mut self, event_handle: &'static dyn IoCtrlEvent)
    {
        let pin: u16 = self.pin.into();

        self.event_handle = Some(event_handle);

        if let Some(handle) = unsafe { IO_QUEUE.search(self.handle).ok() }
        {
            unsafe { IO_EVENT_QUEUE[pin.trailing_zeros() as usize] = Some(handle) };
        }
    }

    fn clean_event_agent(&mut self)
    {
        let pin: u16 = self.pin.into();
        unsafe { IO_EVENT_QUEUE[pin.trailing_zeros() as usize] = None };
    }
}

impl IoCtrl for Io
{
    fn state(&self) -> IoState
    {
        unsafe { HAL_GPIO_ReadPin(self.handle.as_ptr(), self.pin.into()).into() }
    }

    fn set_state(&self, state: IoState)
    {
        unsafe { HAL_GPIO_WritePin(self.handle.as_ptr(), self.pin.into(), state.into()) };
    }

    fn toggle(&self)
    {
        unsafe { HAL_GPIO_TogglePin(self.handle.as_ptr(), self.pin.into()) };
    }
}

/////////////////////////////////////////////////////////////////////////////
// IO Queue
/////////////////////////////////////////////////////////////////////////////

static mut IO_QUEUE: SampleQueue<Io, GPIO_TypeDef, IO_COUNT> = SampleQueue::new();
static mut IO_EVENT_QUEUE: [Option<&'static Io>; GPIO_Pin::count()] = [None; GPIO_Pin::count()];

pub struct IoQueue;

impl IoQueue
{
    #[inline]
    #[allow(static_mut_refs)]
    pub fn allocate(sample_handle: *mut GPIO_TypeDef, pin: GPIO_Pin) -> RetValue<&'static mut Io>
    {
        unsafe { IO_QUEUE.allocate_channel(&Io::new(sample_handle, pin)?, pin as u32) }
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn clean(sample_handle: *mut GPIO_TypeDef, pin: GPIO_Pin)
    {
        NonNull::new(sample_handle)
            .map(|handle| unsafe { IO_QUEUE.clean_channel(handle, pin as u32) });
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn search(sample_handle: *mut GPIO_TypeDef, pin: GPIO_Pin) -> RetValue<&'static Io>
    {
        unsafe {
            IO_QUEUE.search_channel(NonNull::new(sample_handle).ok_or(ErrValue::Param)?, pin as u32)
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// HAL interrupt callback function implementations
/////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern "C" fn HAL_GPIO_EXTI_Callback(pin: u16)
{
    if let Some(sample) = IO_EVENT_QUEUE[pin.trailing_zeros() as usize]
    {
        sample.event_handle.map(|event_handle| event_handle.on_io_state_change());
    }
}

#[no_mangle]
pub unsafe extern "C" fn HAL_GPIO_EXTI_Rising_Callback(pin: u16)
{
    if let Some(sample) = IO_EVENT_QUEUE[pin.trailing_zeros() as usize]
    {
        sample.event_handle.map(|event_handle| event_handle.on_io_state_change());
    }
}

#[no_mangle]
pub unsafe extern "C" fn HAL_GPIO_EXTI_Falling_Callback(pin: u16)
{
    if let Some(sample) = IO_EVENT_QUEUE[pin.trailing_zeros() as usize]
    {
        sample.event_handle.map(|event_handle| event_handle.on_io_state_change());
    }
}
