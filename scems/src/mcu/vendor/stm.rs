mod common;
mod native;
mod device;

pub use native::adc::ADC_HandleTypeDef;
pub use native::can::CAN_HandleTypeDef;
pub use native::dma::DMA_HandleTypeDef;
pub use native::i2c::I2C_HandleTypeDef;
pub use native::io::GPIO_TypeDef;
pub use native::iwdg::IWDG_HandleTypeDef;
pub use native::spi::SPI_HandleTypeDef;
pub use native::uart::UART_HandleTypeDef;
pub use device::*;
