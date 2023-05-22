#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[macro_export]
macro_rules! println {
    () => (print(b"Default"));
    ($arg:expr) => (print($arg,0x0));
    ($($arg:tt)+) => (print($($arg)+));
    // ($($arg:tt)*) => (print($($arg)*));
}

fn print(name: &str, color: u8) {
    let vga_buffer = 0xb8000 as *mut u8;

    let name_b = name.as_bytes();
    for (i, &byte) in name_b.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = if color>0 { color } else { i as u8 };
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // println!();
    println!("ggggggggggggggggggggggggggg",0xa);

    loop {}
}