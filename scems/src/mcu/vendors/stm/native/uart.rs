#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::common::{HAL_Lock, HAL_Status};
use super::dma::DMA;

#[repr(C)]
pub struct UART
{
    pub Instance: *mut UART_Base,
    pub Init: UART_Init,
    pub pTxBuffPtr: *const u8,
    pub TxXferSize: u16,
    pub TxXferCount: u16,
    pub pRxBuffPtr: *mut u8,
    pub RxXferSize: u16,
    pub RxXferCount: u16,
    pub ReceptionType: HAL_UART_RxType,
    pub RxEventType: HAL_UART_RxEventType,
    pub hdmatx: *mut DMA,
    pub hdmarx: *mut DMA,
    pub Lock: HAL_Lock,
    pub gState: HAL_UART_State,
    pub RxState: HAL_UART_State,
    pub ErrorCode: u32,
}

#[repr(C)]
pub struct UART_Base {}

#[repr(C)]
pub struct UART_Init
{
    pub BaudRate: u32,
    pub WordLength: u32,
    pub StopBits: u32,
    pub Parity: u32,
    pub Mode: u32,
    pub HwFlowCtl: u32,
    pub OverSampling: u32,
}

type HAL_UART_RxType = u32;

type HAL_UART_RxEventType = u32;

#[repr(C)]
pub enum HAL_UART_State
{
    HAL_UART_STATE_RESET = 0x00,
    HAL_UART_STATE_READY = 0x20,
    HAL_UART_STATE_BUSY = 0x24,
    HAL_UART_STATE_BUSY_TX = 0x21,
    HAL_UART_STATE_BUSY_RX = 0x22,
    HAL_UART_STATE_BUSY_TX_RX = 0x23,
    HAL_UART_STATE_TIMEOUT = 0xA0,
    HAL_UART_STATE_ERROR = 0xE0,
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn HAL_UART_Transmit(uart: *mut UART, pData: *const u8, Size: u16, Timeout: u32) -> HAL_Status;
    pub fn HAL_UART_Receive(uart: *mut UART, pData: *const u8, Size: u16, Timeout: u32) -> HAL_Status;
    pub fn HAL_UART_Transmit_IT(uart: *mut UART, pData: *const u8, Size: u16) -> HAL_Status;
    pub fn HAL_UART_Receive_IT(uart: *mut UART, pData: *const u8, Size: u16) -> HAL_Status;
    pub fn HAL_UART_Transmit_DMA(uart: *mut UART, pData: *const u8, Size: u16) -> HAL_Status;
    pub fn HAL_UART_Receive_DMA(uart: *mut UART, pData: *const u8, Size: u16) -> HAL_Status;
    pub fn HAL_UART_Abort(uart: *mut UART) -> HAL_Status;
    pub fn HAL_UART_AbortTransmit(uart: *mut UART) -> HAL_Status;
    pub fn HAL_UART_AbortReceive(uart: *mut UART) -> HAL_Status;
    pub fn HAL_UART_Abort_IT(uart: *mut UART) -> HAL_Status;
    pub fn HAL_UART_AbortTransmit_IT(uart: *mut UART) -> HAL_Status;
    pub fn HAL_UART_AbortReceive_IT(uart: *mut UART) -> HAL_Status;
    pub fn HAL_UARTEx_ReceiveToIdle(uart: *mut UART, pData: *const u8, Size: u16, RxLen: &u16, Timeout: u32) -> HAL_Status;
    pub fn HAL_UARTEx_ReceiveToIdle_IT(uart: *mut UART, pData: *const u8, Size: u16) -> HAL_Status;
    pub fn HAL_UARTEx_ReceiveToIdle_DMA(uart: *mut UART, pData: *const u8, Size: u16) -> HAL_Status;
    pub fn HAL_UART_GetState(uart: *mut UART) -> HAL_UART_State;
}
