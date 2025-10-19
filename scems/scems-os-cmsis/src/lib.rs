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

use scems_os::kernel::IKernel;

pub const COMMON_TASK_TICK: u32 = 500;

#[inline]
pub fn delay(time: u32)
{
    kernel::Kernel::delay(time);
}

#[inline]
pub fn delay_interval(time: u32)
{
    kernel::Kernel::delay_interval(time);
}

#[inline]
pub fn systick_value() -> u32
{
    kernel::Kernel::systick_value()
}

#[inline]
pub fn cede()
{
    kernel::Kernel::cede();
}

#[inline]
pub fn exit()
{
    kernel::Kernel::exit();
}
