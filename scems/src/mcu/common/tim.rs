use crate::common::result::IResult;

use super::{AsEventPtr, EventHandle};

pub trait TimBase
where
    Self: EventHandle<dyn TimBaseEventPtr>,
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

pub trait TimBaseEventPtr
where
    Self: TimBaseEvent + AsEventPtr<dyn TimBaseEvent>,
{
}

pub trait TimPwm
where
    Self: EventHandle<dyn TimPwmEventPtr>,
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

pub trait TimPwmEventPtr
where
    Self: TimPwmEvent + AsEventPtr<dyn TimPwmEvent>,
{
}
