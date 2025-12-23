use sces::mcu::io::{IoCtrl, IoDevice, IoState};
use sces_mcu_stm32::io::{GPIO_Pin, GPIO_TypeDef, IoQueue};

pub const GPIOC: *mut GPIO_TypeDef = ((0x40000000 + 0x00020000) + 0x0800) as *mut GPIO_TypeDef;
pub const GPIOH: *mut GPIO_TypeDef = ((0x40000000 + 0x00020000) + 0x1C00) as *mut GPIO_TypeDef;

pub fn main()
{
    let mut io = IoLampExample::new(IoQueue::allocate(GPIOC, GPIO_Pin::P13), IoQueue::allocate(GPIOH, GPIO_Pin::P10));

    loop
    {
        io.tick();
    }
}

pub struct IoLampExample
{
    lamp: IoDevice,
    key: IoDevice,
    key_state: IoState,
}

impl IoLampExample
{
    pub fn new(lamp: IoDevice, key: IoDevice) -> Self
    {
        IoLampExample { lamp, key, key_state: IoState::Reset }
    }

    pub fn tick(&mut self)
    {
        let key_state = self.key.state();

        if key_state == IoState::Set && key_state != self.key_state
        {
            self.key_state = key_state;
            self.lamp.toggle();
        }
    }
}
