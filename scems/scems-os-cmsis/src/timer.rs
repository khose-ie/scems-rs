use core::ffi::c_void;
use core::mem::transmute;
use core::ops::Not;
use core::ptr::null;

use scems::value::ErrValue;
use scems::value::RetValue;
use scems_os::timer::TimerEvent;
use scems_os::timer::{ITimer, TimerMode};

use crate::native::*;

pub struct Timer
{
    handle: osEventFlagsId_t,
    mode: TimerMode,
    event_agent_handle: TimerEventHandle,
}

impl Timer
{
    pub const fn new(mode: TimerMode, event_agent: &dyn TimerEvent) -> Self
    {
        Timer {
            handle: null(),
            mode,
            event_agent_handle: TimerEventHandle::from(unsafe { transmute(event_agent) }),
        }
    }

    #[allow(static_mut_refs)]
    pub unsafe fn func(argument: *mut c_void)
    {
        if let Some(event_agent) = (argument as *mut TimerEventHandle).as_mut()
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
        self.handle.is_null().not().then(|| unsafe { osTimerDelete(self.handle) });
    }
}

impl ITimer for Timer
{
    fn start(&mut self, times: u32) -> RetValue<()>
    {
        if self.handle.is_null()
        {
            self.handle = unsafe {
                osTimerNew(
                    Timer::func,
                    self.mode.into(),
                    self.event_agent_handle.as_void_ptr(),
                    null(),
                )
            };
        }

        if self.handle.is_null()
        {
            return Err(ErrValue::InstanceCreateFailure);
        }

        unsafe { osTimerStart(self.handle, times).into() }
    }

    fn stop(&mut self)
    {
        if self.handle.is_null().not()
        {
            unsafe { osTimerStop(self.handle) };
        }
    }

    fn actived(&self) -> bool
    {
        if self.handle.is_null()
        {
            return false;
        }

        unsafe { osTimerIsRunning(self.handle) != 0 }
    }
}

struct TimerEventHandle
{
    event_agent: *mut dyn TimerEvent,
}

impl TimerEventHandle
{
    pub const fn from(event_agent: &dyn TimerEvent) -> Self
    {
        Self { event_agent: unsafe { transmute(event_agent) } }
    }

    pub const fn event_agent(&self) -> Option<*const dyn TimerEvent>
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
        self as *const TimerEventHandle as *mut c_void
    }
}
