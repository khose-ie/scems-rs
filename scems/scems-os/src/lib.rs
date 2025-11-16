#![no_std]

extern crate alloc;

pub mod events;
pub mod kernel;
pub mod mem;
pub mod message_queue;
pub mod mutex;
pub mod semaphore;
pub mod sxmutex;
pub mod task;
pub mod timer;

pub trait RTOS
{
    const WAIT_NO: u32 = 0;
    const WAIT_100MS: u32 = 0;
    const WAIT_FOREVER: u32 = 0xFFFFFFFF;

    type Events: events::IEvents;
    type MessageQueue: message_queue::IMessageQueue;
    type Mutex: mutex::IMutex;
    type Semaphore: semaphore::ISemaphore;
    type Task: task::ITask;
    type Timer: timer::ITimer;

    fn delay(time: u32);
    fn systick() -> u32;
    fn switch_out();
}
