use core::ptr::null;

use sces::value::{ErrValue, RetValue};
use sces::os::semaphore::ISemaphore;

use crate::os::native::*;

pub struct Semaphore
{
    handle: ScesSemaphoreHandle,
}

impl Drop for Semaphore
{
    fn drop(&mut self)
    {
        unsafe { sces_semaphore_delete(self.handle) };
    }
}

impl ISemaphore for Semaphore
{
    fn new(max_count: u32) -> RetValue<Self>
    {
        let handle = unsafe { sces_semaphore_create(null(), max_count) };
        (!handle.is_null()).then_some(Semaphore { handle }).ok_or(ErrValue::InstanceCreateFailure)
    }

    fn take(&self)
    {
        unsafe { sces_semaphore_take(self.handle, SCES_OS_WAIT_FOREVER) };
    }

    fn attempt_take(&self, timeout: u32) -> RetValue<()>
    {
        unsafe { sces_semaphore_take(self.handle, timeout).into() }
    }

    fn release(&self)
    {
        unsafe { sces_semaphore_release(self.handle) };
    }
}
