pub trait IKernel: Sync
{
    fn delay(time: u32);
    fn delay_interval(time: u32);
    fn systick_value() -> u32;
    fn cede();
    fn exit();
}
