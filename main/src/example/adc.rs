use core::fmt::{self, Write};

use scems::mcu::common::adc::AdcDevice;
use scems::mcu::common::uart::UartDevice;
use scems::mcu::vendor::stm::adc::{ADC_HandleTypeDef, AdcQueue};
use scems::mcu::vendor::stm::uart::{UART_HandleTypeDef, UartQueue};

#[allow(improper_ctypes)]
extern "C" {
    static mut huart1: UART_HandleTypeDef;
    static mut hadc1: ADC_HandleTypeDef;
}

#[allow(static_mut_refs)]
pub unsafe fn main()
{
    let mut example =
        AdcExample::new(AdcQueue::allocate(&mut hadc1).unwrap(), UartQueue::allocate(&mut huart1).unwrap());

    loop
    {
        example.tick();
    }
}

pub struct AdcExample
{
    adc: AdcDevice,
    log: UartDevice,
}

impl AdcExample
{
    pub fn new(adc: AdcDevice, log: UartDevice) -> Self
    {
        AdcExample { adc, log }
    }

    pub fn tick(&mut self)
    {
        let value = self.adc.convert();

        if let Ok(x) = value
        {
            writeln!(self, "ADC value: {}", x).ok();
        }
    }
}

impl Write for AdcExample
{
    fn write_str(&mut self, s: &str) -> fmt::Result
    {
        self.log.transmit(s.as_bytes(), 1000).map_err(|_| fmt::Error)
    }
}
