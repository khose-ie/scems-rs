use core::mem::transmute;

use crate::common::result::{ErrValue, RetValue};
use crate::mcu::common::spi::{SpiDevice, SpiDeviceEventAgent};
use crate::mcu::common::EventLaunch;
pub use crate::mcu::vendor::stm::native::spi::SPI_HandleTypeDef;
use crate::mcu::vendor::stm::native::spi::*;
use crate::mcu::vendor::stm::sample_queue::SampleQueue;
use crate::mcu::vendor::stm::{Handle, SPI_COUNT};

/////////////////////////////////////////////////////////////////////////////
// SPI struct
/////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Spi
{
    handle: *mut SPI_HandleTypeDef,
    event_handle: Option<*const dyn SpiDeviceEventAgent>,
}

impl Spi
{
    pub fn new(handle: *mut SPI_HandleTypeDef) -> RetValue<Self>
    {
        if handle.is_null()
        {
            return Err(ErrValue::Param);
        }

        Ok(Spi { handle, event_handle: None })
    }
}

impl Handle<SPI_HandleTypeDef> for Spi
{
    fn handle_value(&self) -> *mut SPI_HandleTypeDef
    {
        self.handle
    }
}

impl EventLaunch<dyn SpiDeviceEventAgent> for Spi
{
    fn set_event_agent(&mut self, event_handle: &dyn SpiDeviceEventAgent)
    {
        self.event_handle = Some(unsafe { transmute(event_handle as *const dyn SpiDeviceEventAgent) });
    }

    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
    }
}

impl SpiDevice for Spi
{
    fn transmit(&self, data: &[u8], timeout: u32) -> RetValue<()>
    {
        unsafe { HAL_SPI_Transmit(self.handle, data.as_ptr(), data.len() as u16, timeout).into() }
    }

    fn receive(&self, data: &mut [u8], timeout: u32) -> RetValue<()>
    {
        unsafe { HAL_SPI_Receive(self.handle, data.as_ptr(), data.len() as u16, timeout).into() }
    }

    fn transmit_receive(&self, tx_data: &[u8], rx_data: &mut [u8], timeout: u32) -> RetValue<()>
    {
        unsafe {
            HAL_SPI_TransmitReceive(self.handle, tx_data.as_ptr(), rx_data.as_ptr(), tx_data.len() as u16, timeout)
                .into()
        }
    }

    fn async_transmit(&self, data: &[u8]) -> RetValue<()>
    {
        unsafe { HAL_SPI_Transmit_DMA(self.handle, data.as_ptr(), data.len() as u16).into() }
    }

    fn async_receive(&self, data: &mut [u8]) -> RetValue<()>
    {
        unsafe { HAL_SPI_Receive_DMA(self.handle, data.as_ptr(), data.len() as u16).into() }
    }

    fn async_transmit_receive(&self, tx_data: &[u8], rx_data: &mut [u8]) -> RetValue<()>
    {
        unsafe {
            HAL_SPI_TransmitReceive_DMA(self.handle, tx_data.as_ptr(), rx_data.as_ptr(), tx_data.len() as u16).into()
        }
    }

    fn abort(&self) -> RetValue<()>
    {
        unsafe { HAL_SPI_Abort_IT(self.handle).into() }
    }
}

/////////////////////////////////////////////////////////////////////////////
// SPI queue
/////////////////////////////////////////////////////////////////////////////

static mut SPI_QUEUE: SampleQueue<Spi, SPI_HandleTypeDef, SPI_COUNT> = SampleQueue::new();

pub struct SpiQueue;

impl SpiQueue
{
    #[inline]
    #[allow(static_mut_refs)]
    pub fn allocate(sample_handle: *mut SPI_HandleTypeDef) -> RetValue<&'static mut Spi>
    {
        unsafe { SPI_QUEUE.allocate(&Spi::new(sample_handle)?) }
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn clean(sample_handle: *mut SPI_HandleTypeDef)
    {
        unsafe { SPI_QUEUE.clean(sample_handle) };
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn search(sample_handle: *mut SPI_HandleTypeDef) -> RetValue<&'static Spi>
    {
        unsafe { SPI_QUEUE.search(sample_handle) }
    }
}

/////////////////////////////////////////////////////////////////////////////
// HAL interrupt callback function implementations
/////////////////////////////////////////////////////////////////////////////

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_SPI_TxCpltCallback(spi: *mut SPI_HandleTypeDef)
{
    if let Some(sample) = SpiQueue::search(spi).ok()
    {
        if let Some(event_handle) = sample.event_handle
        {
            (*event_handle).on_spi_tx_complete();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_SPI_RxCpltCallback(spi: *mut SPI_HandleTypeDef)
{
    if let Some(sample) = SpiQueue::search(spi).ok()
    {
        if let Some(event_handle) = sample.event_handle
        {
            (*event_handle).on_spi_rx_complete();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_SPI_TxRxCpltCallback(spi: *mut SPI_HandleTypeDef)
{
    if let Some(sample) = SpiQueue::search(spi).ok()
    {
        if let Some(event_handle) = sample.event_handle
        {
            (*event_handle).on_spi_tx_rx_complete();
        }
    }
}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_SPI_TxHalfCpltCallback(spi: *mut SPI_HandleTypeDef) {}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_SPI_RxHalfCpltCallback(spi: *mut SPI_HandleTypeDef) {}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_SPI_TxRxHalfCpltCallback(spi: *mut SPI_HandleTypeDef) {}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_SPI_ErrorCallback(spi: *mut SPI_HandleTypeDef)
{
    if let Some(sample) = SpiQueue::search(spi).ok()
    {
        if let Some(event_handle) = sample.event_handle
        {
            (*event_handle).on_spi_error();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_SPI_AbortCpltCallback(spi: *mut SPI_HandleTypeDef)
{
    if let Some(sample) = SpiQueue::search(spi).ok()
    {
        if let Some(event_handle) = sample.event_handle
        {
            (*event_handle).on_spi_abort_complete();
        }
    }
}
