use core::mem::transmute;

use crate::common::result::{ErrValue, RetValue};
use crate::mcu::common::adc::{AdcCtrl, AdcDeviceEventAgent};
use crate::mcu::common::EventLaunch;
pub use crate::mcu::vendor::stm::native::adc::ADC_HandleTypeDef;
use crate::mcu::vendor::stm::native::adc::*;
use crate::mcu::vendor::stm::sample_queue::SampleQueue;
use crate::mcu::vendor::stm::{Handle, ADC_COUNT};

const ADC_DEF_TIMEOUT: u32 = 1000;

/////////////////////////////////////////////////////////////////////////////
// ADC struct
/////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Adc
{
    handle: *mut ADC_HandleTypeDef,
    event_handle: Option<*const dyn AdcDeviceEventAgent>,
}

impl Adc
{
    fn new(handle: *mut ADC_HandleTypeDef) -> RetValue<Self>
    {
        if handle.is_null()
        {
            return Err(ErrValue::Param);
        }

        Ok(Adc { handle, event_handle: None })
    }
}

impl Handle<ADC_HandleTypeDef> for Adc
{
    fn handle_value(&self) -> *mut ADC_HandleTypeDef
    {
        self.handle
    }
}

impl EventLaunch<dyn AdcDeviceEventAgent> for Adc
{
    fn set_event_agent(&mut self, event_handle: &'static dyn AdcDeviceEventAgent)
    {
        self.event_handle = Some(unsafe { transmute(event_handle as *const dyn AdcDeviceEventAgent) });
    }

    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
    }
}

impl AdcCtrl for Adc
{
    fn convert_once(&self) -> RetValue<u32>
    {
        unsafe {
            HAL_ADC_Start(self.handle).ok()?;
            HAL_ADC_PollForConversion(self.handle, ADC_DEF_TIMEOUT).ok()?;
            Ok(HAL_ADC_GetValue(self.handle))
        }
    }

    fn async_convert_once(&self) -> RetValue<()>
    {
        unsafe { HAL_ADC_Start_IT(self.handle).into() }
    }

    fn async_convert_continuous(&self, data: &mut [u32]) -> RetValue<()>
    {
        unsafe { HAL_ADC_Start_DMA(self.handle, data.as_mut_ptr(), data.len() as u32).into() }
    }

    fn async_terminate_conversion(&self) -> RetValue<()>
    {
        unsafe { HAL_ADC_Stop_DMA(self.handle).into() }
    }
}

/////////////////////////////////////////////////////////////////////////////
// ADC queue
/////////////////////////////////////////////////////////////////////////////

static mut ADC_QUEUE: SampleQueue<Adc, ADC_HandleTypeDef, ADC_COUNT> = SampleQueue::new();

pub struct AdcQueue;

impl AdcQueue
{
    #[inline]
    #[allow(static_mut_refs)]
    pub fn allocate(sample_handle: *mut ADC_HandleTypeDef) -> RetValue<&'static mut Adc>
    {
        unsafe { ADC_QUEUE.allocate(&Adc::new(sample_handle)?) }
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn clean(sample_handle: *mut ADC_HandleTypeDef)
    {
        unsafe { ADC_QUEUE.clean(sample_handle) };
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn search(sample_handle: *mut ADC_HandleTypeDef) -> RetValue<&'static Adc>
    {
        unsafe { ADC_QUEUE.search(sample_handle) }
    }
}

/////////////////////////////////////////////////////////////////////////////
// HAL interrupt callback function implementations
/////////////////////////////////////////////////////////////////////////////

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_ADC_ConvCpltCallback(adc: *mut ADC_HandleTypeDef)
{
    if let Some(sample) = AdcQueue::search(adc).ok()
    {
        if let Some(event_handle) = sample.event_handle
        {
            (*event_handle).on_adc_convert_once_complete(HAL_ADC_GetValue(adc));
        }
    }
}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_ADC_ConvHalfCpltCallback(adc: *mut ADC_HandleTypeDef) {}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_ADC_LevelOutOfWindowCallback(adc: *mut ADC_HandleTypeDef)
{
    if let Some(sample) = AdcQueue::search(adc).ok()
    {
        if let Some(event_handle) = sample.event_handle
        {
            (*event_handle).on_adc_level_out_of_window();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_ADC_ErrorCallback(adc: *mut ADC_HandleTypeDef)
{
    if let Some(sample) = AdcQueue::search(adc).ok()
    {
        if let Some(event_handle) = sample.event_handle
        {
            (*event_handle).on_adc_error();
        }
    }
}
