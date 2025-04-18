use crate::common::result::IResult;

use super::{AsEventPtr, EventHandle};

pub trait Adc
where
    Self: EventHandle<dyn AdcEventPtr>,
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

pub trait AdcEventPtr
where
    Self: AdcEvent + AsEventPtr<dyn AdcEvent>,
{
}
