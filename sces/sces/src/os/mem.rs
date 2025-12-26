use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::slice::from_raw_parts_mut;

use crate::os::RTOS;
use crate::value::RetValue;

pub trait IMemPool
{
    fn new(
        name: &str, buf: &'static mut [u8], block_size: u32, max_block_count: u32,
    ) -> RetValue<Self>
    where
        Self: Sized;

    fn name(&self) -> &str;
    fn block_size(&self) -> u32;
    fn block_count(&self) -> u32;
    fn max_block_count(&self) -> u32;
    fn alloc(&self) -> *mut u8;
    fn free(&self, mem: *mut u8);
}

struct MemPoolSpce<OS, const BKSZ: usize, const BKCT: usize>
where
    OS: RTOS,
{
    handle: Option<OS::MemPool>,
    mem_space: [[u8; BKSZ]; BKCT],
}

impl<OS, const BKSZ: usize, const BKCT: usize> MemPoolSpce<OS, BKSZ, BKCT>
where
    OS: RTOS,
{
    pub const fn new() -> Self
    {
        Self { handle: None, mem_space: [[0; BKSZ]; BKCT] }
    }

    pub fn initialize(&'static mut self, name: &str) -> RetValue<()>
    {
        self.handle = Some(OS::MemPool::new(
            name,
            unsafe { from_raw_parts_mut(&mut self.mem_space as *mut _ as *mut u8, BKSZ * BKCT) },
            BKSZ as u32,
            BKCT as u32,
        )?);

        Ok(())
    }

    pub fn alloc(&self) -> *mut u8
    {
        self.handle.as_ref().map_or(null_mut(), |x| x.alloc())
    }

    pub fn free(&self, mem: *mut u8)
    {
        #[allow(unused_must_use)]
        self.handle.as_ref().map_or((), |x| x.free(mem));
    }
}

pub struct MemorySpace<
    OS,
    const BK1SZ: usize,
    const BK1CT: usize,
    const BK2SZ: usize,
    const BK2CT: usize,
    const BK3SZ: usize,
    const BK3CT: usize,
    const BK4SZ: usize,
    const BK4CT: usize,
> where
    OS: RTOS,
{
    space1: MemPoolSpce<OS, BK1SZ, BK1CT>,
    space2: MemPoolSpce<OS, BK2SZ, BK2CT>,
    space3: MemPoolSpce<OS, BK3SZ, BK3CT>,
    space4: MemPoolSpce<OS, BK4SZ, BK4CT>,
}

impl<
        OS,
        const BK1SZ: usize,
        const BK1CT: usize,
        const BK2SZ: usize,
        const BK2CT: usize,
        const BK3SZ: usize,
        const BK3CT: usize,
        const BK4SZ: usize,
        const BK4CT: usize,
    > MemorySpace<OS, BK1SZ, BK1CT, BK2SZ, BK2CT, BK3SZ, BK3CT, BK4SZ, BK4CT>
where
    OS: RTOS,
{
    pub const fn new() -> Self
    {
        Self {
            space1: MemPoolSpce::new(),
            space2: MemPoolSpce::new(),
            space3: MemPoolSpce::new(),
            space4: MemPoolSpce::new(),
        }
    }

    pub fn initialize(&'static mut self) -> RetValue<()>
    {
        self.space1.initialize("MemorySpace1")?;
        self.space2.initialize("MemorySpace2")?;
        self.space3.initialize("MemorySpace3")?;
        self.space4.initialize("MemorySpace4")?;
        Ok(())
    }
}

unsafe impl<
        OS,
        const BK1SZ: usize,
        const BK1CT: usize,
        const BK2SZ: usize,
        const BK2CT: usize,
        const BK3SZ: usize,
        const BK3CT: usize,
        const BK4SZ: usize,
        const BK4CT: usize,
    > GlobalAlloc for MemorySpace<OS, BK1SZ, BK1CT, BK2SZ, BK2CT, BK3SZ, BK3CT, BK4SZ, BK4CT>
where
    OS: RTOS,
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8
    {
        if layout.size() <= BK1SZ as usize
        {
            self.space1.alloc()
        }
        else if layout.size() <= BK2SZ as usize
        {
            self.space2.alloc()
        }
        else if layout.size() <= BK3SZ as usize
        {
            self.space3.alloc()
        }
        else if layout.size() <= BK4SZ as usize
        {
            self.space4.alloc()
        }
        else
        {
            null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
    {
        if layout.size() <= BK1SZ as usize
        {
            self.space1.free(ptr);
        }
        else if layout.size() <= BK2SZ as usize
        {
            self.space2.free(ptr);
        }
        else if layout.size() <= BK3SZ as usize
        {
            self.space3.free(ptr);
        }
        else if layout.size() <= BK4SZ as usize
        {
            self.space4.free(ptr);
        }
    }
}
