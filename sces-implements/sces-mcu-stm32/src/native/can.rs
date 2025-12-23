#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use sces::mcu::can::CanMessageHead;

use super::{FunctionalState, HAL_StatusTypeDef};

#[repr(C)]
pub struct CAN_HandleTypeDef
{
    pub Instance: *mut CAN_TypeDef,
    pub Init: CAN_InitTypeDef,
    pub State: HAL_CAN_StateTypeDef,
    pub ErrorCode: u32,
}

#[repr(C)]
pub struct CAN_TypeDef {}

#[repr(C)]
pub struct CAN_InitTypeDef
{
    pub Prescaler: u32,
    pub Mode: u32,
    pub SyncJumpWidth: u32,
    pub TimeSeg1: u32,
    pub TimeSeg2: u32,
    pub TimeTriggeredMode: FunctionalState,
    pub AutoBusOff: FunctionalState,
    pub AutoWakeUp: FunctionalState,
    pub AutoRetransmission: FunctionalState,
    pub ReceiveFifoLocked: FunctionalState,
    pub TransmitFifoPriority: FunctionalState,
}

#[repr(C)]
pub enum HAL_CAN_StateTypeDef
{
    HAL_CAN_STATE_RESET = 0x00,
    HAL_CAN_STATE_READY = 0x01,
    HAL_CAN_STATE_LISTENING = 0x02,
    HAL_CAN_STATE_SLEEP_PENDING = 0x03,
    HAL_CAN_STATE_SLEEP_ACTIVE = 0x04,
    HAL_CAN_STATE_ERROR = 0x05,
}

#[repr(C)]
pub struct CAN_TxHeaderTypeDef
{
    pub StdId: u32,
    pub ExtId: u32,
    pub IDE: u32,
    pub RTR: u32,
    pub DLC: u32,
    pub TransmitGlobalTime: FunctionalState,
}

impl From<&CanMessageHead> for CAN_TxHeaderTypeDef
{
    fn from(value: &CanMessageHead) -> Self
    {
        CAN_TxHeaderTypeDef {
            StdId: value.STD_ID,
            ExtId: value.EXT_ID,
            IDE: value.IDE,
            RTR: value.RTR,
            DLC: value.DLC,
            TransmitGlobalTime: FunctionalState::DISABLE,
        }
    }
}

#[repr(C)]
#[derive(Default)]
pub struct CAN_RxHeaderTypeDef
{
    pub StdId: u32,
    pub ExtId: u32,
    pub IDE: u32,
    pub RTR: u32,
    pub DLC: u32,
    pub Timestamp: u32,
    pub FilterMatchIndex: u32,
}

impl From<CAN_RxHeaderTypeDef> for CanMessageHead
{
    fn from(value: CAN_RxHeaderTypeDef) -> Self
    {
        CanMessageHead {
            STD_ID: value.StdId,
            EXT_ID: value.ExtId,
            IDE: value.IDE,
            RTR: value.RTR,
            DLC: value.DLC,
        }
    }
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn HAL_CAN_Start(can: *mut CAN_HandleTypeDef) -> HAL_StatusTypeDef;
    pub fn HAL_CAN_Stop(can: *mut CAN_HandleTypeDef) -> HAL_StatusTypeDef;
    pub fn HAL_CAN_RequestSleep(can: *mut CAN_HandleTypeDef) -> HAL_StatusTypeDef;
    pub fn HAL_CAN_WakeUp(can: *mut CAN_HandleTypeDef) -> HAL_StatusTypeDef;
    pub fn HAL_CAN_IsSleepActive(can: *mut CAN_HandleTypeDef) -> u32;
    pub fn HAL_CAN_AddTxMessage(can: *mut CAN_HandleTypeDef, pHeader: *const CAN_TxHeaderTypeDef, aData: &[u8], pTxMailbox: *mut u32) -> HAL_StatusTypeDef;
    pub fn HAL_CAN_AbortTxRequest(can: *mut CAN_HandleTypeDef, TxMailboxes: u32) -> HAL_StatusTypeDef;
    pub fn HAL_CAN_GetTxMailboxesFreeLevel(can: *mut CAN_HandleTypeDef) -> u32;
    pub fn HAL_CAN_IsTxMessagePending(can: *mut CAN_HandleTypeDef, TxMailboxes: u32) -> u32;
    pub fn HAL_CAN_GetTxTimestamp(can: *mut CAN_HandleTypeDef, TxMailbox: u32) -> u32;
    pub fn HAL_CAN_GetRxMessage(can: *mut CAN_HandleTypeDef, RxFifo: u32, pHeader: *mut CAN_RxHeaderTypeDef, aData: &mut [u8]) -> HAL_StatusTypeDef;
    pub fn HAL_CAN_GetRxFifoFillLevel(can: *mut CAN_HandleTypeDef, RxFifo: u32) -> u32;
}
