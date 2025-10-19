pub trait ISxMutex
{
    fn involve(&mut self);
    fn leave(&mut self);
    fn keep(&mut self);
    fn release(&mut self);
}
