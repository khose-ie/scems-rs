use core::ptr::null;

use crate::common::cast::CastOpt;
use crate::common::result::{IError, IResult};
use crate::os::common::semaphore::ISemaphore;
use crate::os::vendors::cmsis_os::cmsis::*;

pub struct Semaphore
{
    handle: osSemaphoreId_t,
}

impl Semaphore
{
    pub fn new(max_count: u32) -> IResult<Self>
    {
        let handle = unsafe { osSemaphoreNew(max_count, 0, null()).cast_opt().ok_or(IError::InstanceCreate) }?;
        Ok(Semaphore { handle })
    }
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
    fn take(&self)
    {
        unsafe { osSemaphoreAcquire(self.handle, osWaitForever) };
    }

    fn back(&self)
    {
        unsafe { osSemaphoreRelease(self.handle) };
    }

    fn attempt_take(&self, timeout: u32) -> IResult<()>
    {
        unsafe { osSemaphoreAcquire(self.handle, timeout).into() }
    }
}
