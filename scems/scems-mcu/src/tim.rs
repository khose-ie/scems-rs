use super::EventLaunch;
use scems::value::RetValue;

pub type TimBaseDevice = &'static mut dyn TimBaseCtrl;

pub trait TimBaseCtrl
where
    Self: EventLaunch<dyn TimBaseCtrlEvent>,
{
    fn activate(&self) -> RetValue<()>;
    fn deactivate(&self);
    fn async_activate(&self) -> RetValue<()>;
    fn async_deactivate(&self);
    fn async_activate_data(&self, data: &mut [u32]) -> RetValue<()>;
    fn async_deactivate_data(&self);
    fn count_value(&self) -> u32;
}

pub trait TimBaseCtrlEvent
{
    fn on_tim_base_elapse(&self) {}
}

pub type TimPwmDevice = &'static mut dyn TimPwmCtrl;

pub trait TimPwmCtrl
where
    Self: EventLaunch<dyn TimPwmCtrlEvent>,
{
    fn activate(&self) -> RetValue<()>;
    fn deactivate(&self);
    fn async_activate(&self) -> RetValue<()>;
    fn async_deactivate(&self);
    fn async_activate_data(&self, data: &mut [u32]) -> RetValue<()>;
    fn async_deactivate_data(&self);
    fn count_value(&self) -> u32;
}

pub trait TimPwmCtrlEvent
{
    fn on_tim_pwm_finish(&self) {}
}
