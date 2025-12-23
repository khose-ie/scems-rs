use sces::os::RTOS;

mod events;
mod message_queue;
mod mutex;
mod native;
mod semaphore;
mod task;
mod timer;

pub struct MWOS;

impl RTOS for MWOS
{
    type Events = events::Events;

    type MessageQueue = message_queue::MessageQueue;

    type Mutex = mutex::Mutex;

    type Semaphore = semaphore::Semaphore;

    type Task = task::Task;

    type Timer = timer::Timer;

    fn state() -> sces::os::OSState
    {
        unsafe { native::sces_os_state().into() }
    }

    fn ticks() -> u32
    {
        unsafe { native::sces_os_tick_count() }
    }

    fn task_count() -> u32
    {
        unsafe { native::sces_os_task_count() }
    }

    fn current_task() -> Self::Task
    {
        unsafe { Self::Task::from(native::sces_os_current_task()) }
    }

    fn switch_next_task()
    {
        unsafe { native::sces_os_yield() };
    }

    fn exit_current_task()
    {
        unsafe { native::sces_os_exit_task() };
    }

    fn delay(time: u32)
    {
        unsafe { native::sces_os_delay(time) };
    }

    fn delay_interval(time: u32)
    {
        unsafe { native::sces_os_delay_interval(time) };
    }
}
