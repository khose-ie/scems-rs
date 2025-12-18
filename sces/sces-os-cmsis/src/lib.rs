#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod native;

pub mod events;
pub mod mem;
pub mod message_queue;
pub mod mutex;
pub mod semaphore;
pub mod sxmutex;
pub mod task;
pub mod timer;

use crate::native::*;
use sces_os::RTOS;

pub const COMMON_TASK_TICK: u32 = 500;

pub struct CMSISOS;

impl RTOS for CMSISOS
{
    type Events = events::Events;

    type MessageQueue = message_queue::MessageQueue;

    type Mutex = mutex::Mutex;

    type Semaphore = semaphore::Semaphore;

    type Task = task::Task;

    type Timer = timer::Timer;

    #[inline]
    fn delay(time: u32)
    {
        unsafe { osDelay(time) };
    }

    #[inline]
    fn delay_interval(time: u32)
    {
        unsafe { osDelayUntil(time) };
    }

    #[inline]
    fn ostick() -> u32
    {
        unsafe { osKernelGetTickCount() }
    }
}
