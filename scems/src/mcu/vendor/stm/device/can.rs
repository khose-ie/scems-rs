use core::mem::transmute;

use crate::common::result::{ErrValue, RetValue};
use crate::mcu::common::can::{CanCtrl, CanCtrlEvent, CanMessage};
use crate::mcu::common::EventLaunch;
pub use crate::mcu::vendor::stm::native::can::CAN_HandleTypeDef;
use crate::mcu::vendor::stm::native::can::*;
use crate::mcu::vendor::stm::native::*;
use crate::mcu::vendor::stm::sample_queue::SampleQueue;
use crate::mcu::vendor::stm::{Handle, CAN_COUNT};

/////////////////////////////////////////////////////////////////////////////
// CAN struct
/////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Can
{
    handle: *mut CAN_HandleTypeDef,
    event_handle: Option<*const dyn CanCtrlEvent>,
    fifo: u32,
    async_cache: Option<*mut CanMessage>,
}

impl Can
{
    fn new(handle: *mut CAN_HandleTypeDef, fifo: u32) -> RetValue<Self>
    {
        if handle.is_null()
        {
            return Err(ErrValue::Param);
        }

        Ok(Can { handle, event_handle: None, fifo, async_cache: None })
    }
}

impl Handle<CAN_HandleTypeDef> for Can
{
    fn handle_value(&self) -> *mut CAN_HandleTypeDef
    {
        self.handle
    }
}

impl EventLaunch<dyn CanCtrlEvent> for Can
{
    fn set_event_agent(&mut self, event_handle: &'static dyn CanCtrlEvent)
    {
        self.event_handle = Some(unsafe { transmute(event_handle as *const dyn CanCtrlEvent) });
    }

    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
    }
}

impl CanCtrl for Can
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

        let tx_head = CAN_TxHeaderTypeDef::from(&can_message.head);

        let tick = unsafe { HAL_GetTick() };

        loop
        {
            status = unsafe { HAL_CAN_AddTxMessage(self.handle, &tx_head, &can_message.data.content, &mut mail_box) };

            if matches!(status, HAL_StatusTypeDef::HAL_OK)
            {
                break;
            }

            duration = unsafe { HAL_GetTick() - tick };

            if duration > timeout
            {
                break;
            }
        }

        if !matches!(status, HAL_StatusTypeDef::HAL_OK)
        {
            return Err(ErrValue::Busy);
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
            return Err(ErrValue::Busy);
        }

        Ok(())
    }

    fn receive(&self, can_message: &mut CanMessage, timeout: u32) -> RetValue<()>
    {
        let mut status;
        let mut duration: u32;
        let mut rx_head: CAN_RxHeaderTypeDef = Default::default();

        let tick = unsafe { HAL_GetTick() };

        loop
        {
            status =
                unsafe { HAL_CAN_GetRxMessage(self.handle, self.fifo, &mut rx_head, &mut can_message.data.content) };

            if matches!(status, HAL_StatusTypeDef::HAL_OK)
            {
                break;
            }

            duration = unsafe { HAL_GetTick() - tick };

            if duration > timeout
            {
                break;
            }
        }

        if !matches!(status, HAL_StatusTypeDef::HAL_OK)
        {
            return Err(ErrValue::Busy);
        }

        can_message.head.STD_ID = rx_head.StdId;

        Ok(())
    }

    fn async_transmit(&self, can_message: &CanMessage) -> RetValue<()>
    {
        let mut mail_box: u32 = 0;
        let tx_head = CAN_TxHeaderTypeDef::from(&can_message.head);
        unsafe { HAL_CAN_AddTxMessage(self.handle, &tx_head, &can_message.data.content, &mut mail_box).into() }
    }

    fn async_receive(&mut self, can_message: &mut CanMessage)
    {
        self.async_cache = Some(can_message as *const CanMessage as *mut CanMessage);
    }
}

/////////////////////////////////////////////////////////////////////////////
// CAN queue
/////////////////////////////////////////////////////////////////////////////

static mut CAN_QUEUE: SampleQueue<Can, CAN_HandleTypeDef, CAN_COUNT> = SampleQueue::new();

pub struct CanQueue;

impl CanQueue
{
    #[inline]
    #[allow(static_mut_refs)]
    pub fn allocate(sample_handle: *mut CAN_HandleTypeDef, fifo: u32) -> RetValue<&'static mut Can>
    {
        unsafe { CAN_QUEUE.allocate(&Can::new(sample_handle, fifo)?) }
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn clean(sample_handle: *mut CAN_HandleTypeDef)
    {
        unsafe { CAN_QUEUE.clean(sample_handle) };
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn search(sample_handle: *mut CAN_HandleTypeDef) -> RetValue<&'static Can>
    {
        unsafe { CAN_QUEUE.search(sample_handle) }
    }
}

/////////////////////////////////////////////////////////////////////////////
// HAL interrupt callback function implementations
/////////////////////////////////////////////////////////////////////////////

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_TxMailbox0CompleteCallback(can: *mut CAN_HandleTypeDef) {}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_TxMailbox1CompleteCallback(can: *mut CAN_HandleTypeDef) {}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_TxMailbox2CompleteCallback(can: *mut CAN_HandleTypeDef) {}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_TxMailbox0AbortCallback(can: *mut CAN_HandleTypeDef) {}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_TxMailbox1AbortCallback(can: *mut CAN_HandleTypeDef) {}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_TxMailbox2AbortCallback(can: *mut CAN_HandleTypeDef) {}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_CAN_RxFifo0MsgPendingCallback(can: *mut CAN_HandleTypeDef)
{
    let mut value = Err(ErrValue::NotAvailable);

    if let Some(sample) = CanQueue::search(can).ok()
    {
        if let Some(async_cache) = sample.async_cache
        {
            value = sample.receive(&mut *async_cache, 0);
        }

        if !value.is_ok()
        {
            return;
        }

        if let Some(event_handle) = sample.event_handle
        {
            (*event_handle).on_can_message_receive();
        }
    }
}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_RxFifo0FullCallback(can: *mut CAN_HandleTypeDef) {}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_CAN_RxFifo1MsgPendingCallback(can: *mut CAN_HandleTypeDef)
{
    HAL_CAN_RxFifo0MsgPendingCallback(can);
}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_RxFifo1FullCallback(can: *mut CAN_HandleTypeDef) {}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_SleepCallback(can: *mut CAN_HandleTypeDef) {}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_CAN_WakeUpFromRxMsgCallback(can: *mut CAN_HandleTypeDef) {}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_CAN_ErrorCallback(can: *mut CAN_HandleTypeDef)
{
    if let Some(sample) = CanQueue::search(can).ok()
    {
        if let Some(event_handle) = sample.event_handle
        {
            (*event_handle).on_can_error();
        }
    }
}
