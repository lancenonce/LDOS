// Let's create the LDOS
#![no_std]

use core::panic::PanicInfo;

/// called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in b"Hello LD!".iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}