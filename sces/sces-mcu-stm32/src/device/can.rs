use core::ptr::NonNull;

use sces::value::{ErrValue, RetValue};
use sces_mcu::can::{CanCtrl, CanCtrlEvent, CanMessage};
use sces_mcu::EventLaunch;

use crate::device::Handle;
use crate::native::can::*;
use crate::native::*;
use crate::sample_queue::SampleQueue;
use crate::CAN_COUNT;

pub use crate::native::can::CAN_HandleTypeDef;

/////////////////////////////////////////////////////////////////////////////
// CAN Class
/////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Can
{
    handle: NonNull<CAN_HandleTypeDef>,
    event_handle: Option<&'static dyn CanCtrlEvent>,
    fifo: u32,
    async_cache: Option<NonNull<CanMessage>>,
}

impl Can
{
    fn new(handle: *mut CAN_HandleTypeDef, fifo: u32) -> RetValue<Self>
    {
        Ok(Can {
            handle: NonNull::new(handle).ok_or(ErrValue::Param)?,
            event_handle: None,
            fifo,
            async_cache: None,
        })
    }
}

impl Handle<CAN_HandleTypeDef> for Can
{
    fn handle_value(&self) -> *mut CAN_HandleTypeDef
    {
        self.handle.as_ptr()
    }
}

impl EventLaunch<dyn CanCtrlEvent> for Can
{
    fn set_event_agent(&mut self, event_handle: &'static dyn CanCtrlEvent)
    {
        self.event_handle = Some(event_handle);
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
        unsafe { HAL_CAN_Start(self.handle.as_ptr()).into() }
    }

    fn deactivate(&self) -> RetValue<()>
    {
        unsafe { HAL_CAN_Stop(self.handle.as_ptr()).into() }
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
            status = unsafe {
                HAL_CAN_AddTxMessage(
                    self.handle.as_ptr(),
                    &tx_head,
                    &can_message.data,
                    &mut mail_box,
                )
            };

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
            can_status = unsafe { HAL_CAN_IsTxMessagePending(self.handle.as_ptr(), mail_box) };

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
            unsafe { HAL_CAN_AbortTxRequest(self.handle.as_ptr(), mail_box) };
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
            status = unsafe {
                HAL_CAN_GetRxMessage(
                    self.handle.as_ptr(),
                    self.fifo,
                    &mut rx_head,
                    &mut can_message.data,
                )
            };

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

        can_message.head = rx_head.into();

        Ok(())
    }

    fn async_transmit(&self, can_message: &CanMessage) -> RetValue<()>
    {
        let mut mail_box: u32 = 0;
        let tx_head = CAN_TxHeaderTypeDef::from(&can_message.head);
        unsafe {
            HAL_CAN_AddTxMessage(self.handle.as_ptr(), &tx_head, &can_message.data, &mut mail_box)
                .into()
        }
    }

    fn async_receive(&mut self, can_message: &mut CanMessage)
    {
        self.async_cache = Some(NonNull::from(can_message));
    }
}

/////////////////////////////////////////////////////////////////////////////
// CAN Queue
/////////////////////////////////////////////////////////////////////////////

static mut CAN_QUEUE: SampleQueue<Can, CAN_HandleTypeDef, CAN_COUNT> = SampleQueue::new();

pub struct CanQueue;

impl CanQueue
{
    #[inline]
    #[allow(static_mut_refs)]
    pub fn alloc(sample_handle: *mut CAN_HandleTypeDef, fifo: u32)
        -> RetValue<&'static mut Can>
    {
        unsafe { CAN_QUEUE.allocate(&Can::new(sample_handle, fifo)?) }
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn clean(sample_handle: *mut CAN_HandleTypeDef)
    {
        NonNull::new(sample_handle).inspect(|handle| unsafe { CAN_QUEUE.clean(*handle) });
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn search(sample_handle: *mut CAN_HandleTypeDef) -> RetValue<&'static Can>
    {
        unsafe { CAN_QUEUE.search(NonNull::new(sample_handle).ok_or(ErrValue::Param)?) }
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn search_mut(sample_handle: *mut CAN_HandleTypeDef) -> RetValue<&'static mut Can>
    {
        unsafe { CAN_QUEUE.search_mut(NonNull::new(sample_handle).ok_or(ErrValue::Param)?) }
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
    if let Ok(sample) = CanQueue::search_mut(can)
    {
        if let Some(mut async_cache) = sample.async_cache
        {
            if sample.receive(async_cache.as_mut(), 0).is_ok()
            {
                sample.event_handle.inspect(|event_handle| event_handle.on_can_message_receive());
            }
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
    if let Ok(sample) = CanQueue::search(can)
    {
        sample.event_handle.inspect(|event_handle| event_handle.on_can_error());
    }
}
