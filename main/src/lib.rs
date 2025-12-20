#![no_std]

// mod example;
mod model;

use core::panic::PanicInfo;

#[inline]
#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn app_main()
{
    #[allow(unused_must_use)]
    #[cfg(feature = "challen-v2-f429")]
    model::challen_v2_f429::app_main();
    // example::example();
}

#[panic_handler]
fn app_panic(_info: &PanicInfo) -> !
{
    loop
    {}
}
