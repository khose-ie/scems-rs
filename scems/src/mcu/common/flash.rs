use crate::common::result::RetValue;

pub trait Flash
{
    fn erase_sector(&self, sector: u32) -> RetValue<()>;
    fn write(&self, address: u32, data: u8) -> RetValue<()>;
    fn write32(&self, address: u32, data: u32) -> RetValue<()>;
}
