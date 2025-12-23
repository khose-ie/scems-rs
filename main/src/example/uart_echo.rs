use sces::mcu::uart::UartDevice;
use sces_mcu_stm32::uart::{UART_HandleTypeDef, UartQueue};

#[allow(improper_ctypes)]
extern "C" {
    static mut huart1: UART_HandleTypeDef;
}

#[allow(static_mut_refs)]
pub unsafe fn main()
{
    let mut example = UartEchoExample::new(UartQueue::allocate(&mut huart1).unwrap());

    loop
    {
        example.tick();
    }
}

pub struct UartEchoExample
{
    uart: UartDevice,
    cache: [u8; 1024],
}

impl UartEchoExample
{
    pub fn new(uart: UartDevice) -> Self
    {
        UartEchoExample { uart, cache: [0; 1024] }
    }

    pub fn tick(&mut self)
    {
        let value = self.uart.receive(&mut self.cache, 1000);

        if let Ok(x) = value
        {
            #[allow(unused_must_use)]
            self.uart.transmit(&self.cache[0..x as usize], 1000);
        }
    }
}
