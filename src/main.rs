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

static HELLO: &[u8] = b"Hello, World!";

// dont mangle the function name
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // cast to raw pointer
    let vga_buffer = 0xb8000 as *mut u8;

    // iterate over bytes of static HELLO byte string
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {

    }
}