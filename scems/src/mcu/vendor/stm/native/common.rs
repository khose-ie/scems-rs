#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::common::result::{ErrValue, RetValue};

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

impl Into<RetValue<()>> for HAL_StatusTypeDef
{
    fn into(self) -> RetValue<()>
    {
        match self
        {
            Self::HAL_OK => Ok(()),
            Self::HAL_ERROR => Err(ErrValue::Param),
            Self::HAL_BUSY => Err(ErrValue::BusBusy),
            Self::HAL_TIMEOUT => Err(ErrValue::Overtime),
        }
    }
}

impl Into<ErrValue> for HAL_StatusTypeDef
{
    fn into(self) -> ErrValue
    {
        match self
        {
            Self::HAL_OK => ErrValue::None,
            Self::HAL_ERROR => ErrValue::Param,
            Self::HAL_BUSY => ErrValue::BusBusy,
            Self::HAL_TIMEOUT => ErrValue::Overtime,
        }
    }
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn HAL_GetTick() -> u32;
}
