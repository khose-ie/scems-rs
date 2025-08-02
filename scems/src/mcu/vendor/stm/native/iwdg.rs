#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::common::HAL_StatusTypeDef;

#[repr(C)]
pub struct IWDG
{
    pub Instance: *mut IWDG_Base,
    pub Init: IWDG_Init,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct IWDG_Base {}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct IWDG_Init
{
    Prescaler: u32,
    Reload: u32,
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
extern "C" {
    pub fn HAL_IWDG_Refresh(iwdg: *mut IWDG) -> HAL_StatusTypeDef;
}
