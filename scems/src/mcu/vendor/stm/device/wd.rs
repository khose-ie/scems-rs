use crate::mcu::common::wd::WatchDog;
use crate::mcu::vendor::stm::native::iwdg::*;

pub use crate::mcu::vendor::stm::native::iwdg::IWDG_HandleTypeDef;

pub struct WatchDogDevice
{
    handle: *mut IWDG_HandleTypeDef,
}

impl WatchDogDevice
{
    pub fn new(handle: *mut IWDG_HandleTypeDef) -> Self
    {
        WatchDogDevice { handle }
    }
}

impl WatchDog for WatchDogDevice
{
    fn refresh(&self)
    {
        unsafe { HAL_IWDG_Refresh(self.handle) };
    }
}
