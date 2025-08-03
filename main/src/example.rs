use scems::os::{common::task::TaskMain, vendors::task::TaskSample};

mod io_lamp;
mod uart_echo;

pub unsafe fn example()
{
    #[cfg(feature = "example-io-lamp")]
    io_lamp::main();

    #[cfg(feature = "example-uart-echo")]
    uart_echo::main();
}
