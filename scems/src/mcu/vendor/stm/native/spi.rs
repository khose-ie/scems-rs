#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::c_void;

use super::{HAL_LockTypeDef, HAL_StatusTypeDef};
use super::dma::DMA_HandleTypeDef;

#[repr(C)]
pub struct SPI_HandleTypeDef
{
    pub Instance: *mut SPI_TypeDef,
    pub Init: SPI_InitTypeDef,
    pub pTxBuffPtr: *const u8,
    pub TxXferSize: u16,
    pub TxXferCount: u16,
    pub pRxBuffPtr: *mut u8,
    pub RxXferSize: u16,
    pub RxXferCount: u16,
    pub RxISR: *const c_void,
    pub TxISR: *const c_void,
    pub hdmatx: *mut DMA_HandleTypeDef,
    pub hdmarx: *mut DMA_HandleTypeDef,
    pub Lock: HAL_LockTypeDef,
    pub State: HAL_SPI_StateTypeDef,
    pub ErrorCode: u32,
}

#[repr(C)]
pub struct SPI_TypeDef {}

#[repr(C)]
pub struct SPI_InitTypeDef
{
    pub Mode: u32,
    pub Direction: u32,
    pub DataSize: u32,
    pub CLKPolarity: u32,
    pub CLKPhase: u32,
    pub NSS: u32,
    pub BaudRatePrescaler: u32,
    pub FirstBit: u32,
    pub TIMode: u32,
    pub CRCCalculation: u32,
    pub CRCPolynomial: u32,
}

#[repr(C)]
pub enum HAL_SPI_StateTypeDef
{
    HAL_SPI_STATE_RESET = 0x00,
    HAL_SPI_STATE_READY = 0x01,
    HAL_SPI_STATE_BUSY = 0x02,
    HAL_SPI_STATE_BUSY_TX = 0x03,
    HAL_SPI_STATE_BUSY_RX = 0x04,
    HAL_SPI_STATE_BUSY_TX_RX = 0x05,
    HAL_SPI_STATE_ERROR = 0x06,
    HAL_SPI_STATE_ABORT = 0x07,
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
extern "C" {
    pub fn HAL_SPI_Transmit(hspi: *mut SPI_HandleTypeDef, pData: *const u8, Size: u16, Timeout: u32) -> HAL_StatusTypeDef;
    pub fn HAL_SPI_Receive(hspi: *mut SPI_HandleTypeDef, pData: *const u8, Size: u16, Timeout: u32) -> HAL_StatusTypeDef;
    pub fn HAL_SPI_TransmitReceive(hspi: *mut SPI_HandleTypeDef, pTxData: *const u8, pRxData: *const u8, Size: u16, Timeout: u32) -> HAL_StatusTypeDef;
    pub fn HAL_SPI_Transmit_IT(hspi: *mut SPI_HandleTypeDef, pData: *const u8, Size: u16) -> HAL_StatusTypeDef;
    pub fn HAL_SPI_Receive_IT(hspi: *mut SPI_HandleTypeDef, pData: *const u8, Size: u16) -> HAL_StatusTypeDef;
    pub fn HAL_SPI_TransmitReceive_IT(hspi: *mut SPI_HandleTypeDef, pTxData: *const u8, pRxData: *const u8, Size: u16) -> HAL_StatusTypeDef;
    pub fn HAL_SPI_Transmit_DMA(hspi: *mut SPI_HandleTypeDef, pData: *const u8, Size: u16) -> HAL_StatusTypeDef;
    pub fn HAL_SPI_Receive_DMA(hspi: *mut SPI_HandleTypeDef, pData: *const u8, Size: u16) -> HAL_StatusTypeDef;
    pub fn HAL_SPI_TransmitReceive_DMA(hspi: *mut SPI_HandleTypeDef, pTxData: *const u8, pRxData: *const u8, Size: u16) -> HAL_StatusTypeDef;
    pub fn HAL_SPI_Abort(hspi: *mut SPI_HandleTypeDef) -> HAL_StatusTypeDef;
    pub fn HAL_SPI_Abort_IT(hspi: *mut SPI_HandleTypeDef) -> HAL_StatusTypeDef;
}
