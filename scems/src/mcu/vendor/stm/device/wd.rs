use crate::mcu::common::wd::WatchDogCtrl;
use crate::mcu::vendor::stm::native::iwdg::*;

pub use crate::mcu::vendor::stm::native::iwdg::IWDG_HandleTypeDef;

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

