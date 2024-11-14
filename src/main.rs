#![no_std]
#![no_main]

use core::panic::PanicInfo;

// define our own panic handler to replace the one
// provided by the OS
#[panic_handler]
// _info contains the file and line where the panic
// happened
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// dont mangle the function name
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {

    }
}