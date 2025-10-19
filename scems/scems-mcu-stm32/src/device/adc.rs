use core::ptr::NonNull;

use scems::value::{ErrValue, RetValue};
use scems_mcu::adc::{AdcCtrl, AdcCtrlEvent};
use scems_mcu::EventLaunch;

use crate::device::Handle;
use crate::native::adc::*;
use crate::sample_queue::SampleQueue;
use crate::ADC_COUNT;

pub use crate::native::adc::ADC_HandleTypeDef;

const ADC_DEF_TIMEOUT: u32 = 1000;

/////////////////////////////////////////////////////////////////////////////
// ADC Class
/////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Adc
{
    handle: NonNull<ADC_HandleTypeDef>,
    event_handle: Option<&'static dyn AdcCtrlEvent>,
}

impl Adc
{
    fn new(handle: *mut ADC_HandleTypeDef) -> RetValue<Self>
    {
        Ok(Adc { handle: NonNull::new(handle).ok_or(ErrValue::Param)?, event_handle: None })
    }
}

impl Handle<ADC_HandleTypeDef> for Adc
{
    fn handle_value(&self) -> *mut ADC_HandleTypeDef
    {
        self.handle.as_ptr()
    }
}

impl EventLaunch<dyn AdcCtrlEvent> for Adc
{
    fn set_event_agent(&mut self, event_handle: &'static dyn AdcCtrlEvent)
    {
        self.event_handle = Some(event_handle);
    }

    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
    }
}

impl AdcCtrl for Adc
{
    fn convert(&self) -> RetValue<u32>
    {
        let handle = self.handle.as_ptr();

        unsafe { HAL_ADC_Start(handle).ok()? };
        unsafe { HAL_ADC_PollForConversion(handle, ADC_DEF_TIMEOUT).ok()? };
        unsafe { Ok(HAL_ADC_GetValue(handle)) }
    }

    fn async_convert(&self) -> RetValue<()>
    {
        unsafe { HAL_ADC_Start_IT(self.handle.as_ptr()).into() }
    }

    fn async_convert_continuous(&self, data: &mut [u32]) -> RetValue<()>
    {
        unsafe {
            HAL_ADC_Start_DMA(self.handle.as_ptr(), data.as_mut_ptr(), data.len() as u32).into()
        }
    }

    fn async_terminate_conversion(&self) -> RetValue<()>
    {
        unsafe { HAL_ADC_Stop_DMA(self.handle.as_ptr()).into() }
    }

    fn value(&self) -> u32
    {
        unsafe { HAL_ADC_GetValue(self.handle.as_ptr()) }
    }
}

/////////////////////////////////////////////////////////////////////////////
// ADC Queue
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
        NonNull::new(sample_handle).map(|handle| unsafe { ADC_QUEUE.clean(handle) });
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn search(sample_handle: *mut ADC_HandleTypeDef) -> RetValue<&'static Adc>
    {
        unsafe { ADC_QUEUE.search(NonNull::new(sample_handle).ok_or(ErrValue::Param)?) }
    }
}

/////////////////////////////////////////////////////////////////////////////
// HAL interrupt callback function implementations
/////////////////////////////////////////////////////////////////////////////

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_ADC_ConvCpltCallback(adc: *mut ADC_HandleTypeDef)
{
    if let Ok(sample) = AdcQueue::search(adc)
    {
        sample
            .event_handle
            .map(|event_handle| event_handle.on_adc_convert_once_complete(HAL_ADC_GetValue(adc)));
    }
}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_ADC_ConvHalfCpltCallback(adc: *mut ADC_HandleTypeDef) {}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_ADC_LevelOutOfWindowCallback(adc: *mut ADC_HandleTypeDef)
{
    if let Ok(sample) = AdcQueue::search(adc)
    {
        sample.event_handle.map(|event_handle| event_handle.on_adc_level_out_of_window());
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_ADC_ErrorCallback(adc: *mut ADC_HandleTypeDef)
{
    if let Ok(sample) = AdcQueue::search(adc)
    {
        sample.event_handle.map(|event_handle| event_handle.on_adc_error());
    }
}
