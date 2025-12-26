use core::alloc::GlobalAlloc;
use core::ffi::{c_void, CStr};

use sces::os::mem::IMemPool;
use sces::os::RTOS;
use sces::value::{ErrValue, RetValue};

use crate::os::{native::*, MWOS};

#[global_allocator]
static MEM_POOL_AGENT: MemPoolAgent = MemPoolAgent;

struct MemPoolAgent;

unsafe impl GlobalAlloc for MemPoolAgent
{
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8
    {
        unsafe { sces_os_malloc(layout.size() as u32) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout)
    {
        unsafe { sces_os_free(ptr as *mut c_void, layout.size() as u32) };
    }
}

pub struct MemPool
{
    handle: ScesMemPoolHandle,
}

impl IMemPool for MemPool
{
    fn new(
        name: &str, buf: &'static mut [u8], block_size: u32, max_block_count: u32,
    ) -> RetValue<Self>
    where
        Self: Sized,
    {
        let handle = unsafe {
            sces_mem_pool_create_static(
                name.as_ptr(),
                buf.as_mut_ptr(),
                block_size,
                max_block_count,
            )
        };

        (!handle.is_null()).then_some(Self { handle }).ok_or(ErrValue::LowLevelFailure)
    }

    fn name(&self) -> &str
    {
        unsafe {
            CStr::from_ptr(sces_mem_pool_name(self.handle) as *const i8).to_str().unwrap_or("")
        }
    }

    fn block_size(&self) -> u32
    {
        unsafe { sces_mem_pool_block_size(self.handle) }
    }

    fn block_count(&self) -> u32
    {
        unsafe { sces_mem_pool_block_count(self.handle) }
    }

    fn max_block_count(&self) -> u32
    {
        unsafe { sces_mem_pool_max_block_count(self.handle) }
    }

    fn alloc(&self) -> *mut u8
    {
        unsafe { sces_mem_pool_alloc(self.handle, MWOS::WAIT_500) as *mut u8 }
    }

    fn free(&self, mem: *mut u8)
    {
        unsafe { sces_mem_pool_free(self.handle, mem as *mut c_void) };
    }
}
