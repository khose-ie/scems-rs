use crate::mcu::common::wd::WatchDog;
use crate::mcu::vendor::stm::native::iwdg::*;

pub struct WatchDogDevice
{
    handle: *mut IWDG,
}

impl WatchDogDevice
{
    pub fn new(handle: *mut IWDG) -> Self
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
