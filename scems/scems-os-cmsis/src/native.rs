#![allow(dead_code)]

use core::ffi::{c_char, c_void, CStr};
use core::ptr::{null, null_mut};

use scems::value::{ErrValue, RetValue};
use scems_derive::EnumAsI32;
use scems_os::task::TaskPriority;
use scems_os::timer::TimerMode;

/// Timeout value.
///< Wait forever timeout value.
pub const osWaitForever: u32 = 0xFFFFFFFF;

// Flags options (\ref osThreadFlagsWait and \ref osEventFlagsWait).
///< Wait for any flag (default).
pub const osFlagsWaitAny: u32 = 0x00000000;
///< Wait for all flags.
pub const osFlagsWaitAll: u32 = 0x00000001;
///< Do not clear flags which have been specified to wait for.
pub const osFlagsNoClear: u32 = 0x00000002;

// Flags errors (returned by osThreadFlagsXxxx and osEventFlagsXxxx).
///< Error indicator.
pub const osFlagsError: u32 = 0x80000000;
///< osError (-1).
pub const osFlagsErrorUnknown: u32 = 0xFFFFFFFF;
///< osErrorTimeout (-2).
pub const osFlagsErrorTimeout: u32 = 0xFFFFFFFE;
///< osErrorResource (-3).
pub const osFlagsErrorResource: u32 = 0xFFFFFFFD;
///< osErrorParameter (-4).
pub const osFlagsErrorParameter: u32 = 0xFFFFFFFC;
///< osErrorISR (-6).
pub const osFlagsErrorISR: u32 = 0xFFFFFFFA;

// Thread attributes (attr_bits in \ref osThreadAttr_t).
///< Thread created in detached mode (default)
pub const osThreadDetached: u32 = 0x00000000;
///< Thread created in joinable mode
pub const osThreadJoinable: u32 = 0x00000001;

// Mutex attributes (attr_bits in \ref osMutexAttr_t).
///< Recursive mutex.
pub const osMutexRecursive: u32 = 0x00000001;
///< Priority inherit protocol.
pub const osMutexPrioInherit: u32 = 0x00000002;
///< Robust mutex.
pub const osMutexRobust: u32 = 0x00000008;

pub type osThreadId_t = *const c_void;

pub type osTimerId_t = *const c_void;

pub type osEventFlagsId_t = *const c_void;

pub type osMutexId_t = *const c_void;

pub type osSemaphoreId_t = *const c_void;

pub type osMemoryPoolId_t = *const c_void;

pub type osMessageQueueId_t = *const c_void;

pub type osThreadFunc_t = unsafe fn(*mut c_void);

pub type osTimerFunc_t = unsafe fn(*mut c_void);

pub type TZ_ModuleId_t = u32;

#[repr(C)]
#[derive(EnumAsI32)]
pub enum osStatus_t
{
    osOK = 0,
    osError = -1,
    osErrorTimeout = -2,
    osErrorResource = -3,
    osErrorParameter = -4,
    osErrorNoMemory = -5,
    osErrorISR = -6,
    osStatusReserved = 0x7FFFFFFF,
}

impl osStatus_t
{
    pub fn ok(self) -> RetValue<()>
    {
        self.into()
    }
}

impl Into<RetValue<()>> for osStatus_t
{
    fn into(self) -> RetValue<()>
    {
        match self
        {
            Self::osOK => Ok(()),
            _ => Err(self.into()),
        }
    }
}

impl Into<ErrValue> for osStatus_t
{
    fn into(self) -> ErrValue
    {
        match self
        {
            Self::osOK => ErrValue::None,
            Self::osError => ErrValue::Param,
            Self::osErrorTimeout => ErrValue::Busy,
            Self::osErrorResource => ErrValue::Overtime,
            Self::osErrorParameter => ErrValue::Param,
            Self::osErrorNoMemory => ErrValue::MemAllocFailure,
            Self::osErrorISR => ErrValue::Permission,
            Self::osStatusReserved => ErrValue::Unknown,
        }
    }
}

#[repr(C)]
pub enum osKernelState_t
{
    osKernelInactive = 0,
    osKernelReady = 1,
    osKernelRunning = 2,
    osKernelLocked = 3,
    osKernelSuspended = 4,
    osKernelError = -1,
    osKernelReserved = 0x7FFFFFFF,
}

#[repr(C)]
#[derive(PartialEq, Eq)]
pub enum osThreadState_t
{
    osThreadInactive = 0,
    osThreadReady = 1,
    osThreadRunning = 2,
    osThreadBlocked = 3,
    osThreadTerminated = 4,
    osThreadError = -1,
    osThreadReserved = 0x7FFFFFFF,
}

#[repr(C)]
pub enum osPriority_t
{
    osPriorityNone = 0,
    osPriorityIdle = 1,
    osPriorityLow = 8,
    osPriorityLow1 = 8 + 1,
    osPriorityLow2 = 8 + 2,
    osPriorityLow3 = 8 + 3,
    osPriorityLow4 = 8 + 4,
    osPriorityLow5 = 8 + 5,
    osPriorityLow6 = 8 + 6,
    osPriorityLow7 = 8 + 7,
    osPriorityBelowNormal = 16,
    osPriorityBelowNormal1 = 16 + 1,
    osPriorityBelowNormal2 = 16 + 2,
    osPriorityBelowNormal3 = 16 + 3,
    osPriorityBelowNormal4 = 16 + 4,
    osPriorityBelowNormal5 = 16 + 5,
    osPriorityBelowNormal6 = 16 + 6,
    osPriorityBelowNormal7 = 16 + 7,
    osPriorityNormal = 24,
    osPriorityNormal1 = 24 + 1,
    osPriorityNormal2 = 24 + 2,
    osPriorityNormal3 = 24 + 3,
    osPriorityNormal4 = 24 + 4,
    osPriorityNormal5 = 24 + 5,
    osPriorityNormal6 = 24 + 6,
    osPriorityNormal7 = 24 + 7,
    osPriorityAboveNormal = 32,
    osPriorityAboveNormal1 = 32 + 1,
    osPriorityAboveNormal2 = 32 + 2,
    osPriorityAboveNormal3 = 32 + 3,
    osPriorityAboveNormal4 = 32 + 4,
    osPriorityAboveNormal5 = 32 + 5,
    osPriorityAboveNormal6 = 32 + 6,
    osPriorityAboveNormal7 = 32 + 7,
    osPriorityHigh = 40,
    osPriorityHigh1 = 40 + 1,
    osPriorityHigh2 = 40 + 2,
    osPriorityHigh3 = 40 + 3,
    osPriorityHigh4 = 40 + 4,
    osPriorityHigh5 = 40 + 5,
    osPriorityHigh6 = 40 + 6,
    osPriorityHigh7 = 40 + 7,
    osPriorityRealtime = 48,
    osPriorityRealtime1 = 48 + 1,
    osPriorityRealtime2 = 48 + 2,
    osPriorityRealtime3 = 48 + 3,
    osPriorityRealtime4 = 48 + 4,
    osPriorityRealtime5 = 48 + 5,
    osPriorityRealtime6 = 48 + 6,
    osPriorityRealtime7 = 48 + 7,
    osPriorityISR = 56,
    osPriorityError = -1,
    osPriorityReserved = 0x7FFFFFFF,
}

impl From<TaskPriority> for osPriority_t
{
    fn from(value: TaskPriority) -> Self
    {
        match value
        {
            TaskPriority::None => osPriority_t::osPriorityNone,
            TaskPriority::Idle => osPriority_t::osPriorityIdle,
            TaskPriority::Base => osPriority_t::osPriorityLow,
            TaskPriority::Low => osPriority_t::osPriorityBelowNormal,
            TaskPriority::Normal => osPriority_t::osPriorityNormal,
            TaskPriority::High => osPriority_t::osPriorityAboveNormal,
            TaskPriority::Privilege => osPriority_t::osPriorityHigh,
            TaskPriority::RealTime => osPriority_t::osPriorityRealtime,
        }
    }
}

/// Timer type.
pub enum osTimerType_t
{
    osTimerOnce = 0,
    osTimerPeriodic = 1,
}

impl From<TimerMode> for osTimerType_t
{
    fn from(value: TimerMode) -> Self
    {
        match value
        {
            TimerMode::Once => osTimerType_t::osTimerOnce,
            TimerMode::Periodic => osTimerType_t::osTimerPeriodic,
        }
    }
}

#[repr(C)]
pub struct osVersion_t
{
    api: u32,
    kernel: u32,
}

#[repr(C)]
pub struct osThreadAttr_t
{
    pub name: *const c_char,
    pub attr_bits: u32,
    pub cb_mem: *mut c_void,
    pub cb_size: u32,
    pub stack_mem: *mut c_void,
    pub stack_size: u32,
    pub priority: osPriority_t,
    pub tz_module: TZ_ModuleId_t,
    pub reserved: u32,
}

impl osThreadAttr_t
{
    pub fn new(name: &str, stack_size: u32, priority: TaskPriority) -> Self
    {
        let name = unsafe { CStr::from_bytes_with_nul_unchecked(name.as_bytes()) };

        osThreadAttr_t {
            name: name.as_ptr(),
            attr_bits: osThreadDetached,
            cb_mem: null_mut(),
            cb_size: 0,
            stack_mem: null_mut(),
            stack_size,
            priority: priority.into(),
            tz_module: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
pub struct osTimerAttr_t
{
    pub name: *const c_char,
    pub attr_bits: u32,
    pub cb_mem: *mut c_void,
    pub cb_size: u32,
}

#[repr(C)]
pub struct osEventFlagsAttr_t
{
    pub name: *const c_char,
    pub attr_bits: u32,
    pub cb_mem: *mut c_void,
    pub cb_size: u32,
}

impl Default for osEventFlagsAttr_t
{
    fn default() -> Self
    {
        Self {
            name: null(),
            attr_bits: Default::default(),
            cb_mem: null_mut(),
            cb_size: Default::default(),
        }
    }
}

#[repr(C)]
pub struct osMutexAttr_t
{
    pub name: *const c_char,
    pub attr_bits: u32,
    pub cb_mem: *mut c_void,
    pub cb_size: u32,
}

#[repr(C)]
pub struct osSemaphoreAttr_t
{
    pub name: *const c_char,
    pub attr_bits: u32,
    pub cb_mem: *mut c_void,
    pub cb_size: u32,
}

#[repr(C)]
pub struct osMemoryPoolAttr_t
{
    pub name: *const c_char,
    pub attr_bits: u32,
    pub cb_mem: *mut c_void,
    pub cb_size: u32,
    pub mp_mem: *mut c_void,
    pub mp_size: u32,
}

impl Default for osMemoryPoolAttr_t
{
    fn default() -> Self
    {
        Self {
            name: null(),
            attr_bits: 0,
            cb_mem: null_mut(),
            cb_size: 0,
            mp_mem: null_mut(),
            mp_size: 0,
        }
    }
}

#[repr(C)]
pub struct osMessageQueueAttr_t
{
    pub name: *const c_char,
    pub attr_bits: u32,
    pub cb_mem: *mut c_void,
    pub cb_size: u32,
    pub mq_mem: *mut c_void,
    pub mq_size: u32,
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn osKernelInitialize() -> osStatus_t;
    pub fn osKernelGetInfo(version: *mut osVersion_t, id_buf: *mut c_char, id_size: u32) -> osStatus_t;
    pub fn osKernelGetState() -> osKernelState_t;
    pub fn osKernelStart() -> osStatus_t;
    pub fn osKernelLock() -> u32;
    pub fn osKernelUnlock() -> u32;
    pub fn osKernelRestoreLock(lock: i32) -> i32;
    pub fn osKernelSuspend() -> u32;
    pub fn osKernelResume();
    pub fn osKernelGetTickCount() -> u32;
    pub fn osKernelGetTickFreq() -> u32;
    pub fn osKernelGetSysTimerCount() -> u32;
    pub fn osKernelGetSysTimerFreq() -> u32;
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn osThreadNew(func: osThreadFunc_t, argument: *mut c_void, attr: *const osThreadAttr_t) -> osThreadId_t;
    pub fn osThreadGetName(thread_id: osThreadId_t) -> *const c_char;
    pub fn osThreadGetId() -> osThreadId_t;
    pub fn osThreadGetState(thread_id: osThreadId_t) -> osThreadState_t;
    pub fn osThreadGetStackSize(thread_id: osThreadId_t) -> u32;
    pub fn osThreadGetStackSpace(thread_id: osThreadId_t) -> u32;
    pub fn osThreadSetPriority(thread_id: osThreadId_t, priority: osPriority_t) -> osStatus_t;
    pub fn osThreadGetPriority(thread_id: osThreadId_t) -> osPriority_t;
    pub fn osThreadYield() -> osStatus_t;
    pub fn osThreadSuspend(thread_id: osThreadId_t) -> osStatus_t;
    pub fn osThreadResume(thread_id: osThreadId_t) -> osStatus_t;
    pub fn osThreadDetach(thread_id: osThreadId_t) -> osStatus_t;
    pub fn osThreadJoin(thread_id: osThreadId_t) -> osStatus_t;
    pub fn osThreadExit();
    pub fn osThreadTerminate(thread_id: osThreadId_t) -> osStatus_t;
    pub fn osThreadGetCount() -> u32;
    pub fn osThreadEnumerate(thread_id: *mut osThreadId_t, array_items: u32) -> u32;
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn osDelay(ticks: u32) -> osStatus_t;
    pub fn osDelayUntil(ticks: u32) -> osStatus_t;
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn osTimerNew( func: osTimerFunc_t, ty: osTimerType_t, argument: *mut c_void, attr: *const osTimerAttr_t) -> osTimerId_t;
    pub fn osTimerGetName(timer_id: osTimerId_t) -> *const c_char;
    pub fn osTimerStart(timer_id: osTimerId_t, tick: u32) -> osStatus_t;
    pub fn osTimerStop(timer_id: osTimerId_t) -> osStatus_t;
    pub fn osTimerIsRunning(timer_id: osTimerId_t) -> u32;
    pub fn osTimerDelete(timer_id: osTimerId_t) -> osStatus_t;
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn osEventFlagsNew(attr: *const osEventFlagsAttr_t) -> osEventFlagsId_t;
    pub fn osEventFlagsGetName(ef_id: osEventFlagsId_t) -> *const c_char;
    pub fn osEventFlagsSet(ef_id: osEventFlagsId_t, flags: u32) -> u32;
    pub fn osEventFlagsClear(ef_id: osEventFlagsId_t, flags: u32) -> u32;
    pub fn osEventFlagsGet(ef_id: osEventFlagsId_t) -> u32;
    pub fn osEventFlagsWait(ef_id: osEventFlagsId_t, flags: u32, options: u32, timeout: u32) -> u32;
    pub fn osEventFlagsDelete(ef_id: osEventFlagsId_t);
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn osMutexNew(attr: *const osMutexAttr_t) -> osMutexId_t;
    pub fn osMutexGetName(mutex_id: osMutexId_t) -> *const c_char;
    pub fn osMutexAcquire(mutex_id: osMutexId_t, timeout: u32) -> osStatus_t;
    pub fn osMutexRelease(mutex_id: osMutexId_t) -> osStatus_t;
    pub fn osMutexGetOwner(mutex_id: osMutexId_t) -> osStatus_t;
    pub fn osMutexDelete(mutex_id: osMutexId_t) -> osStatus_t;
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn osSemaphoreNew(max_count: u32, initial_count: u32, attr: *const osSemaphoreAttr_t) -> osSemaphoreId_t;
    pub fn osSemaphoreGetName(semaphore_id: osSemaphoreId_t) -> *const c_char;
    pub fn osSemaphoreAcquire(semaphore_id: osSemaphoreId_t, timeout: u32) -> osStatus_t;
    pub fn osSemaphoreRelease(semaphore_id: osSemaphoreId_t) -> osStatus_t;
    pub fn osSemaphoreGetCount(semaphore_id: osSemaphoreId_t) -> u32;
    pub fn osSemaphoreDelete(semaphore_id: osSemaphoreId_t) -> osStatus_t;
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn osMemoryPoolNew(block_count: u32, block_size: u32, attr: *const osMemoryPoolAttr_t) -> osMemoryPoolId_t;
    pub fn osMemoryPoolGetName(mp_id: osMemoryPoolId_t) -> *const c_char;
    pub fn osMemoryPoolAlloc(mp_id: osMemoryPoolId_t, timeout: u32) -> *mut c_void;
    pub fn osMemoryPoolFree(mp_id: osMemoryPoolId_t, block: *mut c_void) -> osStatus_t;
    pub fn osMemoryPoolGetCapacity(mp_id: osMemoryPoolId_t) -> u32;
    pub fn osMemoryPoolGetBlockSize(mp_id: osMemoryPoolId_t) -> u32;
    pub fn osMemoryPoolGetCount(mp_id: osMemoryPoolId_t) -> u32;
    pub fn osMemoryPoolGetSpace(mp_id: osMemoryPoolId_t) -> u32;
    pub fn osMemoryPoolDelete(mp_id: osMemoryPoolId_t) -> osStatus_t;
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn osMessageQueueNew(count: u32, message_size: u32, attr: *const osMessageQueueAttr_t) -> osMessageQueueId_t;
    pub fn osMessageQueueGetName(mq_id: osMessageQueueId_t) -> *const c_char;
    pub fn osMessageQueuePut(mq_id: osMessageQueueId_t, msg_ptr: *const c_void, msg_prio: u8, timeout: u32,) -> osStatus_t;
    pub fn osMessageQueueGet(mq_id: osMessageQueueId_t, msg_ptr: *mut c_void, msg_prio: *mut u8, timeout: u32) -> osStatus_t;
    pub fn osMessageQueueGetCapacity(mq_id: osMessageQueueId_t) -> u32;
    pub fn osMessageQueueGetMsgSize(mq_id: osMessageQueueId_t) -> u32;
    pub fn osMessageQueueGetCount(mq_id: osMessageQueueId_t) -> u32;
    pub fn osMessageQueueGetSpace(mq_id: osMessageQueueId_t) -> u32;
    pub fn osMessageQueueReset(mq_id: osMessageQueueId_t) -> osStatus_t;
    pub fn osMessageQueueDelete(mq_id: osMessageQueueId_t) -> osStatus_t;
}
