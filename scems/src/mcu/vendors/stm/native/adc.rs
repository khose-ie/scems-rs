#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::common::{FunctionalState, HAL_Lock, HAL_Status};
use super::dma::DMA;

#[repr(C)]
pub struct ADC
{
    pub Instance: *mut ADC_Base,
    pub Init: ADC_Init,
    pub NbrOfCurrentConversionRank: u32,
    pub DMA_Handle: *mut DMA,
    pub Lock: HAL_Lock,
    pub State: u32,
    pub ErrorCode: u32,
}

#[repr(C)]
pub struct ADC_Base {}

#[repr(C)]
pub struct ADC_Init
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
    pub fn HAL_ADC_Start(adc: *mut ADC) -> HAL_Status;
    pub fn HAL_ADC_Stop(adc: *mut ADC) -> HAL_Status;
    pub fn HAL_ADC_PollForConversion(adc: *mut ADC, Timeout: u32) -> HAL_Status;
    pub fn HAL_ADC_PollForEvent(adc: *mut ADC, EventType: u32, Timeout: u32) -> HAL_Status;
    pub fn HAL_ADC_Start_IT(adc: *mut ADC) -> HAL_Status;
    pub fn HAL_ADC_Stop_IT(adc: *mut ADC) -> HAL_Status;
    pub fn HAL_ADC_IRQHandler(adc: *mut ADC);
    pub fn HAL_ADC_Start_DMA(adc: *mut ADC, pData: *mut u32, Length: u32) -> HAL_Status;
    pub fn HAL_ADC_Stop_DMA(adc: *mut ADC) -> HAL_Status;
    pub fn HAL_ADC_GetValue(adc: *mut ADC) -> u32;
}
