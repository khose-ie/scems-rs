#![no_std]

mod model;

use core::panic::PanicInfo;

#[no_mangle]
#[rustfmt::skip]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn app_main()
{
    model::app_main();
    loop {}
}

#[rustfmt::skip]
#[panic_handler]
fn app_panic(_info: &PanicInfo) -> !
{
    loop {}
}
