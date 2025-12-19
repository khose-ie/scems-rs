#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use sces_mcu::io::IoState;

use super::HAL_StatusTypeDef;

#[repr(C)]
pub struct GPIO_TypeDef {}

#[repr(C)]
pub enum GPIO_PinState
{
    GPIO_PIN_RESET = 0x00,
    GPIO_PIN_SET = 0x01,
}

impl From<IoState> for GPIO_PinState
{
    fn from(value: IoState) -> Self
    {
        match value
        {
            IoState::Reset => GPIO_PinState::GPIO_PIN_RESET,
            IoState::Set => GPIO_PinState::GPIO_PIN_SET,
        }
    }
}

impl From<GPIO_PinState> for IoState
{
    fn from(value: GPIO_PinState) -> Self
    {
        match value
        {
            GPIO_PinState::GPIO_PIN_RESET => IoState::Reset,
            GPIO_PinState::GPIO_PIN_SET => IoState::Set,
        }
    }
}
#[rustfmt::skip]
#[allow(improper_ctypes)]
extern "C" {
    pub fn HAL_GPIO_ReadPin(GPIOx: *mut GPIO_TypeDef, GPIO_Pin: u16) -> GPIO_PinState;
    pub fn HAL_GPIO_WritePin(GPIOx: *mut GPIO_TypeDef, GPIO_Pin: u16, PinState: GPIO_PinState);
    pub fn HAL_GPIO_TogglePin(GPIOx: *mut GPIO_TypeDef, GPIO_Pin: u16);
    pub fn HAL_GPIO_LockPin(GPIOx: *mut GPIO_TypeDef, GPIO_Pin: u16) -> HAL_StatusTypeDef;
}
