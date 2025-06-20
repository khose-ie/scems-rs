use crate::common::result::IResult;

use super::EventLaunch;

pub trait TimBase
where
    Self: EventLaunch<dyn TimBaseEventAgent>,
{
    fn activate(&self) -> IResult<()>;
    fn deactivate(&self);
    fn async_activate(&self) -> IResult<()>;
    fn async_deactivate(&self);
    fn async_activate_data(&self, data: &mut [u32]) -> IResult<()>;
    fn async_deactivate_data(&self);
    fn count_value(&self) -> u32;
}

pub trait TimBaseEvent
{
    fn on_tim_base_elapse(&mut self) {}
}

pub trait TimBaseEventAgent
{
    fn on_tim_base_elapse(&self) {}
}

pub trait TimPwm
where
    Self: EventLaunch<dyn TimPwmEventAgent>,
{
    fn activate(&self) -> IResult<()>;
    fn deactivate(&self);
    fn async_activate(&self) -> IResult<()>;
    fn async_deactivate(&self);
    fn async_activate_data(&self, data: &mut [u32]) -> IResult<()>;
    fn async_deactivate_data(&self);
    fn count_value(&self) -> u32;
}

pub trait TimPwmEvent
{
    fn on_tim_pwm_finish(&mut self) {}
}

pub trait TimPwmEventAgent
{
    fn on_tim_pwm_finish(&self) {}
}
