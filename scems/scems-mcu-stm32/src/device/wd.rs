use scems_mcu::wd::WatchDogCtrl;

use crate::native::iwdg::*;

pub use crate::native::iwdg::IWDG_HandleTypeDef;

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

