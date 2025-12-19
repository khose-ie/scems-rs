#![no_std]

mod device;
mod native;
mod sample_queue;

use sces_mcu::MCU;

pub use device::*;

const ADC_COUNT: usize = 4;
const CAN_COUNT: usize = 2;
const I2C_COUNT: usize = 8;
const IO_COUNT: usize = 32;
const SPI_COUNT: usize = 8;
const UART_COUNT: usize = 12;
const IWDG_COUNT: usize = 1;

pub struct STM32;

impl MCU for STM32
{
    type Adc = device::adc::Adc;
    type Can = device::can::Can;
    type Flash = device::flash::OnChipFlash;
    type I2cMaster = device::i2c::I2cMaster;
    type I2cMem = device::i2c::I2cMem;
    type I2cSlave = device::i2c::I2cSlave;
    type Io = device::io::Io;
    type Spi = device::spi::Spi;
    type Uart = device::uart::Uart;
    // type TimBase = device::adc::Adc;
    // type TImPwm = device::adc::Adc;
    type WatchDog = device::wd::WatchDog;

    fn tick_value() -> u32
    {
        unsafe { native::HAL_GetTick() }
    }

    fn sleep(time: u32)
    {
        unsafe { native::HAL_Delay(time) };
    }
}
