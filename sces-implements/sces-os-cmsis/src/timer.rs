use core::ops::Not;
use core::ptr::null;

use sces::value::ErrValue;
use sces::value::RetValue;
use sces_os::timer::{ITimer, ITimerEvent, TimerEventAgent, TimerMode};

use crate::native::*;

pub struct Timer
{
    handle: osEventFlagsId_t,
    mode: TimerMode,
    agent: TimerEventAgent,
}

impl Drop for Timer
{
    fn drop(&mut self)
    {
        self.handle.is_null().not().then(|| unsafe { osTimerDelete(self.handle) });
    }
}

impl ITimer for Timer
{
    fn new(mode: TimerMode) -> RetValue<Self>
    where
        Self: Sized,
    {
        Ok(Timer { handle: null(), mode, agent: TimerEventAgent::new() })
    }

    fn active(&mut self, times: u32, event: &dyn ITimerEvent) -> RetValue<()>
    {
        if self.handle.is_null()
        {
            self.agent.set_event(event);
            self.handle = unsafe {
                osTimerNew(Timer::on_time_over, self.mode.into(), self.agent.as_ptr(), null())
            };
        }

        if self.handle.is_null()
        {
            return Err(ErrValue::InstanceCreateFailure);
        }

        unsafe { osTimerStart(self.handle, times).into() }
    }

    fn terminate(&mut self)
    {
        if self.handle.is_null().not()
        {
            unsafe { osTimerStop(self.handle) };
        }
    }

    fn mode(&self) -> TimerMode
    {
        self.mode
    }

    fn state(&self) -> sces_os::timer::TimerState
    {
        if unsafe { osTimerIsRunning(self.handle) } == 0
        {
            sces_os::timer::TimerState::Idle
        }
        else
        {
            sces_os::timer::TimerState::Active
        }
    }
}
