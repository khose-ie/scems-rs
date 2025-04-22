use crate::common::result::IResult;

use super::EventLaunch;

pub trait Adc
where
    Self: EventLaunch<dyn AdcEventAgent>,
{
    fn convert_once(&self) -> IResult<u32>;
    fn async_convert_once(&self) -> IResult<()>;
    fn async_convert_continuous_start(&self, data: &mut [u32]) -> IResult<()>;
    fn async_convert_continuous_stop(&self) -> IResult<()>;
}

pub trait AdcEvent
{
    fn on_adc_convert_once_complete(&mut self, _value: u32) {}
    fn on_adc_level_out_of_window(&mut self) {}
    fn on_adc_error(&mut self) {}
}

pub trait AdcEventAgent
{
    fn on_adc_convert_once_complete(&self, _value: u32) {}
    fn on_adc_level_out_of_window(&self) {}
    fn on_adc_error(&self) {}
}
