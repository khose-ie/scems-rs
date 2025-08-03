mod device;
mod sample_queue;
mod native;

pub use device::*;

const ADC_COUNT: usize = 4;
const CAN_COUNT: usize = 2;
const I2C_COUNT: usize = 8;
const SPI_COUNT: usize = 8;
const UART_COUNT: usize = 12;
