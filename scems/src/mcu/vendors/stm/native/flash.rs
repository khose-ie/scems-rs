#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::common::HAL_Status;

pub const FLASH_TYPEPROGRAM_BYTE: u32 = 0x00000000;
pub const FLASH_TYPEPROGRAM_HALFWORD: u32 = 0x00000001;
pub const FLASH_TYPEPROGRAM_WORD: u32 = 0x00000002;
pub const FLASH_TYPEPROGRAM_DOUBLEWORD: u32 = 0x00000003;

pub const FLASH_TYPEERASE_SECTORS: u32 = 0x00000000;
pub const FLASH_TYPEERASE_MASSERASE: u32 = 0x00000001;

pub const FLASH_VOLTAGE_RANGE1: u32 = 0x0000_0000;
pub const FLASH_VOLTAGE_RANGE2: u32 = 0x0000_0001;
pub const FLASH_VOLTAGE_RANGE3: u32 = 0x0000_0002;
pub const FLASH_VOLTAGE_RANGE4: u32 = 0x0000_0003;

pub const OB_WRPSTATE_DISABLE: u32 = 0x00000000;
pub const OB_WRPSTATE_ENABLE: u32 = 0x00000001;

#[repr(C)]
#[derive(Default)]
pub struct FLASH_EraseInit
{
    pub TypeErase: u32,
    pub Banks: u32,
    pub Sector: u32,
    pub NbSectors: u32,
    pub VoltageRange: u32,
}

#[repr(C)]
pub struct FLASH_OBProgramInit
{
    OptionType: u32,
    WRPState: u32,
    WRPSector: u32,
    Banks: u32,
    RDPLevel: u32,
    BORLevel: u32,
    USERConfig: u8,
}

#[rustfmt::skip]
#[allow(improper_ctypes)]
extern "C" {
    pub fn HAL_FLASH_Program(TypeProgram: u32, Address: u32, Data: u64) -> HAL_Status;
    pub fn HAL_FLASH_Program_IT(TypeProgram: u32, Address: u32, Data: u64) -> HAL_Status;
    pub fn HAL_FLASH_Unlock() -> HAL_Status;
    pub fn HAL_FLASH_Lock() -> HAL_Status;
    pub fn HAL_FLASH_OB_Unlock() -> HAL_Status;
    pub fn HAL_FLASH_OB_Lock() -> HAL_Status;
    pub fn HAL_FLASH_OB_Launch() -> HAL_Status;
    pub fn HAL_FLASHEx_Erase(pEraseInit: *mut FLASH_EraseInit, SectorError: *mut u32) -> HAL_Status;
    pub fn HAL_FLASHEx_Erase_IT(pEraseInit: *mut FLASH_EraseInit) -> HAL_Status;
    pub fn HAL_FLASHEx_OBProgram(pOBInit: *mut FLASH_OBProgramInit) -> HAL_Status;
    pub fn HAL_FLASHEx_OBGetConfig(pOBInit: *mut FLASH_OBProgramInit);
}
