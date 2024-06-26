#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    //vga_buffer::print_something();
    vga_buffer::printvga("Hola Mundo!!!");
    vga_buffer::printvga("Hello World!");
    loop {}
}

use core::panic::PanicInfo;
/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}