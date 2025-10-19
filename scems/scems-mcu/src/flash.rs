use scems::value::RetValue;

pub type FlashDevice = &'static mut dyn FlashCtrl;

pub trait FlashCtrl
{
    fn erase_sector(&self, sector: u32) -> RetValue<()>;
    fn write(&self, address: u32, data: u8) -> RetValue<()>;
    fn write32(&self, address: u32, data: u32) -> RetValue<()>;
}
