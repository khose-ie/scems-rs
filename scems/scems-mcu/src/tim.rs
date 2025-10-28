use super::EventLaunch;
use scems::value::RetValue;

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

pub struct TimBaseDevice
{
    instance: *mut dyn TimBaseCtrl,
}

impl TimBaseDevice
{
    pub const fn new(instance: &'static mut dyn TimBaseCtrl) -> Self
    {
        Self { instance }
    }
}

impl AsRef<dyn TimBaseCtrl> for TimBaseDevice
{
    fn as_ref(&self) -> &'static dyn TimBaseCtrl
    {
        unsafe { &*self.instance }
    }
}

impl AsMut<dyn TimBaseCtrl> for TimBaseDevice
{
    fn as_mut(&mut self) -> &'static mut dyn TimBaseCtrl
    {
        unsafe { &mut *self.instance }
    }
}

pub struct TimPwmDevice
{
    instance: *mut dyn TimPwmCtrl,
}

impl TimPwmDevice
{
    pub const fn new(instance: &'static mut dyn TimPwmCtrl) -> Self
    {
        Self { instance }
    }
}

impl AsRef<dyn TimPwmCtrl> for TimPwmDevice
{
    fn as_ref(&self) -> &'static dyn TimPwmCtrl
    {
        unsafe { &*self.instance }
    }
}

impl AsMut<dyn TimPwmCtrl> for TimPwmDevice
{
    fn as_mut(&mut self) -> &'static mut dyn TimPwmCtrl
    {
        unsafe { &mut *self.instance }
    }
}
