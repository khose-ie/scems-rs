#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::mcu::common::can::CanMessageHead;

use super::common::{FunctionalState, HAL_Status};

#[repr(C)]
pub struct CAN
{
    pub Instance: *mut CAN_Base,
    pub Init: CAN_Init,
    pub State: HAL_CAN_State,
    pub ErrorCode: u32,
}

#[repr(C)]
pub struct CAN_Base {}

#[repr(C)]
pub struct CAN_Init
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
pub enum HAL_CAN_State
{
    HAL_CAN_STATE_RESET = 0x00,
    HAL_CAN_STATE_READY = 0x01,
    HAL_CAN_STATE_LISTENING = 0x02,
    HAL_CAN_STATE_SLEEP_PENDING = 0x03,
    HAL_CAN_STATE_SLEEP_ACTIVE = 0x04,
    HAL_CAN_STATE_ERROR = 0x05,
}

#[repr(C)]
pub struct CAN_TxHeader
{
    pub StdId: u32,
    pub ExtId: u32,
    pub IDE: u32,
    pub RTR: u32,
    pub DLC: u32,
    pub TransmitGlobalTime: FunctionalState,
}

impl From<&CanMessageHead> for CAN_TxHeader
{
    fn from(value: &CanMessageHead) -> Self
    {
        CAN_TxHeader {
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
pub struct CAN_RxHeader
{
    pub StdId: u32,
    pub ExtId: u32,
    pub IDE: u32,
    pub RTR: u32,
    pub DLC: u32,
    pub Timestamp: u32,
    pub FilterMatchIndex: u32,
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn HAL_CAN_Start(can: *mut CAN) -> HAL_Status;
    pub fn HAL_CAN_Stop(can: *mut CAN) -> HAL_Status;
    pub fn HAL_CAN_RequestSleep(can: *mut CAN) -> HAL_Status;
    pub fn HAL_CAN_WakeUp(can: *mut CAN) -> HAL_Status;
    pub fn HAL_CAN_IsSleepActive(can: *mut CAN) -> u32;
    pub fn HAL_CAN_AddTxMessage(can: *mut CAN, pHeader: *const CAN_TxHeader, aData: &[u8], pTxMailbox: *mut u32) -> HAL_Status;
    pub fn HAL_CAN_AbortTxRequest(can: *mut CAN, TxMailboxes: u32) -> HAL_Status;
    pub fn HAL_CAN_GetTxMailboxesFreeLevel(can: *mut CAN) -> u32;
    pub fn HAL_CAN_IsTxMessagePending(can: *mut CAN, TxMailboxes: u32) -> u32;
    pub fn HAL_CAN_GetTxTimestamp(can: *mut CAN, TxMailbox: u32) -> u32;
    pub fn HAL_CAN_GetRxMessage(can: *mut CAN, RxFifo: u32, pHeader: *mut CAN_RxHeader, aData: &mut [u8]) -> HAL_Status;
    pub fn HAL_CAN_GetRxFifoFillLevel(can: *mut CAN, RxFifo: u32) -> u32;
}
