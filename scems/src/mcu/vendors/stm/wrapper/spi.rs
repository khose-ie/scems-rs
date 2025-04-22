use core::mem::transmute;

use crate::common::result::IResult;
use crate::derive::{AsPtr, HandlePtr};
use crate::mcu::common::spi::{Spi, SpiEventAgent};
use crate::mcu::common::{EventLaunch, HandlePtr};
use crate::mcu::vendors::stm::common::DeviceQueue;
use crate::mcu::vendors::stm::native::spi::*;

const SPI_COUNT: usize = 8;
static mut SPIS: DeviceQueue<SPI, SpiDevice, SPI_COUNT> = DeviceQueue::new();

#[derive(AsPtr, HandlePtr)]
pub struct SpiDevice
{
    handle: *mut SPI,
    event_handle: Option<*const dyn SpiEventAgent>,
}

impl SpiDevice
{
    pub fn new(handle: *mut SPI) -> Self
    {
        SpiDevice { handle, event_handle: None }
    }
}

impl Drop for SpiDevice
{
    fn drop(&mut self)
    {
        self.clean_event_agent();
    }
}

impl EventLaunch<dyn SpiEventAgent> for SpiDevice
{
    #[allow(static_mut_refs)]
    fn set_event_agent(&mut self, event_handle: &dyn SpiEventAgent) -> IResult<()>
    {
        self.event_handle = Some(unsafe { transmute(event_handle as *const dyn SpiEventAgent) });
        unsafe { SPIS.alloc(self.as_ptr()) }
    }

    #[allow(static_mut_refs)]
    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
        unsafe { SPIS.clean(self.as_ptr()) };
    }
}

impl Spi for SpiDevice
{
    fn transmit(&self, data: &[u8], timeout: u32) -> IResult<()>
    {
        unsafe { HAL_SPI_Transmit(self.handle, data.as_ptr(), data.len() as u16, timeout).into() }
    }

    fn receive(&self, data: &mut [u8], timeout: u32) -> IResult<()>
    {
        unsafe { HAL_SPI_Receive(self.handle, data.as_ptr(), data.len() as u16, timeout).into() }
    }

    fn transmit_receive(&self, tx_data: &[u8], rx_data: &mut [u8], timeout: u32) -> IResult<()>
    {
        unsafe {
            HAL_SPI_TransmitReceive(self.handle, tx_data.as_ptr(), rx_data.as_ptr(), tx_data.len() as u16, timeout)
                .into()
        }
    }

    fn async_transmit(&self, data: &[u8]) -> IResult<()>
    {
        unsafe { HAL_SPI_Transmit_DMA(self.handle, data.as_ptr(), data.len() as u16).into() }
    }

    fn async_receive(&self, data: &mut [u8]) -> IResult<()>
    {
        unsafe { HAL_SPI_Receive_DMA(self.handle, data.as_ptr(), data.len() as u16).into() }
    }

    fn async_transmit_receive(&self, tx_data: &[u8], rx_data: &mut [u8]) -> IResult<()>
    {
        unsafe {
            HAL_SPI_TransmitReceive_DMA(self.handle, tx_data.as_ptr(), rx_data.as_ptr(), tx_data.len() as u16).into()
        }
    }

    fn abort(&self) -> IResult<()>
    {
        unsafe { HAL_SPI_Abort_IT(self.handle).into() }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_SPI_TxCpltCallback(spi: *mut SPI)
{
    if let Some(sample) = SPIS.find(spi).ok()
    {
        if let Some(event_handle) = (*sample).event_handle
        {
            (*event_handle).on_spi_tx_complete();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_SPI_RxCpltCallback(spi: *mut SPI)
{
    if let Some(sample) = SPIS.find(spi).ok()
    {
        if let Some(event_handle) = (*sample).event_handle
        {
            (*event_handle).on_spi_rx_complete();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_SPI_TxRxCpltCallback(spi: *mut SPI)
{
    if let Some(sample) = SPIS.find(spi).ok()
    {
        if let Some(event_handle) = (*sample).event_handle
        {
            (*event_handle).on_spi_tx_rx_complete();
        }
    }
}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_SPI_TxHalfCpltCallback(spi: *mut SPI) {}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_SPI_RxHalfCpltCallback(spi: *mut SPI) {}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_SPI_TxRxHalfCpltCallback(spi: *mut SPI) {}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_SPI_ErrorCallback(spi: *mut SPI)
{
    if let Some(sample) = SPIS.find(spi).ok()
    {
        if let Some(event_handle) = (*sample).event_handle
        {
            (*event_handle).on_spi_error();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_SPI_AbortCpltCallback(spi: *mut SPI)
{
    if let Some(sample) = SPIS.find(spi).ok()
    {
        if let Some(event_handle) = (*sample).event_handle
        {
            (*event_handle).on_spi_abort_complete();
        }
    }
}
