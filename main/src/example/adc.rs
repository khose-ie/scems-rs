use core::fmt::{self, Write};
use core::ptr::NonNull;

use sces_mcu::adc::{AdcCtrl, AdcCtrlEvent, AdcDevice};
use sces_mcu::uart::UartDevice;
use sces_mcu::EventLaunch;
use sces_mcu_stm32::adc::{ADC_HandleTypeDef, Adc, AdcQueue};
use sces_mcu_stm32::uart::{UART_HandleTypeDef, UartQueue};
use sces_mcu_stm32::Handle;

#[allow(improper_ctypes)]
extern "C" {
    static mut huart1: UART_HandleTypeDef;
    static mut hadc1: ADC_HandleTypeDef;
}

static mut ABC: Option<AdcExample> = None;

#[allow(static_mut_refs)]
pub unsafe fn main()
{
    let mut example = AdcExample::new(
        AdcQueue::allocate(&mut hadc1).unwrap(),
        UartQueue::allocate(&mut huart1).unwrap(),
    );

    example.ena();

    let mut a = AdcQueue::search_mut(&mut hadc1).unwrap();

    if let Some(abc) = &mut ABC
    {
        a.set_event_agent(abc);
    }
    a.set_event_agent(ABC.as_mut().unwrap());

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

    pub fn ena(&mut self)
    {
        NonNull::new(self as *mut AdcExample)
            .map(|a| self.adc.set_event_agent(unsafe { a.as_ref() }));
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

impl AdcCtrlEvent for AdcExample {}
