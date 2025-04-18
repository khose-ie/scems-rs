use crate::common::result::IResult;

pub trait ITimer {
    fn start(&mut self, times: u32) -> IResult<()>;
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
