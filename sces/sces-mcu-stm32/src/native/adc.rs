//! The RUST translation of data struct and function declerations of ADC in STM32 HAL libraries.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::dma::DMA_HandleTypeDef;
use super::{FunctionalState, HAL_LockTypeDef, HAL_StatusTypeDef};

#[repr(C)]
pub struct ADC_HandleTypeDef
{
    pub Instance: *mut ADC_TypeDef,
    pub Init: ADC_InitTypeDef,
    pub NbrOfCurrentConversionRank: u32,
    pub DMA_Handle: *mut DMA_HandleTypeDef,
    pub Lock: HAL_LockTypeDef,
    pub State: u32,
    pub ErrorCode: u32,
}

#[repr(C)]
pub struct ADC_TypeDef {}

#[repr(C)]
pub struct ADC_InitTypeDef
{
    pub ClockPrescaler: u32,
    pub Resolution: u32,
    pub DataAlign: u32,
    pub ScanConvMode: u32,
    pub EOCSelection: u32,
    pub ContinuousConvMode: FunctionalState,
    pub NbrOfConversion: u32,
    pub DiscontinuousConvMode: FunctionalState,
    pub NbrOfDiscConversion: u32,
    pub ExternalTrigConv: u32,
    pub ExternalTrigConvEdge: u32,
    pub DMAContinuousRequests: FunctionalState,
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn HAL_ADC_Start(adc: *mut ADC_HandleTypeDef) -> HAL_StatusTypeDef;
    pub fn HAL_ADC_Stop(adc: *mut ADC_HandleTypeDef) -> HAL_StatusTypeDef;
    pub fn HAL_ADC_PollForConversion(adc: *mut ADC_HandleTypeDef, Timeout: u32) -> HAL_StatusTypeDef;
    pub fn HAL_ADC_PollForEvent(adc: *mut ADC_HandleTypeDef, EventType: u32, Timeout: u32) -> HAL_StatusTypeDef;
    pub fn HAL_ADC_Start_IT(adc: *mut ADC_HandleTypeDef) -> HAL_StatusTypeDef;
    pub fn HAL_ADC_Stop_IT(adc: *mut ADC_HandleTypeDef) -> HAL_StatusTypeDef;
    pub fn HAL_ADC_IRQHandler(adc: *mut ADC_HandleTypeDef);
    pub fn HAL_ADC_Start_DMA(adc: *mut ADC_HandleTypeDef, pData: *mut u32, Length: u32) -> HAL_StatusTypeDef;
    pub fn HAL_ADC_Stop_DMA(adc: *mut ADC_HandleTypeDef) -> HAL_StatusTypeDef;
    pub fn HAL_ADC_GetValue(adc: *mut ADC_HandleTypeDef) -> u32;
}
