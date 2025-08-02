use core::mem::transmute;

use crate::common::result::RetValue;
use crate::derive::{AsPtr, HandlePtr};
use crate::mcu::common::uart::{Uart, UartEventAgent};
use crate::mcu::common::{EventLaunch, HandlePtr};
use crate::mcu::vendor::stm::common::DeviceQueue;
use crate::mcu::vendor::stm::native::uart::*;

const UART_COUNT: usize = 8;
static mut UARTS: DeviceQueue<UART_HandleTypeDef, UartDevice, UART_COUNT> = DeviceQueue::new();

#[derive(AsPtr, HandlePtr)]
pub struct UartDevice
{
    handle: *mut UART_HandleTypeDef,
    event_handle: Option<*const dyn UartEventAgent>,
}

impl UartDevice
{
    pub fn new(handle: *mut UART_HandleTypeDef) -> Self
    {
        UartDevice { handle, event_handle: None }
    }
}

impl Drop for UartDevice
{
    fn drop(&mut self)
    {
        self.clean_event_agent();
    }
}

impl EventLaunch<dyn UartEventAgent> for UartDevice
{
    #[allow(static_mut_refs)]
    fn set_event_agent(&mut self, event_handle: &dyn UartEventAgent) -> RetValue<()>
    {
        self.event_handle = Some(unsafe { transmute(event_handle as *const dyn UartEventAgent) });
        unsafe { UARTS.alloc(self.as_ptr()) }
    }

    #[allow(static_mut_refs)]
    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
        unsafe { UARTS.clean(self.as_ptr()) };
    }
}

impl Uart for UartDevice
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
