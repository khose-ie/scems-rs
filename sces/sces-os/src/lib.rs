#![no_std]

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

    /// Associated Types for RTOS Components
    /// Events type
    /// Defines the event handling mechanism
    type Events: events::IEvents;

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

    /// Create a delay for the specified time in milliseconds
    /// # Arguments
    /// * `time: u32` - The delay duration in milliseconds
    fn delay(time: u32);

    /// Create a delay until the specified time in milliseconds
    /// # Arguments
    /// * `time: u32` - The target time in milliseconds
    fn delay_interval(time: u32);

    /// Get the current OS tick count
    /// # Returns
    /// * `u32` - The current OS tick count in milliseconds
    fn ostick() -> u32;
}
