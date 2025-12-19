#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod adc;
pub mod can;
pub mod dma;
pub mod flash;
pub mod i2c;
pub mod io;
pub mod iwdg;
pub mod spi;
pub mod uart;

use sces::value::{ErrValue, RetValue};

#[repr(C)]
pub enum HAL_LockTypeDef
{
    HAL_UNLOCKED = 0x00,
    HAL_LockTypeDefED = 0x01,
}

#[repr(C)]
pub enum FunctionalState
{
    DISABLE = 0,
    ENABLE = 1,
}

#[repr(C)]
#[derive(Debug)]
pub enum HAL_StatusTypeDef
{
    HAL_OK = 0,
    HAL_ERROR = 1,
    HAL_BUSY = 2,
    HAL_TIMEOUT = 3,
}

impl HAL_StatusTypeDef
{
    pub fn ok(self) -> RetValue<()>
    {
        self.into()
    }
}

impl From<HAL_StatusTypeDef> for RetValue<()>
{
    fn from(status: HAL_StatusTypeDef) -> Self
    {
        match status
        {
            HAL_StatusTypeDef::HAL_OK => Ok(()),
            HAL_StatusTypeDef::HAL_ERROR => Err(ErrValue::Param),
            HAL_StatusTypeDef::HAL_BUSY => Err(ErrValue::Busy),
            HAL_StatusTypeDef::HAL_TIMEOUT => Err(ErrValue::Timeout),
        }
    }
}

impl From<HAL_StatusTypeDef> for ErrValue
{
    fn from(value: HAL_StatusTypeDef) -> Self
    {
        match value
        {
            HAL_StatusTypeDef::HAL_OK => Self::None,
            HAL_StatusTypeDef::HAL_ERROR => Self::Param,
            HAL_StatusTypeDef::HAL_BUSY => Self::Busy,
            HAL_StatusTypeDef::HAL_TIMEOUT => Self::Timeout,
        }
    }
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn HAL_GetTick() -> u32;
    pub fn HAL_Delay(Delay: u32);
}
