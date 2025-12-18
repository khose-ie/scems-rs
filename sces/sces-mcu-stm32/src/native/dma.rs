#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::c_void;

use super::HAL_LockTypeDef;

#[repr(C)]
pub struct DMA_HandleTypeDef
{
    pub Instance: *mut DMA_Stream_TypeDef,
    pub Init: DMA_InitTypeDef,
    pub Lock: HAL_LockTypeDef,
    pub State: HAL_DMA_StateTypeDef,
    pub Parent: *mut c_void,
    pub XferCpltCallback: *mut c_void,
    pub XferHalfCpltCallback: *mut c_void,
    pub XferM1CpltCallback: *mut c_void,
    pub XferM1HalfCpltCallback: *mut c_void,
    pub XferErrorCallback: *mut c_void,
    pub XferAbortCallback: *mut c_void,
    pub ErrorCode: u32,
    pub StreamBaseAddress: u32,
    pub StreamIndex: u32,
}

#[repr(C)]
pub struct DMA_InitTypeDef
{
    pub Channel: u32,
    pub Direction: u32,
    pub PeriphInc: u32,
    pub MemInc: u32,
    pub PeriphDataAlignment: u32,
    pub MemDataAlignment: u32,
    pub Mode: u32,
    pub Priority: u32,
    pub FIFOMode: u32,
    pub FIFOThreshold: u32,
    pub MemBurst: u32,
    pub PeriphBurst: u32,
}

#[repr(C)]
pub struct DMA_Stream_TypeDef {}

#[repr(C)]
pub enum HAL_DMA_StateTypeDef
{
    HAL_DMA_STATE_RESET = 0x00,
    HAL_DMA_STATE_READY = 0x01,
    HAL_DMA_STATE_BUSY = 0x02,
    HAL_DMA_STATE_TIMEOUT = 0x03,
    HAL_DMA_STATE_ERROR = 0x04,
    HAL_DMA_STATE_ABORT = 0x05,
}
