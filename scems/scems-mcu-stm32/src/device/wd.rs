use core::ptr::NonNull;

use scems::value::{ErrValue, RetValue};
use scems_mcu::wd::WatchDogCtrl;

use crate::{Handle, IWDG_COUNT, native::iwdg::*, sample_queue::SampleQueue};

pub use crate::native::iwdg::IWDG_HandleTypeDef;

#[derive(Clone, Copy)]
pub struct WatchDog
{
    handle: *mut IWDG_HandleTypeDef,
}

impl WatchDog
{
    pub fn new(handle: *mut IWDG_HandleTypeDef) -> Self
    {
        WatchDog { handle }
    }
}

impl WatchDogCtrl for WatchDog
{
    fn refresh(&self)
    {
        unsafe { HAL_IWDG_Refresh(self.handle) };
    }
}

impl Handle<IWDG_HandleTypeDef> for WatchDog
{
    fn handle_value(&self) -> *mut IWDG_HandleTypeDef
    {
        self.handle
    }
}

static mut WD_QUEUE: SampleQueue<WatchDog, IWDG_HandleTypeDef, IWDG_COUNT> = SampleQueue::new();

pub struct WatchDogQueue;

impl WatchDogQueue
{
    #[inline]
    #[allow(static_mut_refs)]
    pub fn alloc(sample_handle: *mut IWDG_HandleTypeDef) -> RetValue<&'static mut WatchDog>
    {
        unsafe { WD_QUEUE.allocate(&WatchDog::new(sample_handle)) }
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn clean(sample_handle: *mut IWDG_HandleTypeDef)
    {
        NonNull::new(sample_handle).inspect(|handle| unsafe { WD_QUEUE.clean(*handle) });
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn search(sample_handle: *mut IWDG_HandleTypeDef) -> RetValue<&'static WatchDog>
    {
        unsafe { WD_QUEUE.search(NonNull::new(sample_handle).ok_or(ErrValue::Param)?) }
    }
}
