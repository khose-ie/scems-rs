use core::ptr::null;

use sces::value::{ErrValue, RetValue};
use sces_os::mutex::IMutex;

use crate::os::native::*;

pub struct Mutex
{
    handle: ScesMutexHandle,
}

impl Drop for Mutex
{
    fn drop(&mut self)
    {
        unsafe { sces_mutex_delete(self.handle) };
    }
}

impl IMutex for Mutex
{
    fn new() -> RetValue<Self>
    {
        let handle = unsafe { sces_mutex_create(null()) };
        (!handle.is_null()).then_some(Mutex { handle }).ok_or(ErrValue::InstanceCreateFailure)
    }

    fn lock(&self)
    {
        unsafe { sces_mutex_lock(self.handle, SCES_OS_WAIT_FOREVER) };
    }

    fn attempt_lock(&self, time: u32) -> RetValue<()>
    {
        unsafe { sces_mutex_lock(self.handle, time).into() }
    }

    fn unlock(&self)
    {
        unsafe { sces_mutex_unlock(self.handle) };
    }
}
