use core::alloc::GlobalAlloc;
use core::ffi::c_void;
use core::ops::Not;
use core::ptr::{null, null_mut};

use scems::value::{ErrValue, RetValue};

use crate::native::*;

pub struct MemZone
{
    block_count: u32,
    block_size: u32,
    address: &'static [u8],
}

impl MemZone
{
    pub const fn new(block_count: u32, block_size: u32, address: &'static [u8]) -> Self
    {
        Self { block_count, block_size, address }
    }
}

#[derive(Clone, Copy)]
struct MemBlock
{
    handle: osMemoryPoolId_t,
    block_count: u32,
    block_size: u32,
}

impl MemBlock
{
    pub const fn new() -> Self
    {
        Self { handle: null(), block_count: 0, block_size: 0 }
    }
}

struct MemSpace
{
    mem: [MemBlock; 4],
}

impl MemSpace
{
    pub const fn new() -> Self
    {
        Self { mem: [MemBlock::new(); 4] }
    }

    pub fn initialize(&mut self, mem: &[MemZone; 4]) -> RetValue<()>
    {
        let mut attribute = osMemoryPoolAttr_t::default();

        for idx in 0..4
        {
            attribute.mp_mem = mem[idx].address.as_ptr() as *mut c_void;
            attribute.mp_size = mem[idx].block_count * mem[idx].block_size;

            self.mem[idx].block_count = mem[idx].block_count;
            self.mem[idx].block_size = mem[idx].block_size;
            self.mem[idx].handle =
                unsafe { osMemoryPoolNew(mem[idx].block_count, mem[idx].block_size, &attribute) };

            if self.mem[idx].handle.is_null()
            {
                return Err(ErrValue::InstanceCreate);
            }
        }

        Ok(())
    }

    pub fn finalize(&mut self)
    {
        for mem in self.mem.iter()
        {
            if mem.handle.is_null().not()
            {
                unsafe { osMemoryPoolDelete(mem.handle) };
            }
        }
    }

    pub fn choose_block(&self, size: u32) -> RetValue<osMemoryPoolId_t>
    {
        for mem in self.mem.iter()
        {
            if size <= mem.block_size
            {
                return Ok(mem.handle);
            }
        }

        Err(ErrValue::MemAlloc)
    }
}

unsafe impl GlobalAlloc for MemSpace
{
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8
    {
        if let Ok(mp) = self.choose_block(layout.size() as u32)
        {
            osMemoryPoolAlloc(mp, 100) as *mut u8
        }
        else
        {
            null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout)
    {
        if let Ok(mp) = self.choose_block(layout.size() as u32)
        {
            osMemoryPoolFree(mp, ptr as *mut c_void);
        }
    }
}

#[global_allocator]
static mut MEM_SPACE: MemSpace = MemSpace::new();

/// The public function to initialize the global mem space for alloc feature.
/// It can register max 4 mem zones as the mem pool with different block size.
pub fn initialize_mem_space(mem: &[MemZone; 4]) -> RetValue<()>
{
    #[allow(static_mut_refs)]
    unsafe { MEM_SPACE.initialize(mem) }
}
