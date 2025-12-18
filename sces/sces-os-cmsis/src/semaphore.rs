use core::{ops::Not, ptr::null};

use sces::value::{ErrValue, RetValue};
use sces_os::semaphore::ISemaphore;

use crate::native::*;

pub struct Semaphore
{
    handle: osSemaphoreId_t,
}

impl Drop for Semaphore
{
    fn drop(&mut self)
    {
        unsafe { osSemaphoreDelete(self.handle) };
    }
}

impl ISemaphore for Semaphore
{
    fn new(max_count: u32) -> RetValue<Self>
    {
        let handle = unsafe { osSemaphoreNew(max_count, 0, null()) };
        handle.is_null().not().then_some(handle).ok_or(ErrValue::InstanceCreateFailure)?;
        Ok(Semaphore { handle })
    }

    fn take(&self)
    {
        unsafe { osSemaphoreAcquire(self.handle, osWaitForever) };
    }

    fn back(&self)
    {
        unsafe { osSemaphoreRelease(self.handle) };
    }

    fn attempt_take(&self, timeout: u32) -> RetValue<()>
    {
        unsafe { osSemaphoreAcquire(self.handle, timeout).into() }
    }
}
