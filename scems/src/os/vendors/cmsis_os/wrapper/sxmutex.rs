use core::ptr::null;

use super::delay;
use crate::common::cast::CastOpt;
use crate::common::result::{ErrValue, RetValue};
use crate::os::common::sxmutex::ISxMutex;
use crate::os::vendors::cmsis_os::cmsis::*;

const WAIT_TIME: u32 = 10;

pub struct SxMutex
{
    handle: osMutexId_t,
    in_keep: bool,
    request_keep: bool,
    involve_num: u8,
}

impl SxMutex
{
    pub fn new() -> RetValue<Self>
    {
        let handle = unsafe { osMutexNew(null()).cast_opt().ok_or(ErrValue::InstanceCreate) }?;
        Ok(SxMutex { handle, in_keep: false, request_keep: false, involve_num: 0 })
    }
}

impl Drop for SxMutex
{
    fn drop(&mut self)
    {
        unsafe { osMutexDelete(self.handle) };
    }
}

impl ISxMutex for SxMutex
{
    fn involve(&mut self)
    {
        unsafe { osMutexAcquire(self.handle, osWaitForever) };

        while self.in_keep || self.request_keep
        {
            unsafe { osMutexRelease(self.handle) };
            delay(WAIT_TIME);
            unsafe { osMutexAcquire(self.handle, osWaitForever) };
        }

        while self.involve_num == 0xFF
        {
            unsafe { osMutexRelease(self.handle) };
            delay(WAIT_TIME);
            unsafe { osMutexAcquire(self.handle, osWaitForever) };
        }

        self.involve_num += 1;
        unsafe { osMutexRelease(self.handle) };
    }

    fn leave(&mut self)
    {
        unsafe { osMutexAcquire(self.handle, osWaitForever) };

        if self.in_keep
        {
            self.involve_num = 0;
            unsafe { osMutexRelease(self.handle) };
            return;
        }

        if self.involve_num != 0
        {
            self.involve_num -= 1;
        }

        unsafe { osMutexRelease(self.handle) };
    }

    fn keep(&mut self)
    {
        unsafe { osMutexAcquire(self.handle, osWaitForever) };

        while self.request_keep
        {
            unsafe { osMutexRelease(self.handle) };
            delay(WAIT_TIME);
            unsafe { osMutexAcquire(self.handle, osWaitForever) };
        }

        self.request_keep = true;

        while self.in_keep
        {
            unsafe { osMutexRelease(self.handle) };
            delay(WAIT_TIME);
            unsafe { osMutexAcquire(self.handle, osWaitForever) };
        }

        self.in_keep = true;
        self.request_keep = false;

        unsafe { osMutexRelease(self.handle) };
    }

    fn release(&mut self)
    {
        unsafe { osMutexAcquire(self.handle, osWaitForever) };
        self.in_keep = false;
        unsafe { osMutexRelease(self.handle) };
    }
}
