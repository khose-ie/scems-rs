#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::common::{HAL_Lock, HAL_Status};
use super::dma::DMA;

#[repr(C)]
pub struct I2C
{
    pub Instance: *mut I2C_Base,
    pub Init: I2C_Init,
    pub pBuffPtr: *mut u8,
    pub XferSize: u16,
    pub XferCount: u16,
    pub XferOptions: u32,
    pub PreviousState: u32,
    pub hdmatx: *mut DMA,
    pub hdmarx: *mut DMA,
    pub Lock: HAL_Lock,
    pub State: HAL_I2C_State,
    pub Mode: HAL_I2C_Mode,
    pub ErrorCode: u32,
    pub Devaddress: u32,
    pub Memaddress: u32,
    pub MemaddSize: u32,
    pub EventCount: u32,
}

#[repr(C)]
pub struct I2C_Base {}

#[repr(C)]
pub struct I2C_Init
{
    ClockSpeed: u32,
    DutyCycle: u32,
    OwnAddress1: u32,
    AddressingMode: u32,
    DualAddressMode: u32,
    OwnAddress2: u32,
    GeneralCallMode: u32,
    NoStretchMode: u32,
}

#[repr(C)]
pub enum HAL_I2C_State
{
    HAL_I2C_STATE_RESET = 0x00,
    HAL_I2C_STATE_READY = 0x20,
    HAL_I2C_STATE_BUSY = 0x24,
    HAL_I2C_STATE_BUSY_TX = 0x21,
    HAL_I2C_STATE_BUSY_RX = 0x22,
    HAL_I2C_STATE_LISTEN = 0x28,
    HAL_I2C_STATE_BUSY_TX_LISTEN = 0x29,
    HAL_I2C_STATE_BUSY_RX_LISTEN = 0x2A,
    HAL_I2C_STATE_ABORT = 0x60,
    HAL_I2C_STATE_TIMEOUT = 0xA0,
    HAL_I2C_STATE_ERROR = 0xE0,
}

#[repr(C)]
pub enum HAL_I2C_Mode
{
    None = 0x00,
    Master = 0x10,
    Slave = 0x20,
    Mem = 0x40,
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
extern "C" {
    pub fn HAL_I2C_Master_Transmit(hi2c: *mut I2C, DevAddr: u16, pData: *const u8, Size: u16, Timeout: u32) -> HAL_Status;
    pub fn HAL_I2C_Master_Receive(hi2c: *mut I2C, DevAddr: u16, pData: *const u8, Size: u16, Timeout: u32) -> HAL_Status;
    pub fn HAL_I2C_Slave_Transmit(hi2c: *mut I2C, pData: *const u8, Size: u16, Timeout: u32) -> HAL_Status;
    pub fn HAL_I2C_Slave_Receive(hi2c: *mut I2C, pData: *const u8, Size: u16, Timeout: u32) -> HAL_Status;
    pub fn HAL_I2C_Mem_Write(hi2c: *mut I2C, DevAddr: u16, MemAddr: u16, MemAddSize: u16, pData: *const u8, Size: u16, Timeout: u32) -> HAL_Status;
    pub fn HAL_I2C_Mem_Read(hi2c: *mut I2C, DevAddr: u16, MemAddr: u16, MemAddSize: u16, pData: *const u8, Size: u16, Timeout: u32) -> HAL_Status;
    pub fn HAL_I2C_IsDeviceReady(hi2c: *mut I2C, DevAddr: u16, Trials: u32, Timeout: u32) -> HAL_Status;
    pub fn HAL_I2C_Master_Transmit_IT(hi2c: *mut I2C, DevAddr: u16, pData: *const u8, Size: u16) -> HAL_Status;
    pub fn HAL_I2C_Master_Receive_IT(hi2c: *mut I2C, DevAddr: u16, pData: *const u8, Size: u16) -> HAL_Status;
    pub fn HAL_I2C_Slave_Transmit_IT(hi2c: *mut I2C, pData: *const u8, Size: u16) -> HAL_Status;
    pub fn HAL_I2C_Slave_Receive_IT(hi2c: *mut I2C, pData: *const u8, Size: u16) -> HAL_Status;
    pub fn HAL_I2C_Mem_Write_IT(hi2c: *mut I2C, DevAddr: u16, MemAddr: u16, MemAddSize: u16, pData: *const u8, Size: u16) -> HAL_Status;
    pub fn HAL_I2C_Mem_Read_IT(hi2c: *mut I2C, DevAddr: u16, MemAddr: u16, MemAddSize: u16, pData: *const u8, Size: u16) -> HAL_Status;
    pub fn HAL_I2C_Master_Seq_Transmit_IT(hi2c: *mut I2C, DevAddr: u16, pData: *const u8, Size: u16, XferOptions: u32) -> HAL_Status;
    pub fn HAL_I2C_Master_Seq_Receive_IT(hi2c: *mut I2C, DevAddr: u16, pData: *const u8, Size: u16, XferOptions: u32) -> HAL_Status;
    pub fn HAL_I2C_Slave_Seq_Transmit_IT(hi2c: *mut I2C, pData: *const u8, Size: u16, XferOptions: u32) -> HAL_Status;
    pub fn HAL_I2C_Slave_Seq_Receive_IT(hi2c: *mut I2C, pData: *const u8, Size: u16, XferOptions: u32) -> HAL_Status;
    pub fn HAL_I2C_EnableListen_IT(hi2c: *mut I2C) -> HAL_Status;
    pub fn HAL_I2C_DisableListen_IT(hi2c: *mut I2C) -> HAL_Status;
    pub fn HAL_I2C_Master_Abort_IT(hi2c: *mut I2C, DevAddr: u16) -> HAL_Status;
    pub fn HAL_I2C_Master_Transmit_DMA(hi2c: *mut I2C, DevAddr: u16, pData: *const u8, Size: u16) -> HAL_Status;
    pub fn HAL_I2C_Master_Receive_DMA(hi2c: *mut I2C, DevAddr: u16, pData: *const u8, Size: u16) -> HAL_Status;
    pub fn HAL_I2C_Slave_Transmit_DMA(hi2c: *mut I2C, pData: *const u8, Size: u16) -> HAL_Status;
    pub fn HAL_I2C_Slave_Receive_DMA(hi2c: *mut I2C, pData: *const u8, Size: u16) -> HAL_Status;
    pub fn HAL_I2C_Mem_Write_DMA(hi2c: *mut I2C, DevAddr: u16, MemAddr: u16, MemAddSize: u16, pData: *const u8, Size: u16) -> HAL_Status;
    pub fn HAL_I2C_Mem_Read_DMA(hi2c: *mut I2C, DevAddr: u16, MemAddr: u16, MemAddSize: u16, pData: *const u8, Size: u16) -> HAL_Status;
    pub fn HAL_I2C_Master_Seq_Transmit_DMA(hi2c: *mut I2C, DevAddr: u16, pData: *const u8, Size: u16, XferOptions: u32) -> HAL_Status;
    pub fn HAL_I2C_Master_Seq_Receive_DMA(hi2c: *mut I2C, DevAddr: u16, pData: *const u8, Size: u16, XferOptions: u32) -> HAL_Status;
    pub fn HAL_I2C_Slave_Seq_Transmit_DMA(hi2c: *mut I2C, pData: *const u8, Size: u16, XferOptions: u32) -> HAL_Status;
    pub fn HAL_I2C_Slave_Seq_Receive_DMA(hi2c: *mut I2C, pData: *const u8, Size: u16, XferOptions: u32) -> HAL_Status;
    pub fn HAL_I2C_EV_IRQHandler(hi2c: *mut I2C);
    pub fn HAL_I2C_ER_IRQHandler(hi2c: *mut I2C);
    pub fn HAL_I2C_MasterTxCpltCallback(hi2c: *mut I2C);
    pub fn HAL_I2C_MasterRxCpltCallback(hi2c: *mut I2C);
    pub fn HAL_I2C_SlaveTxCpltCallback(hi2c: *mut I2C);
    pub fn HAL_I2C_SlaveRxCpltCallback(hi2c: *mut I2C);
    pub fn HAL_I2C_AddrCallback(hi2c: *mut I2C, TransferDirection: u8, AddrMatchCode: u16);
    pub fn HAL_I2C_ListenCpltCallback(hi2c: *mut I2C);
    pub fn HAL_I2C_MemTxCpltCallback(hi2c: *mut I2C);
    pub fn HAL_I2C_MemRxCpltCallback(hi2c: *mut I2C);
    pub fn HAL_I2C_ErrorCallback(hi2c: *mut I2C);
    pub fn HAL_I2C_AbortCpltCallback(hi2c: *mut I2C);
}
