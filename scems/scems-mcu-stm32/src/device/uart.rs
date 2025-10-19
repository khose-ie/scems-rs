//! The wrapper struct packed interfaces of STM32 HAL libraries to operate the UART peripheral.

use core::ptr::NonNull;

use scems::value::{ErrValue, RetValue};
use scems_mcu::uart::{UartCtrl, UartCtrlEvent};
use scems_mcu::EventLaunch;

use crate::device::Handle;
use crate::native::uart::*;
use crate::sample_queue::SampleQueue;
use crate::UART_COUNT;

pub use crate::native::uart::UART_HandleTypeDef;

/////////////////////////////////////////////////////////////////////////////
// UART Class
/////////////////////////////////////////////////////////////////////////////

/// The top encapsulation which be used to operate the STM32 UART peripheral.
///
/// This struct implements the trait [crate::mcu::common::uart::Uart], it will call STM32 HAL
/// functions to meet the specification of interface of the trait.
///
/// Please attention, create this struct will not initialize the peripheral, the initialization
/// should be done in a low-level code, and in the startup time of MCU.
/// For a initialized peripheral, you can create a this struct with it as the handle, and use
/// this struct to operate it.
#[derive(Clone, Copy)]
pub struct Uart
{
    handle: NonNull<UART_HandleTypeDef>,
    event_handle: Option<&'static dyn UartCtrlEvent>,
}

impl Uart
{
    fn new(handle: *mut UART_HandleTypeDef) -> RetValue<Self>
    {
        Ok(Uart { handle: NonNull::new(handle).ok_or(ErrValue::Param)?, event_handle: None })
    }
}

impl Handle<UART_HandleTypeDef> for Uart
{
    fn handle_value(&self) -> *mut UART_HandleTypeDef
    {
        self.handle.as_ptr()
    }
}

impl EventLaunch<dyn UartCtrlEvent> for Uart
{
    fn set_event_agent(&mut self, event_handle: &'static dyn UartCtrlEvent)
    {
        self.event_handle = Some(event_handle);
    }

    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
    }
}

impl UartCtrl for Uart
{
    fn transmit(&self, data: &[u8], timeout: u32) -> RetValue<()>
    {
        unsafe {
            HAL_UART_Transmit(self.handle.as_ptr(), data.as_ptr(), data.len() as u16, timeout)
                .into()
        }
    }

    fn receive(&self, data: &mut [u8], timeout: u32) -> RetValue<u32>
    {
        let mut size: u16 = 0;
        unsafe {
            HAL_UARTEx_ReceiveToIdle(
                self.handle.as_ptr(),
                data.as_ptr(),
                data.len() as u16,
                &mut size,
                timeout,
            )
            .ok()?
        };
        Ok(size as u32)
    }

    fn receive_size(&self, data: &mut [u8], timeout: u32) -> RetValue<()>
    {
        unsafe {
            HAL_UART_Receive(self.handle.as_ptr(), data.as_ptr(), data.len() as u16, timeout).into()
        }
    }

    fn async_transmit(&self, data: &[u8]) -> RetValue<()>
    {
        unsafe {
            HAL_UART_Transmit_DMA(self.handle.as_ptr(), data.as_ptr(), data.len() as u16).into()
        }
    }

    fn async_receive(&self, data: &mut [u8]) -> RetValue<()>
    {
        unsafe {
            HAL_UARTEx_ReceiveToIdle_DMA(self.handle.as_ptr(), data.as_ptr(), data.len() as u16)
                .into()
        }
    }

    fn async_receive_size(&self, data: &mut [u8]) -> RetValue<()>
    {
        unsafe {
            HAL_UART_Receive_DMA(self.handle.as_ptr(), data.as_ptr(), data.len() as u16).into()
        }
    }

    fn abort(&self) -> RetValue<()>
    {
        unsafe { HAL_UART_Abort(self.handle.as_ptr()).into() }
    }
}

/////////////////////////////////////////////////////////////////////////////
// UART queue
/////////////////////////////////////////////////////////////////////////////

static mut UART_QUEUE: SampleQueue<Uart, UART_HandleTypeDef, UART_COUNT> = SampleQueue::new();

pub struct UartQueue;

impl UartQueue
{
    #[inline]
    #[allow(static_mut_refs)]
    pub fn allocate(sample_handle: *mut UART_HandleTypeDef) -> RetValue<&'static mut Uart>
    {
        unsafe { UART_QUEUE.allocate(&Uart::new(sample_handle)?) }
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn clean(sample_handle: *mut UART_HandleTypeDef)
    {
        NonNull::new(sample_handle).map(|handle| unsafe { UART_QUEUE.clean(handle) });
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn search(sample_handle: *mut UART_HandleTypeDef) -> RetValue<&'static Uart>
    {
        unsafe { UART_QUEUE.search(NonNull::new(sample_handle).ok_or(ErrValue::Param)?) }
    }
}

/////////////////////////////////////////////////////////////////////////////
// HAL interrupt callback function implementations
/////////////////////////////////////////////////////////////////////////////

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_UART_TxCpltCallback(uart: *mut UART_HandleTypeDef)
{
    if let Ok(sample) = UartQueue::search(uart)
    {
        sample.event_handle.map(|event_handle| event_handle.on_uart_tx_complete());
    }
}

// #[no_mangle]
// pub unsafe extern "C" fn HAL_UART_TxHalfCpltCallback(uart: *mut UART_HandleTypeDef) {}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_UART_RxCpltCallback(uart: *mut UART_HandleTypeDef)
{
    if let Ok(sample) = UartQueue::search(uart)
    {
        sample.event_handle.map(|event_handle| event_handle.on_uart_rx_size_complete());
    }
}

// #[no_mangle]
// pub extern "C" fn HAL_UART_RxHalfCpltCallback(uart: *mut UART_HandleTypeDef) {}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_UART_ErrorCallback(uart: *mut UART_HandleTypeDef)
{
    if let Ok(sample) = UartQueue::search(uart)
    {
        sample.event_handle.map(|event_handle| event_handle.on_uart_error());
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_UART_AbortCpltCallback(uart: *mut UART_HandleTypeDef)
{
    if let Ok(sample) = UartQueue::search(uart)
    {
        sample.event_handle.map(|event_handle| event_handle.on_uart_abort_complete());
    }
}

// #[no_mangle]
// pub unsafe extern "C" fn HAL_UART_AbortTransmitCpltCallback(uart: *mut UART_HandleTypeDef) {}

// #[no_mangle]
// pub unsafe extern "C" fn HAL_UART_AbortReceiveCpltCallback(uart: *mut UART_HandleTypeDef) {}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_UARTEx_RxEventCallback(uart: *mut UART_HandleTypeDef, size: u16)
{
    if let Ok(sample) = UartQueue::search(uart)
    {
        sample.event_handle.map(|event_handle| event_handle.on_uart_rx_complete(size as u32));
    }
}
