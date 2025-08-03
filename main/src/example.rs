mod io_lamp;

pub fn example()
{
    #[cfg(feature = "example-io-lamp")]
    io_lamp::main();

}
