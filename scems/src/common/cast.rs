pub trait CastOpt<T>
{
    fn cast_opt(self) -> Option<T>;
}

impl<T: ?Sized> CastOpt<*const T> for *const T
{
    #[inline]
    #[rustfmt::skip]
    fn cast_opt(self) -> Option<*const T>
    {
        if self.is_null() { None } else { Some(self) }
    }
}

impl<T: ?Sized> CastOpt<*mut T> for *mut T
{
    #[inline]
    #[rustfmt::skip]
    fn cast_opt(self) -> Option<*mut T>
    {
        if self.is_null() { None } else { Some(self) }
    }
}

pub trait CastPtr<T>
{
    fn cast_ptr(&self) -> *const T;
    fn cast_ptr_mut(&mut self) -> *mut T;
}

impl<T> CastPtr<T> for T
{
    #[inline]
    fn cast_ptr(&self) -> *const T
    {
        self as *const T
    }

    #[inline]
    fn cast_ptr_mut(&mut self) -> *mut T
    {
        self as *mut T
    }
}
