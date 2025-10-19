use scems_os::kernel::IKernel;

use crate::native::*;

pub struct Kernel {}

impl IKernel for Kernel
{
    #[inline]
    fn delay(time: u32)
    {
        unsafe { osDelay(time) };
    }

    #[inline]
    fn delay_interval(time: u32)
    {
        unsafe { osDelayUntil(time) };
    }

    #[inline]
    fn systick_value() -> u32
    {
        unsafe { osKernelGetTickCount() }
    }

    #[inline]
    fn cede()
    {
        unsafe { osThreadYield() };
    }

    #[inline]
    fn exit()
    {
        unsafe { osThreadExit() };
    }
}
