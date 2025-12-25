use core::ffi::{c_void, CStr};

use sces::os::mem::IMemPool;
use sces::os::RTOS;
use sces::value::{ErrValue, RetValue};

use crate::native::*;
use crate::CMSISOS;

pub struct MemPool
{
    handle: osMemoryPoolId_t,
}

impl IMemPool for MemPool
{
    fn new(
        name: &str, buf: &'static mut [u8], block_size: u32, max_block_count: u32,
    ) -> RetValue<Self>
    where
        Self: Sized,
    {
        let mut attribute = osMemoryPoolAttr_t::default();

        attribute.name = name.as_ptr() as *const i8;
        attribute.mp_mem = buf.as_mut_ptr() as *mut c_void;
        attribute.mp_size = block_size * max_block_count;

        let handle = unsafe { osMemoryPoolNew(max_block_count, block_size, &attribute) };

        (!handle.is_null()).then_some(Self { handle }).ok_or(ErrValue::LowLevelFailure)
    }

    fn name(&self) -> &str
    {
        unsafe { CStr::from_ptr(osMemoryPoolGetName(self.handle)).to_str().unwrap_or("") }
    }

    fn block_size(&self) -> u32
    {
        unsafe { osMemoryPoolGetBlockSize(self.handle) }
    }

    fn block_count(&self) -> u32
    {
        unsafe { osMemoryPoolGetCount(self.handle) }
    }

    fn max_block_count(&self) -> u32
    {
        unsafe { osMemoryPoolGetCapacity(self.handle) }
    }

    fn alloc(&self) -> *mut u8
    {
        unsafe { osMemoryPoolAlloc(self.handle, CMSISOS::WAIT_200) as *mut u8 }
    }

    fn free(&self, mem: *mut u8)
    {
        unsafe { osMemoryPoolFree(self.handle, mem as *mut c_void) };
    }
}
