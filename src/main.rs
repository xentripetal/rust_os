#![no_std] // Disable standard library since that is OS depednent
#![no_main] // disable rust entry points

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";


// this function is the entry point, since the linker looks for a function
// named `_start` by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}


#[panic_handler] // no_std means no panic support by default, make our own
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
