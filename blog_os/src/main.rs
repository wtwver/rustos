#![no_std]
#![no_main]

mod vga_buffer;


use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


static HELLO: &[u8] = b"Hello World111222!";
fn print(name: &[u8]) {
    let vga_buffer = 0xb8000 as *mut u8;
    // static HELLO: &[u8] = name;

    for (i, &byte) in name.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    print(b"ggggg");
    loop {}
}