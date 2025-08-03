use crate::common::result::RetValue;

use super::EventLaunch;

pub trait TimBaseDevice
where
    Self: EventLaunch<dyn TimBaseDeviceEventAgent>,
{
    fn activate(&self) -> RetValue<()>;
    fn deactivate(&self);
    fn async_activate(&self) -> RetValue<()>;
    fn async_deactivate(&self);
    fn async_activate_data(&self, data: &mut [u32]) -> RetValue<()>;
    fn async_deactivate_data(&self);
    fn count_value(&self) -> u32;
}

pub trait TimBaseEvent
{
    fn on_tim_base_elapse(&mut self) {}
}

pub trait TimBaseDeviceEventAgent
{
    fn on_tim_base_elapse(&self) {}
}

pub trait TimPwmDevice
where
    Self: EventLaunch<dyn TimPwmDeviceEventAgent>,
{
    fn activate(&self) -> RetValue<()>;
    fn deactivate(&self);
    fn async_activate(&self) -> RetValue<()>;
    fn async_deactivate(&self);
    fn async_activate_data(&self, data: &mut [u32]) -> RetValue<()>;
    fn async_deactivate_data(&self);
    fn count_value(&self) -> u32;
}

pub trait TimPwmEvent
{
    fn on_tim_pwm_finish(&mut self) {}
}

pub trait TimPwmDeviceEventAgent
{
    fn on_tim_pwm_finish(&self) {}
}
