use crate::os::common::kernel::IKernel;
use crate::os::vendors::cmsis_os::cmsis::*;

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
