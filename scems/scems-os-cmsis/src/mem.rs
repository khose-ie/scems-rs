use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use core::ops::Not;
use core::ptr::{null, null_mut};

use scems::value::{ErrValue, RetValue};

use crate::native::*;

const MEM_POOL_NUM: usize = 4;

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

    pub fn create_pool(&mut self, count: u32, size: u32, address: &'static [u8]) -> RetValue<()>
    {
        let mut attribute = osMemoryPoolAttr_t::default();

        attribute.mp_mem = address.as_ptr() as *mut c_void;
        attribute.mp_size = count * size;

        self.block_count = count;
        self.block_size = size;

        self.handle = unsafe { osMemoryPoolNew(count, size, &attribute) };
        self.handle.is_null().not().then_some(()).ok_or(ErrValue::MemAllocFailure)
    }
}

struct MemSpace
{
    mem: [MemBlock; MEM_POOL_NUM],
}

impl MemSpace
{
    pub const fn new() -> Self
    {
        Self { mem: [MemBlock::new(); MEM_POOL_NUM] }
    }

    pub fn initialize(&mut self, mem: &[MemZone; MEM_POOL_NUM]) -> RetValue<()>
    {
        for (i, zone) in mem.iter().enumerate()
        {
            self.mem[i].create_pool(zone.block_count, zone.block_size, zone.address)?;
        }

        Ok(())
    }
}

unsafe impl GlobalAlloc for MemSpace
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8
    {
        self.mem
            .iter()
            .find(|mem| layout.size() <= mem.block_size as usize)
            .map(|mem| unsafe { osMemoryPoolAlloc(mem.handle, 100) as *mut u8 })
            .unwrap_or(null_mut())
    }

    /// Deallocate the memory block pointed to by `ptr` with the given `layout`.
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout)
    {
        self.mem
            .iter()
            .find(|mem| layout.size() <= mem.block_size as usize)
            .map(|mem| unsafe { osMemoryPoolFree(mem.handle, ptr as *mut c_void) });
    }
}

#[global_allocator]
static mut MEM_SPACE: MemSpace = MemSpace::new();

/// The public function to initialize the global mem space for alloc feature.
/// It can register max 4 mem zones as the mem pool with different block size.
pub fn initialize_mem_space(mem: &[MemZone; 4]) -> RetValue<()>
{
    #[allow(static_mut_refs)]
    unsafe {
        MEM_SPACE.initialize(mem)
    }
}
