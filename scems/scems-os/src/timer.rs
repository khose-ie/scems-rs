use scems::value::RetValue;

pub trait ITimer
{
    fn start(&mut self, times: u32) -> RetValue<()>;
    fn stop(&mut self);
    fn actived(&self) -> bool;
}

#[derive(Clone, Copy)]
pub enum TimerMode
{
    Once,
    Periodic,
}

pub trait TimerEvent
{
    fn on_time_over(&mut self) {}
}

pub trait TimerEventAgent
{
    fn on_time_over(&self) {}
}
