use core::ffi::c_void;
use core::ptr::null;

use crate::common::cast::CastOpt;
use crate::common::result::IError;
use crate::common::result::IResult;
use crate::os::common::timer::{ITimer, TimerEvent, TimerMode};
use crate::os::vendors::cmsis_os::cmsis::*;

pub struct Timer
{
    handle: osEventFlagsId_t,
    mode: TimerMode,
    event_handle_agent: EventHandleAgent,
}

impl Timer
{
    pub const fn new(mode: TimerMode, event_handle: &dyn TimerEvent) -> Self
    {
        Timer { handle: null(), mode, event_handle_agent: EventHandleAgent::from(event_handle) }
    }

    #[allow(static_mut_refs)]
    pub fn func(argument: *mut c_void)
    {
        if let Some(event_handle_agent) = unsafe { (argument as *mut EventHandleAgent).as_mut() }
        {
            if let Some(event_handle) = unsafe { event_handle_agent.event_handle.as_mut() }
            {
                event_handle.on_time_over();
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
    fn start(&mut self, times: u32) -> IResult<()>
    {
        if let None = self.handle.cast_opt()
        {
            self.handle =
                unsafe { osTimerNew(Timer::func, self.mode.into(), self.event_handle_agent.as_void_ptr(), null()) };
        }

        let x = self.handle.cast_opt().ok_or(IError::InstanceCreate)?;
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

struct EventHandleAgent
{
    event_handle: *mut dyn TimerEvent,
}

impl EventHandleAgent
{
    pub const fn from(event_handle: &dyn TimerEvent) -> Self
    {
        Self { event_handle: event_handle as *const dyn TimerEvent as *mut dyn TimerEvent }
    }

    pub const fn as_void_ptr(&self) -> *mut c_void
    {
        self as *const EventHandleAgent as *mut c_void
    }
}
