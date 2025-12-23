//! SCES OS Abstraction Layer - Native FFI Bindings
//!
//! This module provides Rust FFI bindings for the SCES OS abstraction layer,
//! providing types and function prototypes for various OS services such as tasks,
//! events, message queues, memory pools, mutexes, semaphores, and timers.

#![allow(dead_code)]

use core::ffi::{c_uchar, c_void};

use sces::os::{
    task::{TaskPriority, TaskState},
    timer::TimerState,
    OSState,
};

use crate::native::ScesRetVal;

// ============================================================================
// Constants
// ============================================================================

pub const SCES_OS_WAIT_NO: u32 = 0;
pub const SCES_OS_WAIT_FOREVER: u32 = u32::MAX;

pub const SCES_EVENT_NONE: u32 = 0x00000000;
pub const SCES_EVENT_ALL: u32 = 0xFFFFFFFF;

// ============================================================================
// Handle Types (Opaque Pointers)
// ============================================================================

/// Event handle type - Opaque handle representing an event object
pub type ScesEventHandle = *mut c_void;

/// Message queue handle type - Opaque handle representing a message queue object
pub type ScesMessageQueueHandle = *mut c_void;

/// Memory pool handle type - Opaque handle representing a memory pool object
pub type ScesMemPoolHandle = *mut c_void;

/// Mutex handle type - Opaque handle representing a mutex object
pub type ScesMutexHandle = *mut c_void;

/// Semaphore handle type - Opaque handle representing a semaphore object
pub type ScesSemaphoreHandle = *mut c_void;

/// Task handle type - Opaque handle representing a task control block
pub type ScesTaskHandle = *mut c_void;

/// Timer handle type - Opaque handle representing a timer object
pub type ScesTimerHandle = *mut c_void;

// ============================================================================
// Enumerations
// ============================================================================

/// OS state enumeration - Represents the various states the OS can be in
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScesOsState
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

impl From<ScesOsState> for OSState
{
    fn from(state: ScesOsState) -> Self
    {
        match state
        {
            ScesOsState::Running => OSState::Running,
            ScesOsState::Initializing => OSState::Initializing,
            ScesOsState::Blocked => OSState::Blocked,
            ScesOsState::Suspended => OSState::Suspended,
            ScesOsState::Locked => OSState::Locked,
            ScesOsState::Terminated => OSState::Terminated,
            ScesOsState::ErrInitMem => OSState::ErrInitMem,
            ScesOsState::UnknownErr => OSState::UnknownErr,
        }
    }
}

/// Task state enumeration - Represents the various states a task can be in
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScesTaskState
{
    Inactive = 0,
    Ready = 1,
    Running = 2,
    Blocked = 3,
    Terminated = 4,
    Error = 5,
    Unknown = u32::MAX,
}

impl From<ScesTaskState> for TaskState
{
    fn from(state: ScesTaskState) -> Self
    {
        match state
        {
            ScesTaskState::Inactive => TaskState::Inactive,
            ScesTaskState::Ready => TaskState::Ready,
            ScesTaskState::Running => TaskState::Running,
            ScesTaskState::Blocked => TaskState::Blocked,
            ScesTaskState::Terminated => TaskState::Terminated,
            ScesTaskState::Error => TaskState::Error,
            ScesTaskState::Unknown => TaskState::Unknown,
        }
    }
}

/// Task priority enumeration - Represents the priority levels for tasks
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScesTaskPriority
{
    None = 0,
    Idle = 1,
    Base = 2,
    Low = 3,
    Normal = 4,
    High = 5,
    Privilege = 6,
    Realtime = 7,
}

impl ScesTaskPriority
{
    pub const MAX: ScesTaskPriority = ScesTaskPriority::Realtime;
}

impl From<TaskPriority> for ScesTaskPriority
{
    fn from(priority: TaskPriority) -> Self
    {
        match priority
        {
            TaskPriority::None => ScesTaskPriority::None,
            TaskPriority::Idle => ScesTaskPriority::Idle,
            TaskPriority::Base => ScesTaskPriority::Base,
            TaskPriority::Low => ScesTaskPriority::Low,
            TaskPriority::Normal => ScesTaskPriority::Normal,
            TaskPriority::High => ScesTaskPriority::High,
            TaskPriority::Privilege => ScesTaskPriority::Privilege,
            TaskPriority::RealTime => ScesTaskPriority::Realtime,
        }
    }
}

impl From<ScesTaskPriority> for TaskPriority
{
    fn from(priority: ScesTaskPriority) -> Self
    {
        match priority
        {
            ScesTaskPriority::None => TaskPriority::None,
            ScesTaskPriority::Idle => TaskPriority::Idle,
            ScesTaskPriority::Base => TaskPriority::Base,
            ScesTaskPriority::Low => TaskPriority::Low,
            ScesTaskPriority::Normal => TaskPriority::Normal,
            ScesTaskPriority::High => TaskPriority::High,
            ScesTaskPriority::Privilege => TaskPriority::Privilege,
            ScesTaskPriority::Realtime => TaskPriority::RealTime,
        }
    }
}

/// Timer state enumeration - Represents the various states a timer can be in
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScesTimerState
{
    Idle = 0,
    Active = 1,
    Expired = 2,
    Deleted = 3,
    Error = 4,
    Unknown = u32::MAX,
}

impl From<ScesTimerState> for TimerState
{
    fn from(state: ScesTimerState) -> Self
    {
        match state
        {
            ScesTimerState::Idle => TimerState::Idle,
            ScesTimerState::Active => TimerState::Active,
            ScesTimerState::Expired => TimerState::Expired,
            ScesTimerState::Deleted => TimerState::Deleted,
            ScesTimerState::Error => TimerState::Error,
            ScesTimerState::Unknown => TimerState::Unknown,
        }
    }
}

// ============================================================================
// External C Functions
// ============================================================================

extern "C" {
    // ------------------------------------------------------------------------
    // OS Functions
    // ------------------------------------------------------------------------

    /// Get the current OS state
    pub fn sces_os_state() -> ScesOsState;

    /// Get the current system tick count in milliseconds
    pub fn sces_os_tick_count() -> u32;

    /// Get the number of tasks currently in the system
    pub fn sces_os_task_count() -> u32;

    // /// Suspend the task scheduler
    // /// Returns previous scheduler state to be used for resuming
    // pub fn sces_os_suspend_schedule() -> u32;

    // /// Resume the task scheduler
    // pub fn sces_os_resume_schedule(previous_state: u32);

    /// Get the currently running task
    pub fn sces_os_current_task() -> ScesTaskHandle;

    /// Yield the processor from the current task
    pub fn sces_os_yield();

    /// Delay the current task for a specified time
    pub fn sces_os_delay(ticks: u32);

    /// Delay the current task until a specified time
    pub fn sces_os_delay_interval(ticks: u32);

    /// Exit the current task
    pub fn sces_os_exit_task();

    /// Exit the current task created with static stack allocation
    pub fn sces_os_exit_task_static();

    // ------------------------------------------------------------------------
    // Event Functions
    // ------------------------------------------------------------------------

    /// Create a new event object
    pub fn sces_event_create(name: *const c_uchar) -> ScesEventHandle;

    /// Delete an event object
    pub fn sces_event_delete(event: ScesEventHandle);

    /// Get the name of an event object
    pub fn sces_event_name(event: ScesEventHandle) -> *const c_uchar;

    /// Get the current state of an event object
    pub fn sces_event_state(event: ScesEventHandle) -> u32;

    /// Put event flags
    pub fn sces_event_put(event: ScesEventHandle, flags: u32) -> ScesRetVal;

    /// Wait for event flags
    pub fn sces_event_wait(event: ScesEventHandle, events_value: u32, timeout: u32) -> ScesRetVal;

    /// Clear event flags
    pub fn sces_event_clear(event: ScesEventHandle, events_value: u32);

    /// Wait for event flags and clear them
    pub fn sces_event_wait_and_clear(
        event: ScesEventHandle, events_value: u32, out_events_value: *mut u32, timeout: u32,
    ) -> ScesRetVal;

    // ------------------------------------------------------------------------
    // Message Queue Functions
    // ------------------------------------------------------------------------

    /// Create a new message queue
    pub fn sces_mq_create(
        name: *const c_uchar, message_size: u32, message_count: u32,
    ) -> ScesMessageQueueHandle;

    /// Create a new message queue with static buffer
    pub fn sces_mq_create_static(
        name: *const c_uchar, message_buffer: *mut u8, message_size: u32, message_count: u32,
    ) -> ScesMessageQueueHandle;

    /// Delete a message queue
    pub fn sces_mq_delete(queue: ScesMessageQueueHandle);

    /// Delete a message queue created with static buffer
    pub fn sces_mq_delete_static(queue: ScesMessageQueueHandle);

    /// Get the name of a message queue
    pub fn sces_mq_name(queue: ScesMessageQueueHandle) -> *const c_uchar;

    /// Get the size of a message in the queue
    pub fn sces_mq_message_size(queue: ScesMessageQueueHandle) -> u32;

    /// Get the number of messages currently in the queue
    pub fn sces_mq_message_count(queue: ScesMessageQueueHandle) -> u32;

    /// Get the maximum number of messages the queue can hold
    pub fn sces_mq_max_message_count(queue: ScesMessageQueueHandle) -> u32;

    /// Send a message to the queue
    pub fn sces_mq_send(
        queue: ScesMessageQueueHandle, message: *const c_void, timeout: u32,
    ) -> ScesRetVal;

    /// Receive a message from the queue
    pub fn sces_mq_receive(
        queue: ScesMessageQueueHandle, message: *mut c_void, timeout: u32,
    ) -> ScesRetVal;

    /// Clear all messages from the queue
    pub fn sces_mq_clear(queue: ScesMessageQueueHandle);

    // ------------------------------------------------------------------------
    // Memory Pool Functions
    // ------------------------------------------------------------------------

    /// Create a new memory pool
    pub fn sces_mem_pool_create(
        name: *const c_uchar, block_size: u32, block_count: u32,
    ) -> ScesMemPoolHandle;

    /// Create a new memory pool with static buffer
    pub fn sces_mem_pool_create_static(
        name: *const c_uchar, pool_buffer: *mut u8, block_size: u32, block_count: u32,
    ) -> ScesMemPoolHandle;

    /// Delete a memory pool
    pub fn sces_mem_pool_delete(pool: ScesMemPoolHandle);

    /// Delete a memory pool created with static buffer
    pub fn sces_mem_pool_delete_static(pool: ScesMemPoolHandle);

    /// Get the name of a memory pool
    pub fn sces_mem_pool_name(pool: ScesMemPoolHandle) -> *const c_uchar;

    /// Get the size of each memory block in the pool
    pub fn sces_mem_pool_block_size(pool: ScesMemPoolHandle) -> u32;

    /// Get the number of used memory blocks in the pool
    pub fn sces_mem_pool_block_count(pool: ScesMemPoolHandle) -> u32;

    /// Get the maximum number of memory blocks in the pool
    pub fn sces_mem_pool_max_block_count(pool: ScesMemPoolHandle) -> u32;

    /// Allocate a memory block from the pool
    pub fn sces_mem_pool_alloc(pool: ScesMemPoolHandle, timeout: u32) -> *mut c_void;

    /// Free a memory block back to the pool
    pub fn sces_mem_pool_free(pool: ScesMemPoolHandle, block: *mut c_void);

    // ------------------------------------------------------------------------
    // Mutex Functions
    // ------------------------------------------------------------------------

    /// Create a new mutex
    pub fn sces_mutex_create(name: *const c_uchar) -> ScesMutexHandle;

    /// Delete a mutex
    pub fn sces_mutex_delete(mutex: ScesMutexHandle);

    /// Get the name of a mutex
    pub fn sces_mutex_name(mutex: ScesMutexHandle) -> *const c_uchar;

    /// Get the owner of a mutex
    pub fn sces_mutex_owner(mutex: ScesMutexHandle) -> ScesTaskHandle;

    /// Lock a mutex
    pub fn sces_mutex_lock(mutex: ScesMutexHandle, timeout: u32) -> ScesRetVal;

    /// Unlock a mutex
    pub fn sces_mutex_unlock(mutex: ScesMutexHandle) -> ScesRetVal;

    // ------------------------------------------------------------------------
    // Semaphore Functions
    // ------------------------------------------------------------------------

    /// Create a new semaphore
    pub fn sces_semaphore_create(name: *const c_uchar, max_count: u32) -> ScesSemaphoreHandle;

    /// Delete a semaphore
    pub fn sces_semaphore_delete(semaphore: ScesSemaphoreHandle);

    /// Get the name of a semaphore
    pub fn sces_semaphore_name(semaphore: ScesSemaphoreHandle) -> *const c_uchar;

    /// Get the current count of a semaphore
    pub fn sces_semaphore_count(semaphore: ScesSemaphoreHandle) -> u32;

    /// Take (decrement) a semaphore
    pub fn sces_semaphore_take(semaphore: ScesSemaphoreHandle, timeout: u32) -> ScesRetVal;

    /// Release (increment) a semaphore
    pub fn sces_semaphore_release(semaphore: ScesSemaphoreHandle) -> ScesRetVal;

    // ------------------------------------------------------------------------
    // Task Functions
    // ------------------------------------------------------------------------

    /// Create a new task
    pub fn sces_task_create(
        name: *const c_uchar, main: extern "C" fn(*mut c_void), arg: *mut c_void, stack_size: u32,
        priority: ScesTaskPriority,
    ) -> ScesTaskHandle;

    /// Create a new task with static stack allocation
    pub fn sces_task_create_static(
        name: *const c_uchar, main: extern "C" fn(*mut c_void), arg: *mut c_void, stack: *mut u8,
        stack_size: u32, priority: ScesTaskPriority,
    ) -> ScesTaskHandle;

    /// Delete a task
    pub fn sces_task_delete(task: ScesTaskHandle);

    /// Delete a task created with static stack allocation
    pub fn sces_task_delete_static(task: ScesTaskHandle);

    /// Get the name of a task
    pub fn sces_task_name(task: ScesTaskHandle) -> *const c_uchar;

    /// Get the stack size of a task
    pub fn sces_task_stack_size(task: ScesTaskHandle) -> u32;

    /// Get the priority of a task
    pub fn sces_task_priority(task: ScesTaskHandle) -> ScesTaskPriority;

    /// Get the current state of a task
    pub fn sces_task_state(task: ScesTaskHandle) -> ScesTaskState;

    /// Set the priority of a task
    pub fn sces_task_set_priority(task: ScesTaskHandle, priority: ScesTaskPriority) -> ScesRetVal;

    /// Suspend a task
    pub fn sces_task_suspend(task: ScesTaskHandle) -> ScesRetVal;

    /// Resume a suspended task
    pub fn sces_task_resume(task: ScesTaskHandle) -> ScesRetVal;

    // ------------------------------------------------------------------------
    // Timer Functions
    // ------------------------------------------------------------------------

    /// Create a new one-shot timer
    pub fn sces_timer_create_once(
        name: *const c_uchar, callback: extern "C" fn(*mut c_void), arg: *mut c_void,
    ) -> ScesTimerHandle;

    /// Create a new periodic timer
    pub fn sces_timer_create_periodic(
        name: *const c_uchar, callback: extern "C" fn(*mut c_void), arg: *mut c_void,
    ) -> ScesTimerHandle;

    /// Delete a timer
    pub fn sces_timer_delete(timer: ScesTimerHandle);

    /// Get the name of a timer
    pub fn sces_timer_name(timer: ScesTimerHandle) -> *const c_uchar;

    /// Get the current state of a timer
    pub fn sces_timer_state(timer: ScesTimerHandle) -> ScesTimerState;

    /// Start a timer
    pub fn sces_timer_start(timer: ScesTimerHandle, timeout: u32) -> ScesRetVal;

    /// Stop a timer
    pub fn sces_timer_stop(timer: ScesTimerHandle);
}
