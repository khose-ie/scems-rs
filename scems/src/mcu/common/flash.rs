use crate::common::result::IResult;

pub trait Flash
{
    fn erase_sector(&self, sector: u32) -> IResult<()>;
    fn write(&self, address: u32, data: u8) -> IResult<()>;
    fn write32(&self, address: u32, data: u32) -> IResult<()>;
}
