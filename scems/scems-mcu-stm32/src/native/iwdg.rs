#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::HAL_StatusTypeDef;

#[repr(C)]
pub struct IWDG_HandleTypeDef
{
    pub Instance: *mut IWDG_TypeDef,
    pub Init: IWDG_InitTypeDef,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct IWDG_TypeDef {}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct IWDG_InitTypeDef
{
    Prescaler: u32,
    Reload: u32,
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
extern "C" {
    pub fn HAL_IWDG_Refresh(iwdg: *mut IWDG_HandleTypeDef) -> HAL_StatusTypeDef;
}
