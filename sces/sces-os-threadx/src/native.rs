#![allow(dead_code)]
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_schar, c_void};
use core::ptr::null_mut;

/// ThreadX native constants.
pub const TX_NO_WAIT: u32 = 0;
pub const TX_WAIT_FOREVER: u32 = 0xFFFFFFFF;
pub const TX_AND: u32 = 2;
pub const TX_AND_CLEAR: u32 = 3;
pub const TX_OR: u32 = 0;
pub const TX_OR_CLEAR: u32 = 1;
pub const TX_1_ULONG: u32 = 1;
pub const TX_2_ULONG: u32 = 2;
pub const TX_4_ULONG: u32 = 4;
pub const TX_8_ULONG: u32 = 8;
pub const TX_16_ULONG: u32 = 16;
pub const TX_NO_TIME_SLICE: u32 = 0;
pub const TX_AUTO_START: u32 = 1;
pub const TX_DONT_START: u32 = 0;
pub const TX_AUTO_ACTIVATE: u32 = 1;
pub const TX_NO_ACTIVATE: u32 = 0;
pub const TX_TRUE: u32 = 1;
pub const TX_FALSE: u32 = 0;
pub const TX_NULL: *mut c_void = null_mut();
pub const TX_INHERIT: u32 = 1;
pub const TX_NO_INHERIT: u32 = 0;
pub const TX_THREAD_ENTRY: u32 = 0;
pub const TX_THREAD_EXIT: u32 = 1;
pub const TX_NO_SUSPENSIONS: u32 = 0;
pub const TX_NO_MESSAGES: u32 = 0;
pub const TX_EMPTY: u32 = 0;
pub const TX_CLEAR_ID: u32 = 0;
pub const TX_STACK_FILL: u32 = 0xEFEFEFEF;

/// ThreadX thread states.
pub const TX_READY: u32 = 0;
pub const TX_COMPLETED: u32 = 1;
pub const TX_TERMINATED: u32 = 2;
pub const TX_SUSPENDED: u32 = 3;
pub const TX_SLEEP: u32 = 4;
pub const TX_QUEUE_SUSP: u32 = 5;
pub const TX_SEMAPHORE_SUSP: u32 = 6;
pub const TX_EVENT_FLAG: u32 = 7;
pub const TX_BLOCK_MEMORY: u32 = 8;
pub const TX_BYTE_MEMORY: u32 = 9;
pub const TX_IO_DRIVER: u32 = 10;
pub const TX_FILE: u32 = 11;
pub const TX_TCP_IP: u32 = 12;
pub const TX_MUTEX_SUSP: u32 = 13;
pub const TX_PRIORITY_CHANGE: u32 = 14;

/// ThreadX return status codes.
pub const TX_SUCCESS: u32 = 0x00;
pub const TX_DELETED: u32 = 0x01;
pub const TX_POOL_ERROR: u32 = 0x02;
pub const TX_PTR_ERROR: u32 = 0x03;
pub const TX_WAIT_ERROR: u32 = 0x04;
pub const TX_SIZE_ERROR: u32 = 0x05;
pub const TX_GROUP_ERROR: u32 = 0x06;
pub const TX_NO_EVENTS: u32 = 0x07;
pub const TX_OPTION_ERROR: u32 = 0x08;
pub const TX_QUEUE_ERROR: u32 = 0x09;
pub const TX_QUEUE_EMPTY: u32 = 0x0A;
pub const TX_QUEUE_FULL: u32 = 0x0B;
pub const TX_SEMAPHORE_ERROR: u32 = 0x0C;
pub const TX_NO_INSTANCE: u32 = 0x0D;
pub const TX_THREAD_ERROR: u32 = 0x0E;
pub const TX_PRIORITY_ERROR: u32 = 0x0F;
pub const TX_NO_MEMORY: u32 = 0x10;
pub const TX_START_ERROR: u32 = 0x10;
pub const TX_DELETE_ERROR: u32 = 0x11;
pub const TX_RESUME_ERROR: u32 = 0x12;
pub const TX_CALLER_ERROR: u32 = 0x13;
pub const TX_SUSPEND_ERROR: u32 = 0x14;
pub const TX_TIMER_ERROR: u32 = 0x15;
pub const TX_TICK_ERROR: u32 = 0x16;
pub const TX_ACTIVATE_ERROR: u32 = 0x17;
pub const TX_THRESH_ERROR: u32 = 0x18;
pub const TX_SUSPEND_LIFTED: u32 = 0x19;
pub const TX_WAIT_ABORTED: u32 = 0x1A;
pub const TX_WAIT_ABORT_ERROR: u32 = 0x1B;
pub const TX_MUTEX_ERROR: u32 = 0x1C;
pub const TX_NOT_AVAILABLE: u32 = 0x1D;
pub const TX_NOT_OWNED: u32 = 0x1E;
pub const TX_INHERIT_ERROR: u32 = 0x1F;
pub const TX_NOT_DONE: u32 = 0x20;
pub const TX_CEILING_EXCEEDED: u32 = 0x21;
pub const TX_INVALID_CEILING: u32 = 0x22;
pub const TX_FEATURE_NOT_ENABLED: u32 = 0xFF;

pub struct TX_TIMER_INTERNAL {
    // /* Define the timer's remaining ticks and initial ticks.  */
    // ULONG               tx_timer_internal_remaining_ticks;
    // ULONG               tx_timer_internal_initial_ticks;

    // /* Define the pointer to the next timer in the list.  */
    // struct TX_TIMER_INTERNAL_STRUCT
    //                     *tx_timer_internal_next;

    // /* Define the pointer to the associated thread.  This is NULL for
    //    periodic timers.  */
    // struct TX_THREAD_STRUCT
    //                     *tx_timer_internal_thread_ptr;

    // /* Define the timer's internal flags.  This includes whether the
    //    timer is periodic or one-shot.  */
    // UINT                tx_timer_internal_flags;
}

pub struct TX_TIMER {}

#[repr(C)]
pub struct TX_THREAD
{
    tx_thread_id: u32,
    tx_thread_run_count: u32,
    tx_thread_stack_ptr: *mut c_void,
    tx_thread_stack_start: *mut c_void,
    tx_thread_stack_end: *mut c_void,
    tx_thread_stack_size: u32,
    tx_thread_time_slice: u32,
    tx_thread_new_time_slice: u32,

        tx_thread_ready_next: *mut TX_THREAD,
        tx_thread_ready_previous: *mut TX_THREAD,

        tx_thread_name: *mut c_char,
        tx_thread_priority: u32,
        tx_thread_state: u32,
        tx_thread_delayed_suspend: u32,
        tx_thread_suspending: u32,
        tx_thread_preempt_threshold: u32,

            tx_thread_schedule_hook: Option<extern "C" fn(thread_ptr: *mut TX_THREAD, id: u32)>,

        tx_thread_entry: Option<extern "C" fn(id: u32)>,
        tx_thread_entry_parameter: u32,

        tx_thread_timer: TX_TIMER_INTERNAL,

        tx_thread_suspend_cleanup: Option<extern "C" fn(thread_ptr: *mut TX_THREAD, suspension_sequence: u32)>,
        tx_thread_suspend_control_block: *mut c_void,
        tx_thread_suspended_next: *mut TX_THREAD,
        tx_thread_suspended_previous: *mut TX_THREAD,
        tx_thread_suspend_info: u32,
        tx_thread_additional_suspend_info: *mut c_void,
        tx_thread_suspend_option: u32,
        tx_thread_suspend_status: u32,

        tx_thread_created_next: *mut TX_THREAD,
        tx_thread_created_previous: *mut TX_THREAD,

    //     /* Define the third port extension in the thread control block. This
    //        is typically defined to whitespace in tx_port.h.  */
    //     TX_THREAD_EXTENSION_2

    //     /* Define a pointer type for FileX extensions.  */
    // #ifndef TX_NO_FILEX_POINTER
    //     VOID                *tx_thread_filex_ptr;
    // #endif

    //     /* Define the priority inheritance variables. These will be used
    //        to manage priority inheritance changes applied to this thread
    //        as a result of mutex get operations.  */
    //     UINT                tx_thread_user_priority;
    //     UINT                tx_thread_user_preempt_threshold;
    //     UINT                tx_thread_inherit_priority;

    //     /* Define the owned mutex count and list head pointer.  */
    //     UINT                tx_thread_owned_mutex_count;
    //     struct TX_MUTEX_STRUCT
    //                         *tx_thread_owned_mutex_list;

    // #ifdef TX_THREAD_ENABLE_PERFORMANCE_INFO

    //     /* Define the number of times this thread is resumed.  */
    //     ULONG               tx_thread_performance_resume_count;

    //     /* Define the number of times this thread suspends.  */
    //     ULONG               tx_thread_performance_suspend_count;

    //     /* Define the number of times this thread is preempted by calling
    //        a ThreadX API service.  */
    //     ULONG               tx_thread_performance_solicited_preemption_count;

    //     /* Define the number of times this thread is preempted by an
    //        ISR calling a ThreadX API service.  */
    //     ULONG               tx_thread_performance_interrupt_preemption_count;

    //     /* Define the number of priority inversions for this thread.  */
    //     ULONG               tx_thread_performance_priority_inversion_count;

    //     /* Define the last thread pointer to preempt this thread.  */
    //     struct TX_THREAD_STRUCT
    //                         *tx_thread_performance_last_preempting_thread;

    //     /* Define the total number of times this thread was time-sliced.  */
    //     ULONG               tx_thread_performance_time_slice_count;

    //     /* Define the total number of times this thread relinquishes.  */
    //     ULONG               tx_thread_performance_relinquish_count;

    //     /* Define the total number of times this thread had a timeout.  */
    //     ULONG               tx_thread_performance_timeout_count;

    //     /* Define the total number of times this thread had suspension lifted
    //        because of the tx_thread_wait_abort service.  */
    //     ULONG               tx_thread_performance_wait_abort_count;
    // #endif

    //     /* Define the highest stack pointer variable.  */
    //     VOID                *tx_thread_stack_highest_ptr;   /* Stack highest usage pointer  */

    // #ifndef TX_DISABLE_NOTIFY_CALLBACKS

    //     /* Define the application callback routine used to notify the application when
    //        the thread is entered or exits.  */
    //     VOID                (*tx_thread_entry_exit_notify)(struct TX_THREAD_STRUCT *thread_ptr, UINT type);
    // #endif

    //     /* Define the fourth port extension in the thread control block. This
    //        is typically defined to whitespace in tx_port.h.  */
    //     TX_THREAD_EXTENSION_3

    //     /* Define variables for supporting execution profile. */
    //     /* Note that in ThreadX 5.x, user would define TX_ENABLE_EXECUTION_CHANGE_NOTIFY and use TX_THREAD_EXTENSION_3
    //        to define the following two variables.
    //        For Azure RTOS 6, user shall use TX_EXECUTION_PROFILE_ENABLE instead of TX_ENABLE_EXECUTION_CHANGE_NOTIFY,
    //        and SHALL NOT add variables to TX_THREAD_EXTENSION_3. */
    // #if (defined(TX_EXECUTION_PROFILE_ENABLE) && !defined(TX_ENABLE_EXECUTION_CHANGE_NOTIFY))
    //     EXECUTION_TIME              tx_thread_execution_time_total;
    //     EXECUTION_TIME_SOURCE_TYPE  tx_thread_execution_time_last_start;
    // #endif

    //     /* Define suspension sequence number.  This is used to ensure suspension is still valid when
    //        cleanup routine executes.  */
    //     ULONG               tx_thread_suspension_sequence;

    // #if defined(TX_ENABLE_RANDOM_NUMBER_STACK_FILLING) && defined(TX_ENABLE_STACK_CHECKING)

    //     /* Define the random stack fill number. This can be used to detect stack overflow.  */
    //     ULONG               tx_thread_stack_fill_value;
    // #endif

    //     /* Define the user extension field.  This typically is defined
    //        to white space, but some ports of ThreadX may need to have
    //        additional fields in the thread control block.  This is
    //        defined in the file tx_port.h.  */
    //     TX_THREAD_USER_EXTENSION
}

pub struct TX_BLOCK_POOL {}
pub struct TX_BYTE_POOL {}
pub struct TX_EVENT_FLAGS_GROUP {}
pub struct TX_MUTEX {}
pub struct TX_QUEUE {}
pub struct TX_SEMAPHORE {}

#[rustfmt::skip]
unsafe extern "C"
{
    pub fn _txe_thread_create(
        thread_ptr: *mut TX_THREAD, name_ptr: *const c_char,
        entry_function: Option<extern "C" fn(id: u32)>, entry_input: u32, stack_start: *mut c_void,
        stack_size: u32, priority: u32, preempt_threshold: u32, time_slice: u32, auto_start: u32,
        thread_control_block_size: u32,
    ) -> u32;

    pub fn _txe_thread_delete(thread_ptr: *mut TX_THREAD) -> u32;

    pub fn _tx_thread_identify(thread_ptr: *mut TX_THREAD) -> u32;

    pub fn _txe_thread_resume(thread_ptr: *mut TX_THREAD) -> u32;

    pub fn _txe_thread_suspend(thread_ptr: *mut TX_THREAD) -> u32;

    pub fn _txe_thread_priority_change(
        thread_ptr: *mut TX_THREAD, new_priority: u32, old_priority: *mut u32,
    ) -> u32;

    pub fn _txe_thread_time_slice_change(
        thread_ptr: *mut TX_THREAD, new_time_slice: u32, old_time_slice: *mut u32,
    ) -> u32;

    pub fn _txe_thread_info_get(
        thread_ptr: *mut TX_THREAD, name: *mut *mut c_schar, state: *mut u32, run_count: *mut u32,
        priority: *mut u32, preemption_threshold: *mut u32, time_slice: *mut u32,
        next_thread: *mut *mut TX_THREAD, next_suspended_thread: *mut *mut TX_THREAD,
    ) -> u32;

}
