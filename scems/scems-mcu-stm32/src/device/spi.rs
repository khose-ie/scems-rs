use core::ptr::NonNull;

use scems::value::{ErrValue, RetValue};
use scems_mcu::spi::{SpiCtrl, SpiCtrlEvent};
use scems_mcu::EventLaunch;

use crate::device::Handle;
use crate::native::spi::*;
use crate::sample_queue::SampleQueue;
use crate::SPI_COUNT;

pub use crate::native::spi::SPI_HandleTypeDef;

/////////////////////////////////////////////////////////////////////////////
// SPI Class
/////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Spi
{
    handle: NonNull<SPI_HandleTypeDef>,
    event_handle: Option<*const dyn SpiCtrlEvent>,
}

impl Spi
{
    pub fn new(handle: *mut SPI_HandleTypeDef) -> RetValue<Self>
    {
        Ok(Spi { handle: NonNull::new(handle).ok_or(ErrValue::Param)?, event_handle: None })
    }
}

impl Handle<SPI_HandleTypeDef> for Spi
{
    fn handle_value(&self) -> *mut SPI_HandleTypeDef
    {
        self.handle.as_ptr()
    }
}

impl EventLaunch<dyn SpiCtrlEvent> for Spi
{
    fn set_event_agent(&mut self, event_handle: &'static dyn SpiCtrlEvent)
    {
        self.event_handle = Some(event_handle);
    }

    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
    }
}

impl SpiCtrl for Spi
{
    fn transmit(&self, data: &[u8], timeout: u32) -> RetValue<()>
    {
        unsafe {
            HAL_SPI_Transmit(self.handle.as_ptr(), data.as_ptr(), data.len() as u16, timeout).into()
        }
    }

    fn receive(&self, data: &mut [u8], timeout: u32) -> RetValue<()>
    {
        unsafe {
            HAL_SPI_Receive(self.handle.as_ptr(), data.as_ptr(), data.len() as u16, timeout).into()
        }
    }

    fn transmit_receive(&self, tx_data: &[u8], rx_data: &mut [u8], timeout: u32) -> RetValue<()>
    {
        unsafe {
            HAL_SPI_TransmitReceive(
                self.handle.as_ptr(),
                tx_data.as_ptr(),
                rx_data.as_ptr(),
                tx_data.len() as u16,
                timeout,
            )
            .into()
        }
    }

    fn async_transmit(&self, data: &[u8]) -> RetValue<()>
    {
        unsafe {
            HAL_SPI_Transmit_DMA(self.handle.as_ptr(), data.as_ptr(), data.len() as u16).into()
        }
    }

    fn async_receive(&self, data: &mut [u8]) -> RetValue<()>
    {
        unsafe {
            HAL_SPI_Receive_DMA(self.handle.as_ptr(), data.as_ptr(), data.len() as u16).into()
        }
    }

    fn async_transmit_receive(&self, tx_data: &[u8], rx_data: &mut [u8]) -> RetValue<()>
    {
        unsafe {
            HAL_SPI_TransmitReceive_DMA(
                self.handle.as_ptr(),
                tx_data.as_ptr(),
                rx_data.as_ptr(),
                tx_data.len() as u16,
            )
            .into()
        }
    }

    fn abort(&self) -> RetValue<()>
    {
        unsafe { HAL_SPI_Abort_IT(self.handle.as_ptr()).into() }
    }
}

/////////////////////////////////////////////////////////////////////////////
// SPI Queue
/////////////////////////////////////////////////////////////////////////////

static mut SPI_QUEUE: SampleQueue<Spi, SPI_HandleTypeDef, SPI_COUNT> = SampleQueue::new();

pub struct SpiQueue;

impl SpiQueue
{
    #[inline]
    #[allow(static_mut_refs)]
    pub fn alloc(sample_handle: *mut SPI_HandleTypeDef) -> RetValue<&'static mut Spi>
    {
        unsafe { SPI_QUEUE.allocate(&Spi::new(sample_handle)?) }
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn clean(sample_handle: *mut SPI_HandleTypeDef)
    {
        NonNull::new(sample_handle).map(|handle| unsafe { SPI_QUEUE.clean(handle) });
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn search(sample_handle: *mut SPI_HandleTypeDef) -> RetValue<&'static Spi>
    {
        unsafe { SPI_QUEUE.search(NonNull::new(sample_handle).ok_or(ErrValue::Param)?) }
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
