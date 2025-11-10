pub trait Service
{
    fn initialize(exe: &'static Self)
    where
        Self: Sized;
}
