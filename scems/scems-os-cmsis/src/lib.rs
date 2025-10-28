#![no_std]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod native;

pub mod events;
pub mod kernel;
pub mod mem;
pub mod message_queue;
pub mod mutex;
pub mod semaphore;
pub mod sxmutex;
pub mod task;
pub mod timer;

use scems_os::{kernel::IKernel, OS};

pub const COMMON_TASK_TICK: u32 = 500;

pub struct CMSISOS;

impl OS for CMSISOS
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
        kernel::Kernel::delay(time);
    }

    #[inline]
    fn systick() -> u32
    {
        kernel::Kernel::systick_value()
    }

    #[inline]
    fn switch_out()
    {
        kernel::Kernel::cede();
    }
}
