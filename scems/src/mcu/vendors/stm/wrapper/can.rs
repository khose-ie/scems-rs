use core::mem::transmute;

use crate::common::result::{ErrValue, RetValue};
use crate::derive::{AsPtr, HandlePtr};
use crate::mcu::common::can::{Can, CanEventAgent, CanMessage};
use crate::mcu::common::{EventLaunch, HandlePtr};
use crate::mcu::vendors::stm::common::DeviceQueue;
use crate::mcu::vendors::stm::native::can::*;
use crate::mcu::vendors::stm::native::common::{HAL_Status, *};

const CAN_COUNT: usize = 8;
static mut CANS: DeviceQueue<CAN, CanDevice, CAN_COUNT> = DeviceQueue::new();

#[derive(AsPtr, HandlePtr)]
pub struct CanDevice
{
    handle: *mut CAN,
    event_handle: Option<*const dyn CanEventAgent>,
    fifo: u32,
    async_cache: Option<*mut CanMessage>,
}

impl CanDevice
{
    pub fn new(handle: *mut CAN, fifo: u32) -> Self
    {
        CanDevice { handle, event_handle: None, fifo, async_cache: None }
    }
}

impl Drop for CanDevice
{
    fn drop(&mut self)
    {
        self.clean_event_agent();
    }
}

impl EventLaunch<dyn CanEventAgent> for CanDevice
{
    #[allow(static_mut_refs)]
    fn set_event_agent(&mut self, event_handle: &dyn CanEventAgent) -> RetValue<()>
    {
        self.event_handle = Some(unsafe { transmute(event_handle as *const dyn CanEventAgent) });
        unsafe { CANS.alloc(self.as_ptr()) }
    }

    #[allow(static_mut_refs)]
    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
        unsafe { CANS.clean(self.as_ptr()) };
    }
}

impl Can for CanDevice
{
    fn activate(&self) -> RetValue<()>
    {
        unsafe { HAL_CAN_Start(self.handle).into() }
    }

    fn deactivate(&self) -> RetValue<()>
    {
        unsafe { HAL_CAN_Stop(self.handle).into() }
    }

    fn transmit(&self, can_message: &CanMessage, timeout: u32) -> RetValue<()>
    {
        let mut status;
        let mut can_status;
        let mut duration: u32;
        let mut mail_box: u32 = 0;

        let tx_head = CAN_TxHeader::from(&can_message.head);

        let tick = unsafe { HAL_GetTick() };

        loop
        {
            status = unsafe { HAL_CAN_AddTxMessage(self.handle, &tx_head, &can_message.data.content, &mut mail_box) };

            if matches!(status, HAL_Status::HAL_OK)
            {
                break;
            }

            duration = unsafe { HAL_GetTick() - tick };

            if duration > timeout
            {
                break;
            }
        }

        if !matches!(status, HAL_Status::HAL_OK)
        {
            return Err(ErrValue::BusBusy);
        }

        let tick = unsafe { HAL_GetTick() };

        loop
        {
            can_status = unsafe { HAL_CAN_IsTxMessagePending(self.handle, mail_box) };

            if can_status == 0
            {
                break;
            }

            duration = unsafe { HAL_GetTick() - tick };

            if duration > timeout
            {
                break;
            }
        }

        if can_status != 0
        {
            unsafe { HAL_CAN_AbortTxRequest(self.handle, mail_box) };
            return Err(ErrValue::BusBusy);
        }

        Ok(())
    }

    fn receive(&self, can_message: &mut CanMessage, timeout: u32) -> RetValue<()>
    {
        let mut status;
        let mut duration: u32;
        let mut rx_head: CAN_RxHeader = Default::default();

        let tick = unsafe { HAL_GetTick() };

        loop
        {
            status =
                unsafe { HAL_CAN_GetRxMessage(self.handle, self.fifo, &mut rx_head, &mut can_message.data.content) };

            if matches!(status, HAL_Status::HAL_OK)
            {
                break;
            }

            duration = unsafe { HAL_GetTick() - tick };

            if duration > timeout
            {
                break;
            }
        }

        if !matches!(status, HAL_Status::HAL_OK)
        {
            return Err(ErrValue::BusBusy);
        }

        can_message.head.STD_ID = rx_head.StdId;

        Ok(())
    }

    fn async_transmit(&self, can_message: &CanMessage) -> RetValue<()>
    {
        let mut mail_box: u32 = 0;
        let tx_head = CAN_TxHeader::from(&can_message.head);
        unsafe { HAL_CAN_AddTxMessage(self.handle, &tx_head, &can_message.data.content, &mut mail_box).into() }
    }

    fn async_receive(&mut self, can_message: &mut CanMessage)
    {
        self.async_cache = Some(can_message as *const CanMessage as *mut CanMessage);
    }
}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_TxMailbox0CompleteCallback(can: *mut CAN) {}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_TxMailbox1CompleteCallback(can: *mut CAN) {}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_TxMailbox2CompleteCallback(can: *mut CAN) {}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_TxMailbox0AbortCallback(can: *mut CAN) {}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_TxMailbox1AbortCallback(can: *mut CAN) {}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_TxMailbox2AbortCallback(can: *mut CAN) {}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_CAN_RxFifo0MsgPendingCallback(can: *mut CAN)
{
    let mut value = Err(ErrValue::NotAvailable);

    if let Some(sample) = CANS.find(can).ok()
    {
        if let Some(async_cache) = (*sample).async_cache
        {
            value = (*sample).receive(&mut *async_cache, 0);
        }

        if !value.is_ok()
        {
            return;
        }

        if let Some(event_handle) = (*sample).event_handle
        {
            (*event_handle).on_can_message_receive();
        }
    }
}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_RxFifo0FullCallback(can: *mut CAN) {}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_CAN_RxFifo1MsgPendingCallback(can: *mut CAN)
{
    HAL_CAN_RxFifo0MsgPendingCallback(can);
}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_RxFifo1FullCallback(can: *mut CAN) {}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_SleepCallback(can: *mut CAN) {}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_WakeUpFromRxMsgCallback(can: *mut CAN) {}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_CAN_ErrorCallback(can: *mut CAN)
{
    if let Some(sample) = CANS.find(can).ok()
    {
        if let Some(event_handle) = (*sample).event_handle
        {
            (*event_handle).on_can_error();
        }
    }
}
