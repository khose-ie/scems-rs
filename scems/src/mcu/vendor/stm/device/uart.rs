//! The wrapper struct packed interfaces of STM32 HAL libraries to operate the UART peripheral.

use core::mem::transmute;

use crate::common::result::RetValue;
use crate::derive::{AsPtr, HandlePtr};
use crate::mcu::common::uart::{UartDevice, UartDeviceEventAgent};
use crate::mcu::common::{EventLaunch, HandlePtr};
use crate::mcu::vendor::stm::device_queue::{DeviceQueue, SampleQueue};
use crate::mcu::vendor::stm::native::uart::*;

pub use crate::mcu::vendor::stm::native::uart::UART_HandleTypeDef;
use crate::mcu::vendor::PeriphDevice;

const UART_COUNT: usize = 8;
static mut UARTS: DeviceQueue<UART_HandleTypeDef, Uart, UART_COUNT> = DeviceQueue::new();

/// The top encapsulation which be used to operate the STM32 UART peripheral.
///
/// This struct implements the trait [crate::mcu::common::uart::Uart], it will call STM32 HAL
/// functions to meet the specification of interface of the trait.
///
/// Please attention, create this struct will not initialize the peripheral, the initialization
/// should be done in a low-level code, and in the startup time of MCU.
/// For a initialized peripheral, you can create a this struct with it as the handle, and use
/// this struct to operate it.
#[derive(AsPtr, HandlePtr, Clone, Copy)]
pub struct Uart
{
    handle: *mut UART_HandleTypeDef,
    event_handle: Option<*const dyn UartDeviceEventAgent>,
}

impl PeriphDevice<UART_HandleTypeDef> for Uart
{
    fn new(handle: *mut UART_HandleTypeDef) -> Self
    {
        Uart { handle, event_handle: None }
    }

    fn handle_value(&self) -> *mut UART_HandleTypeDef
    {
        self.handle
    }
}

impl EventLaunch<dyn UartDeviceEventAgent> for Uart
{
    #[allow(static_mut_refs)]
    fn set_event_agent(&mut self, event_handle: &dyn UartDeviceEventAgent) -> RetValue<()>
    {
        self.event_handle = unsafe { Some(transmute(event_handle as *const dyn UartDeviceEventAgent)) };
        Ok(())
    }

    #[allow(static_mut_refs)]
    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
    }
}

impl UartDevice for Uart
{
    fn transmit(&self, data: &[u8], timeout: u32) -> RetValue<()>
    {
        unsafe { HAL_UART_Transmit(self.handle, data.as_ptr(), data.len() as u16, timeout).into() }
    }

    fn receive(&self, data: &mut [u8], timeout: u32) -> RetValue<u32>
    {
        let mut size: u16 = 0;
        unsafe { HAL_UARTEx_ReceiveToIdle(self.handle, data.as_ptr(), data.len() as u16, &mut size, timeout).ok()? };
        Ok(size as u32)
    }

    fn receive_size(&self, data: &mut [u8], timeout: u32) -> RetValue<()>
    {
        unsafe { HAL_UART_Receive(self.handle, data.as_ptr(), data.len() as u16, timeout).into() }
    }

    fn async_transmit(&self, data: &[u8]) -> RetValue<()>
    {
        unsafe { HAL_UART_Transmit_DMA(self.handle, data.as_ptr(), data.len() as u16).into() }
    }

    fn async_receive(&self, data: &mut [u8]) -> RetValue<()>
    {
        unsafe { HAL_UARTEx_ReceiveToIdle_DMA(self.handle, data.as_ptr(), data.len() as u16).into() }
    }

    fn async_receive_size(&self, data: &mut [u8]) -> RetValue<()>
    {
        unsafe { HAL_UART_Receive_DMA(self.handle, data.as_ptr(), data.len() as u16).into() }
    }

    fn abort(&self) -> RetValue<()>
    {
        unsafe { HAL_UART_Abort(self.handle).into() }
    }
}

pub static mut UART_DEVICE_QUEUE: SampleQueue<Uart, UART_HandleTypeDef, UART_COUNT> = SampleQueue::new();

pub struct UartDeviceQueue;

impl UartDeviceQueue
{
    #[inline]
    #[allow(static_mut_refs)]
    pub fn allocate(sample_handle: *mut UART_HandleTypeDef) -> RetValue<&'static mut Uart>
    {
        unsafe { UART_DEVICE_QUEUE.allocate(sample_handle) }
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn clean(sample_handle: *mut UART_HandleTypeDef)
    {
        unsafe { UART_DEVICE_QUEUE.clean(sample_handle) };
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn search(sample_handle: *mut UART_HandleTypeDef) -> RetValue<&'static Uart>
    {
        unsafe { UART_DEVICE_QUEUE.search(sample_handle) }
    }

}





#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_UART_TxCpltCallback(uart: *mut UART_HandleTypeDef)
{
    if let Some(sample) = UARTS.find(uart).ok()
    {
        if let Some(event_handle) = (*sample).event_handle
        {
            (*event_handle).on_uart_tx_complete();
        }
    }
}

// #[no_mangle]
// pub unsafe extern "C" fn HAL_UART_TxHalfCpltCallback(uart: *mut UART_HandleTypeDef) {}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_UART_RxCpltCallback(uart: *mut UART_HandleTypeDef)
{
    if let Some(sample) = UARTS.find(uart).ok()
    {
        if let Some(event_handle) = (*sample).event_handle
        {
            (*event_handle).on_uart_rx_size_complete();
        }
    }
}

// #[no_mangle]
// pub extern "C" fn HAL_UART_RxHalfCpltCallback(uart: *mut UART_HandleTypeDef) {}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_UART_ErrorCallback(uart: *mut UART_HandleTypeDef)
{
    if let Some(sample) = UARTS.find(uart).ok()
    {
        if let Some(event_handle) = (*sample).event_handle
        {
            (*event_handle).on_uart_error();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_UART_AbortCpltCallback(uart: *mut UART_HandleTypeDef)
{
    if let Some(sample) = UARTS.find(uart).ok()
    {
        if let Some(event_handle) = (*sample).event_handle
        {
            (*event_handle).on_uart_abort_complete();
        }
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
    if let Some(sample) = UARTS.find(uart).ok()
    {
        if let Some(event_handle) = (*sample).event_handle
        {
            (*event_handle).on_uart_rx_complete(size as u32);
        }
    }
}
