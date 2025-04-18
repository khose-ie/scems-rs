#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::common::result::{IError, IResult};

#[repr(C)]
pub enum HAL_Lock
{
    HAL_UNLOCKED = 0x00,
    HAL_LOCKED = 0x01,
}

#[repr(C)]
pub enum FunctionalState
{
    DISABLE = 0,
    ENABLE = 1,
}

#[repr(C)]
#[derive(Debug)]
pub enum HAL_Status
{
    HAL_OK = 0,
    HAL_ERROR = 1,
    HAL_BUSY = 2,
    HAL_TIMEOUT = 3,
}

impl HAL_Status
{
    pub fn ok(self) -> IResult<()>
    {
        self.into()
    }
}

impl Into<IResult<()>> for HAL_Status
{
    fn into(self) -> IResult<()>
    {
        match self
        {
            Self::HAL_OK => Ok(()),
            Self::HAL_ERROR => Err(IError::Param),
            Self::HAL_BUSY => Err(IError::BusBusy),
            Self::HAL_TIMEOUT => Err(IError::Overtime),
        }
    }
}

impl Into<IError> for HAL_Status
{
    fn into(self) -> IError
    {
        match self
        {
            Self::HAL_OK => IError::None,
            Self::HAL_ERROR => IError::Param,
            Self::HAL_BUSY => IError::BusBusy,
            Self::HAL_TIMEOUT => IError::Overtime,
        }
    }
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn HAL_GetTick() -> u32;
}
