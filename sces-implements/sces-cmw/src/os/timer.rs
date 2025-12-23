use core::ptr::{null, null_mut};

use sces::value::{ErrValue, RetValue};
use sces_os::timer::{ITimer, ITimerEvent, TimerEventAgent, TimerMode, TimerState};

use crate::os::native::*;

pub struct Timer
{
    mode: TimerMode,
    handle: ScesTimerHandle,
    event_agent: TimerEventAgent,
}

impl Drop for Timer
{
    fn drop(&mut self)
    {
        unsafe { sces_timer_delete(self.handle) };
    }
}

impl ITimer for Timer
{
    fn new(mode: TimerMode) -> RetValue<Self>
    where
        Self: Sized,
    {
        Ok(Timer { mode, handle: null_mut(), event_agent: TimerEventAgent::new() })
    }

    fn state(&self) -> TimerState
    {
        unsafe { sces_timer_state(self.handle).into() }
    }

    fn mode(&self) -> TimerMode
    {
        self.mode
    }

    fn active(&mut self, times: u32, event: &dyn ITimerEvent) -> RetValue<()>
    {
        if !self.handle.is_null()
        {
            if unsafe { sces_timer_state(self.handle) } != ScesTimerState::Active
            {
                unsafe { sces_timer_start(self.handle, times) };
                return Ok(());
            }
            else
            {
                return Err(ErrValue::InstanceDuplicate);
            }
        }
        else
        {
            self.event_agent.set_event(event);

            match self.mode
            {
                TimerMode::Once =>
                {
                    self.handle = unsafe {
                        sces_timer_create_once(
                            null(),
                            Timer::on_time_over,
                            self.event_agent.as_ptr(),
                        )
                    };
                }
                TimerMode::Periodic =>
                {
                    self.handle = unsafe {
                        sces_timer_create_periodic(
                            null(),
                            Timer::on_time_over,
                            self.event_agent.as_ptr(),
                        )
                    };
                }
            }

            (!self.handle.is_null()).then_some(()).ok_or(ErrValue::InstanceCreateFailure)?;
            return unsafe { sces_timer_start(self.handle, times).into() };
        }
    }

    fn terminate(&mut self)
    {
        unsafe { sces_timer_stop(self.handle) };
    }
}
