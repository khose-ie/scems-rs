use core::fmt::Write;
use core::ops::DerefMut;

use alloc::boxed::Box;

pub struct ConsoleCache
{
    data: Box<[u8; 256]>,
    length: usize,
}

impl ConsoleCache
{
    pub fn new() -> Self
    {
        Self { data: Box::new([0; 256]), length: 0 }
    }

    pub fn as_bytes(&self) -> &[u8]
    {
        &self.data[..self.length]
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8]
    {
        self.data.deref_mut()
    }

    pub fn clean(&mut self)
    {
        self.data.fill(0);
        self.length = 0;
    }

    pub fn set_length(&mut self, length: usize)
    {
        self.length = length;
    }
}

impl Write for ConsoleCache
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result
    {
        let content = s.as_bytes();
        let size = content.len().min(self.data.len().saturating_sub(self.length));

        if size > 0
        {
            self.data[self.length..self.length + size].copy_from_slice(&content[..size]);
            self.length += size;
        }

        Ok(())
    }
}
