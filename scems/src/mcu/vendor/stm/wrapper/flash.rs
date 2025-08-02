use crate::common::result::RetValue;
use crate::mcu::common::flash::Flash;
use crate::mcu::vendor::stm::native::flash::*;

pub struct OnChipFlashDevice {}

impl OnChipFlashDevice
{
    pub fn new() -> Self
    {
        OnChipFlashDevice {}
    }
}

impl OnChipFlashDevice
{
    unsafe fn write_common(&self, kind: u32, address: u32, data: u64) -> RetValue<()>
    {
        HAL_FLASH_Lock().ok()?;
        let value = HAL_FLASH_Program(kind, address, data);
        HAL_FLASH_Unlock();
        value.into()
    }
}

impl Flash for OnChipFlashDevice
{
    fn erase_sector(&self, sector: u32) -> RetValue<()>
    {
        let mut error: u32 = 0;
        let mut erase_init: FLASH_EraseInitTypeDef = Default::default();

        erase_init.TypeErase = FLASH_TYPEERASE_SECTORS;
        erase_init.Banks = 0;
        erase_init.Sector = sector;
        erase_init.NbSectors = 1;
        erase_init.VoltageRange = FLASH_VOLTAGE_RANGE3;

        unsafe {
            HAL_FLASH_Lock().ok()?;
            let value = HAL_FLASHEx_Erase(&mut erase_init, &mut error);
            HAL_FLASH_Unlock();
            value.into()
        }
    }

    fn write(&self, address: u32, data: u8) -> RetValue<()>
    {
        unsafe { self.write_common(FLASH_TYPEPROGRAM_BYTE, address, data as u64) }
    }

    fn write32(&self, address: u32, data: u32) -> RetValue<()>
    {
        unsafe { self.write_common(FLASH_TYPEPROGRAM_WORD, address, data as u64) }
    }
}
