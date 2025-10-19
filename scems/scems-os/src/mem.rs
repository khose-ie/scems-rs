use core::ops::{Deref, DerefMut, Index, IndexMut};

use scems::value::RetValue;

pub trait IMemPool<T>
where
    T: Copy,
{
    fn handle(&self) -> T;
    fn block_size(&self) -> u32;
}

pub trait IMemBlock<T>
where
    Self: Deref + DerefMut,
{
    fn set(&mut self, value: T);
    fn clean(&mut self);
}

pub trait IMemBlockHeap<T>
where
    Self: Deref + DerefMut,
{
    fn set(&mut self, value: T);
    fn clean(&mut self);
}

pub trait IMemCache<const N: usize>
where
    Self: Index<usize> + IndexMut<usize> + AsRef<[u8]> + AsMut<[u8]>,
{
    fn set(&mut self, value: &[u8]);
    fn fill(&mut self, value: u8, size: usize);
    fn clean(&mut self);
}

pub trait IMemQueue<T, const N: usize>
where
    T: ?Sized + Clone,
    Self: Index<usize> + IndexMut<usize>,
{
    fn push(&mut self, data: &T) -> RetValue<usize>;
    fn remove(&mut self, data: &T);
    fn remove_position(&mut self, index: usize);
    fn search(&self, data: &T) -> RetValue<usize>;
    fn expend(&mut self) -> RetValue<()>;
    fn num(&self) -> usize;
}
