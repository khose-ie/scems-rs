use core::mem::transmute;

use crate::common::result::RetValue;
use crate::derive::{AsPtr, HandlePtr};
use crate::mcu::common::adc::{Adc, AdcEventAgent};
use crate::mcu::common::{EventLaunch, HandlePtr};
use crate::mcu::vendor::stm::common::DeviceQueue;
use crate::mcu::vendor::stm::native::adc::*;

const ADC_DEF_TIMEOUT: u32 = 1000;

const ADC_COUNT: usize = 8;
static mut ADCS: DeviceQueue<ADC_HandleTypeDef, AdcDevice, ADC_COUNT> = DeviceQueue::new();

#[derive(AsPtr, HandlePtr)]
pub struct AdcDevice
{
    handle: *mut ADC_HandleTypeDef,
    event_handle: Option<*const dyn AdcEventAgent>,
}

impl AdcDevice
{
    pub fn new(handle: *mut ADC_HandleTypeDef) -> Self
    {
        AdcDevice { handle, event_handle: None }
    }
}

impl Drop for AdcDevice
{
    fn drop(&mut self)
    {
        self.clean_event_agent();
    }
}

impl EventLaunch<dyn AdcEventAgent> for AdcDevice
{
    #[allow(static_mut_refs)]
    fn set_event_agent(&mut self, event_handle: &dyn AdcEventAgent) -> RetValue<()>
    {
        self.event_handle = Some(unsafe { transmute(event_handle as *const dyn AdcEventAgent) });
        unsafe { ADCS.alloc(self.as_ptr()) }
    }

    #[allow(static_mut_refs)]
    fn clean_event_agent(&mut self)
    {
        if let Some(_) = self.event_handle
        {
            unsafe { ADCS.clean(self.as_ptr()) };
            self.event_handle = None;
        }
    }
}

impl Adc for AdcDevice
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

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_ADC_ConvCpltCallback(adc: *mut ADC_HandleTypeDef)
{
    if let Some(sample) = ADCS.find(adc).ok()
    {
        if let Some(event_handle) = (*sample).event_handle
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
    if let Some(sample) = ADCS.find(adc).ok()
    {
        if let Some(event_handle) = (*sample).event_handle
        {
            (*event_handle).on_adc_level_out_of_window();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_ADC_ErrorCallback(adc: *mut ADC_HandleTypeDef)
{
    if let Some(sample) = ADCS.find(adc).ok()
    {
        if let Some(event_handle) = (*sample).event_handle
        {
            (*event_handle).on_adc_error();
        }
    }
}
