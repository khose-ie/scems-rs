use core::ffi::c_void;
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::ptr::{copy, drop_in_place, null, write_bytes};
use core::slice::{from_raw_parts, from_raw_parts_mut};

use scems::value::{ErrValue, RetValue};
use scems_os::mem::{IMemBlock, IMemBlockHeap, IMemCache, IMemPool, IMemQueue};

use crate::native::*;

const MEM_POOL_NUM: usize = 5;

static mut MEM_POOLS: [MemPool; MEM_POOL_NUM] = [MemPool::new(); MEM_POOL_NUM];

#[derive(Clone, Copy)]
pub struct MemPool
{
    handle: osMemoryPoolId_t,
    block_size: u32,
}

impl MemPool
{
    pub const fn new() -> Self
    {
        Self { handle: null(), block_size: 0 }
    }

    pub fn from(handle: osMemoryPoolId_t, block_size: u32) -> Self
    {
        Self { handle, block_size }
    }

    pub(crate) fn initialize(&mut self, handle: osMemoryPoolId_t, block_size: u32)
    {
        if block_size > 0
        {
            if !self.handle.is_null()
            {
                self.handle = handle;
                self.block_size = block_size;
            }
        }
    }

    #[allow(static_mut_refs)]
    pub(crate) unsafe fn choice(size: u32) -> RetValue<&'static dyn IMemPool<osMemoryPoolId_t>>
    {
        let mut choose: Option<&'static dyn IMemPool<osMemoryPoolId_t>> = None;

        for mem_pool in MEM_POOLS.as_ref()
        {
            if size.le(&mem_pool.block_size())
            {
                choose = Some(mem_pool);
                break;
            }
        }

        Ok(choose.ok_or(ErrValue::InstanceNotFound)?)
    }

    pub(crate) unsafe fn malloc(size: u32) -> RetValue<(*mut c_void, u32)>
    {
        const MEM_ALLOC_TIME: u32 = 100;

        let pool = MemPool::choice(size)?;

        let mem = osMemoryPoolAlloc(pool.handle(), MEM_ALLOC_TIME);

        if mem.is_null()
        {
            return Err(ErrValue::MemAlloc);
        }

        Ok((mem, pool.block_size()))
    }

    pub(crate) unsafe fn mfree(size: u32, mem: *mut c_void)
    {
        if let Ok(mem_pool) = MemPool::choice(size)
        {
            osMemoryPoolFree(mem_pool.handle(), mem);
        }
    }
}

impl IMemPool<osMemoryPoolId_t> for MemPool
{
    #[inline]
    fn handle(&self) -> osMemoryPoolId_t
    {
        self.handle
    }

    #[inline]
    fn block_size(&self) -> u32
    {
        self.block_size
    }
}

unsafe impl Sync for MemPool {}

pub struct MemBlock<T>
{
    mem_space: *mut T,
}

impl<T> MemBlock<T>
{
    pub fn new() -> RetValue<Self>
    {
        let (mem, _) = unsafe { MemPool::malloc(size_of::<T>() as u32) }?;
        unsafe { write_bytes(mem, 0, size_of::<T>()) };
        Ok(MemBlock { mem_space: mem.cast() })
    }

    pub fn from(sample: T) -> RetValue<Self>
    {
        let (mem, _) = unsafe { MemPool::malloc(size_of::<T>() as u32) }?;
        unsafe { *(mem.cast()) = sample };
        Ok(MemBlock { mem_space: mem.cast() })
    }
}

impl<T> Drop for MemBlock<T>
{
    fn drop(&mut self)
    {
        if !self.mem_space.is_null()
        {
            unsafe { drop_in_place(self.mem_space) };
            unsafe { MemPool::mfree(size_of::<T>() as u32, self.mem_space.cast()) };
        }
    }
}

impl<T> Deref for MemBlock<T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        unsafe { self.mem_space.as_ref().unwrap_unchecked() }
    }
}

impl<T> DerefMut for MemBlock<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        unsafe { self.mem_space.as_mut().unwrap_unchecked() }
    }
}

impl<T> AsRef<T> for MemBlock<T>
{
    fn as_ref(&self) -> &T
    {
        unsafe { self.mem_space.as_ref().unwrap_unchecked() }
    }
}

impl<T> AsMut<T> for MemBlock<T>
{
    fn as_mut(&mut self) -> &mut T
    {
        unsafe { self.mem_space.as_mut().unwrap_unchecked() }
    }
}

impl<T> IMemBlock<T> for MemBlock<T>
{
    fn set(&mut self, value: T)
    {
        if !self.mem_space.is_null()
        {
            unsafe { *self.mem_space = value };
        }
    }

    fn clean(&mut self)
    {
        if !self.mem_space.is_null()
        {
            unsafe { drop_in_place(self.mem_space) };
            unsafe { write_bytes(self.mem_space, 0, size_of::<T>()) };
        }
    }
}

pub struct MemBlockHeap<T>
{
    mem_space: *mut T,
}

impl<T> MemBlockHeap<T>
{
    pub fn new() -> RetValue<Self>
    {
        let mem = unsafe { MemHeap::malloc(size_of::<T>() as u32) }?;
        unsafe { write_bytes(mem, 0, size_of::<T>()) };
        Ok(MemBlockHeap { mem_space: mem.cast() })
    }

    pub fn from(sample: T) -> RetValue<Self>
    {
        let mem = unsafe { MemHeap::malloc(size_of::<T>() as u32) }?;
        unsafe { *(mem.cast()) = sample };
        Ok(MemBlockHeap { mem_space: mem.cast() })
    }
}

impl<T> Drop for MemBlockHeap<T>
{
    fn drop(&mut self)
    {
        if !self.mem_space.is_null()
        {
            unsafe { drop_in_place(self.mem_space) };
            unsafe { MemHeap::free(self.mem_space as *mut c_void) };
        }
    }
}

impl<T> Deref for MemBlockHeap<T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        unsafe { self.mem_space.as_ref().unwrap_unchecked() }
    }
}

impl<T> DerefMut for MemBlockHeap<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        unsafe { self.mem_space.as_mut().unwrap_unchecked() }
    }
}

impl<T> IMemBlockHeap<T> for MemBlockHeap<T>
{
    fn set(&mut self, value: T)
    {
        if !self.mem_space.is_null()
        {
            unsafe { drop_in_place(self.mem_space) };
            unsafe { *self.mem_space = value };
        }
    }

    fn clean(&mut self)
    {
        if !self.mem_space.is_null()
        {
            unsafe { drop_in_place(self.mem_space) };
            unsafe { write_bytes(self.mem_space, 0, size_of::<T>()) };
        }
    }
}

pub struct MemCache<const N: usize>
{
    mem_space: *mut u8,
    size: usize,
}

impl<const N: usize> MemCache<N>
{
    pub fn new() -> RetValue<Self>
    {
        let (mem, size) = unsafe { MemPool::malloc(N as u32)? };
        Ok(MemCache { mem_space: mem.cast(), size: size as usize })
    }
}

impl<const N: usize> Index<usize> for MemCache<N>
{
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output
    {
        unsafe { self.mem_space.add(index).as_ref().unwrap_unchecked() }
    }
}

impl<const N: usize> IndexMut<usize> for MemCache<N>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output
    {
        unsafe { self.mem_space.add(index).as_mut().unwrap_unchecked() }
    }
}

impl<const N: usize> AsRef<[u8]> for MemCache<N>
{
    fn as_ref(&self) -> &[u8]
    {
        unsafe { from_raw_parts(self.mem_space, self.size) }
    }
}

impl<const N: usize> AsMut<[u8]> for MemCache<N>
{
    fn as_mut(&mut self) -> &mut [u8]
    {
        unsafe { from_raw_parts_mut(self.mem_space, self.size) }
    }
}

impl<const N: usize> IMemCache<N> for MemCache<N>
{
    fn set(&mut self, value: &[u8])
    {
        if !self.mem_space.is_null()
        {
            unsafe { self.mem_space.copy_from(value.as_ptr(), value.len()) };
        }
    }

    fn fill(&mut self, value: u8, size: usize)
    {
        if !self.mem_space.is_null()
        {
            unsafe { write_bytes(self.mem_space, value, size) };
        }
    }

    fn clean(&mut self)
    {
        if !self.mem_space.is_null()
        {
            unsafe { write_bytes(self.mem_space, 0, self.size) };
        }
    }
}

pub struct MemQueue<T, const N: usize>
where
    T: Clone,
{
    mem_space: *mut T,
    use_num: usize,
    max_num: usize,
}

impl<T, const N: usize> MemQueue<T, N>
where
    T: Clone,
{
    pub fn new() -> RetValue<Self>
    {
        let (mem, size) = unsafe { MemPool::malloc((size_of::<T>() * N) as u32)? };
        Ok(MemQueue { mem_space: mem.cast(), use_num: 0, max_num: size as usize / size_of::<T>() })
    }

    pub fn iter(&self) -> MemQueueIter<'_, T>
    {
        MemQueueIter { ptr: self.mem_space, index: 0, end: self.use_num, _marker: core::marker::PhantomData }
    }

    pub fn iter_mut(&mut self) -> MemQueueIterMut<'_, T>
    {
        MemQueueIterMut { ptr: self.mem_space, index: 0, end: self.use_num, _marker: core::marker::PhantomData }
    }
}

impl<T, const N: usize> IMemQueue<T, N> for MemQueue<T, N>
where
    T: Clone,
{
    fn push(&mut self, data: &T) -> RetValue<usize>
    {
        if self.use_num >= self.max_num
        {
            self.expend()?;
        }

        let existence = self.search(data);

        if existence.is_ok()
        {
            return existence;
        }

        let next = self.use_num;

        self[next] = data.clone();
        self.use_num += 1;

        Ok(self.use_num as usize)
    }

    fn remove(&mut self, data: &T)
    {
        self.remove_position(self.search(data).ok().unwrap_or(self.use_num));
    }

    fn remove_position(&mut self, index: usize)
    {
        if index >= self.use_num
        {
            return;
        }

        self.use_num -= 1;
        let last = self.use_num;
        self[index] = self[last].clone();

        unsafe { write_bytes(&mut self[last], 0, size_of::<T>()) };
    }

    fn search(&self, data: &T) -> RetValue<usize>
    {
        for index in 0..self.use_num
        {
            if core::ptr::addr_eq(self.mem_space.wrapping_add(index), data)
            {
                return Ok(index);
            }
        }

        Err(ErrValue::InstanceNotFound)
    }

    fn expend(&mut self) -> RetValue<()>
    {
        let size = self.max_num * size_of::<T>();
        let (mem, new_size) = unsafe { MemPool::malloc((size * 2) as u32)? };

        unsafe {
            copy(self.mem_space, mem.cast(), size);
            MemPool::mfree(size as u32, self.mem_space.cast());
        }

        self.mem_space = mem.cast();
        self.max_num = new_size as usize / size_of::<T>();

        Ok(())
    }

    fn num(&self) -> usize
    {
        self.use_num as usize
    }
}

impl<T, const N: usize> Index<usize> for MemQueue<T, N>
where
    T: Clone,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output
    {
        unsafe { self.mem_space.add(index).as_ref().unwrap_unchecked() }
    }
}

impl<T, const N: usize> IndexMut<usize> for MemQueue<T, N>
where
    T: Clone,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output
    {
        unsafe { self.mem_space.add(index).as_mut().unwrap_unchecked() }
    }
}

pub struct MemQueueIter<'a, T>
{
    ptr: *const T,
    index: usize,
    end: usize,
    _marker: core::marker::PhantomData<&'a T>,
}

impl<'a, T> Iterator for MemQueueIter<'a, T>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.index >= self.end
        {
            return None;
        }

        let item = unsafe { &*self.ptr.add(self.index) };
        self.index += 1;
        Some(item)
    }
}

pub struct MemQueueIterMut<'a, T>
{
    ptr: *mut T,
    index: usize,
    end: usize,
    _marker: core::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for MemQueueIterMut<'a, T>
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.index >= self.end
        {
            return None;
        }
        let item = unsafe { &mut *self.ptr.add(self.index) };
        self.index += 1;
        Some(item)
    }
}

#[allow(static_mut_refs)]
pub fn assign_mem_pool(mem_pool: &dyn IMemPool<osMemoryPoolId_t>) -> RetValue<()>
{
    for mem_pool_sample in unsafe { MEM_POOLS.as_mut() }
    {
        if mem_pool_sample.handle().is_null()
        {
            mem_pool_sample.initialize(mem_pool.handle(), mem_pool.block_size());
            return Ok(());
        }
    }

    Err(ErrValue::StackOverflow)
}

pub struct MemHeap {}

impl MemHeap
{
    #[inline]
    pub unsafe fn malloc(size: u32) -> RetValue<*mut c_void>
    {
        let space = malloc(size);

        if space.is_null()
        {
            return Err(ErrValue::MemAlloc);
        }

        Ok(space)
    }

    #[inline]
    pub unsafe fn free(ptr: *mut c_void)
    {
        free(ptr);
    }
}

unsafe extern "C" {
    pub fn malloc(size: u32) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
}
