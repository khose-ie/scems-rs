use core::ffi::c_void;
use core::mem::transmute;
use core::ptr::null;

use crate::common::cast::CastOpt;
use crate::common::result::ErrValue;
use crate::common::result::RetValue;
use crate::os::common::timer::TimerEventAgent;
use crate::os::common::timer::{ITimer, TimerMode};
use crate::os::vendors::cmsis_os::cmsis::*;

pub struct Timer
{
    handle: osEventFlagsId_t,
    mode: TimerMode,
    event_agent_handle: TimerEventAgentHandle,
}

impl Timer
{
    pub const fn new(mode: TimerMode, event_agent: &dyn TimerEventAgent) -> Self
    {
        Timer {
            handle: null(),
            mode,
            event_agent_handle: TimerEventAgentHandle::from(unsafe { transmute(event_agent) }),
        }
    }

    #[allow(static_mut_refs)]
    pub unsafe fn func(argument: *mut c_void)
    {
        if let Some(event_agent) = (argument as *mut TimerEventAgentHandle).as_mut()
        {
            if let Some(event_agent) = event_agent.event_agent()
            {
                (*event_agent).on_time_over();
            }
        }
    }
}

impl Drop for Timer
{
    fn drop(&mut self)
    {
        if let Some(x) = self.handle.cast_opt()
        {
            unsafe { osTimerDelete(x) };
        }
    }
}

impl ITimer for Timer
{
    fn start(&mut self, times: u32) -> RetValue<()>
    {
        if let None = self.handle.cast_opt()
        {
            self.handle =
                unsafe { osTimerNew(Timer::func, self.mode.into(), self.event_agent_handle.as_void_ptr(), null()) };
        }

        let x = self.handle.cast_opt().ok_or(ErrValue::InstanceCreate)?;
        unsafe { osTimerStart(x, times).into() }
    }

    fn stop(&mut self)
    {
        if let Some(x) = self.handle.cast_opt()
        {
            unsafe { osTimerStop(x) };
        }
    }

    fn actived(&self) -> bool
    {
        #[rustfmt::skip]
        let Some(x) = self.handle.cast_opt() else { return false; };
        unsafe { osTimerIsRunning(x) != 0 }
    }
}

struct TimerEventAgentHandle
{
    event_agent: *mut dyn TimerEventAgent,
}

impl TimerEventAgentHandle
{
    pub const fn from(event_agent: &dyn TimerEventAgent) -> Self
    {
        Self { event_agent: unsafe { transmute(event_agent) } }
    }

    pub const fn event_agent(&self) -> Option<*const dyn TimerEventAgent>
    {
        if self.event_agent.is_null()
        {
            None
        }
        else
        {
            Some(self.event_agent)
        }
    }

    pub const fn as_void_ptr(&self) -> *mut c_void
    {
        self as *const TimerEventAgentHandle as *mut c_void
    }
}
