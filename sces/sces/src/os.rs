use crate::value::RetValue;

/// sces OS Library
/// This library provides abstractions and implementations for various RTOS functionalities.
/// It includes modules for events, memory management, message queues, mutexes,
/// semaphores, tasks, and timers.
/// The library is designed to be modular and extensible, allowing for easy integration
/// with different RTOS backends.
/// The core trait `RTOS` defines the interface for RTOS functionalities,
/// enabling consistent API usage across various platforms and architectures.
extern crate alloc;

pub mod events;
pub mod mem;
pub mod message_queue;
pub mod mutex;
pub mod semaphore;
pub mod sxmutex;
pub mod task;
pub mod timer;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OSState
{
    Running = 0,
    Initializing = 1,
    Blocked = 2,
    Suspended = 3,
    Locked = 4,
    Terminated = 5,
    ErrInitMem = 6,
    UnknownErr = u32::MAX,
}

/// Real-Time Operating System (RTOS) Trait
/// Defines the interface for RTOS functionalities
/// This trait includes methods for task management, synchronization primitives, and timing functions
/// Implement this trait for different RTOS backends to provide a consistent API
/// across various platforms and architectures.
/// The trait includes associated types for events, message queues, mutexes, semaphores, tasks, and timers,
/// allowing for flexible and modular implementations.
/// Common wait time constants are also defined for ease of use in task scheduling and synchronization.
pub trait RTOS
{
    /// Common wait time constants
    /// No wait
    const WAIT_0: u32 = 0;

    /// Common wait time constants
    /// 50 milliseconds
    const WAIT_50: u32 = 50;

    /// Common wait time constants
    /// 100 milliseconds
    const WAIT_100: u32 = 100;

    /// Common wait time constants
    /// 200 milliseconds
    const WAIT_200: u32 = 200;

    /// Common wait time constants
    /// 500 milliseconds
    const WAIT_500: u32 = 500;

    /// Common wait time constants
    /// Forever wait
    const WAIT_MAX: u32 = 0xFFFFFFFF;

    /// Common wait time constants
    /// Default wait time
    const WAIT_DEF: u32 = Self::WAIT_200;

    /// Stack size constant for tasks
    /// 1 Kilobyte stack size
    const TASK_STACK_1K: u32 = 1024;

    /// Stack size constant for tasks
    /// 2 Kilobytes stack size
    const TASK_STACK_2K: u32 = 2048;

    /// Stack size constant for tasks
    /// 4 Kilobytes stack size
    const TASK_STACK_4K: u32 = 4096;

    /// Stack size constant for tasks
    /// 8 Kilobytes stack size
    const TASK_STACK_8K: u32 = 8192;

    /// Associated Types for RTOS Components
    /// Events type
    /// Defines the event handling mechanism
    type Events: events::IEvents;

    /// Memory Pool type
    /// Defines the memory pool management mechanism
    type MemPool: mem::IMemPool;

    /// Message Queue type
    /// Defines the message queue mechanism
    type MessageQueue: message_queue::IMessageQueue;

    /// Mutex type
    /// Defines the mutex mechanism
    type Mutex: mutex::IMutex;

    /// Semaphore type
    /// Defines the semaphore mechanism
    type Semaphore: semaphore::ISemaphore;

    /// Task type
    /// Defines the task management mechanism
    type Task: task::ITask;

    /// Timer type
    /// Defines the timer mechanism
    type Timer: timer::ITimer;

    /// Initialize the RTOS
    /// # Returns
    /// * `RetValue<()>` - Result of the initialization
    fn initialize() -> RetValue<()>;

    /// Get the current OS state
    /// # Returns
    /// * `OSState` - The current state of the OS
    fn state() -> OSState;

    /// Get the current OS tick count
    /// # Returns
    /// * `u32` - The current OS tick count in milliseconds
    fn ticks() -> u32;

    /// Get the current number of tasks
    /// # Returns
    /// * `u32` - The current number of tasks in the OS
    fn task_count() -> u32;

    /// Get the handle of the currently running task
    /// # Returns
    /// * `Self::Task` - The handle of the currently running task
    fn current_task() -> Self::Task;

    /// Switch the execution to the next task
    fn switch_next_task();

    /// Exit the currently running task
    /// This function does not return
    fn exit_current_task();

    /// Create a delay for the specified time in milliseconds
    /// # Arguments
    /// * `time: u32` - The delay duration in milliseconds
    fn delay(time: u32);

    /// Create a delay until the specified time in milliseconds
    /// # Arguments
    /// * `time: u32` - The target time in milliseconds
    fn delay_interval(time: u32);
}
