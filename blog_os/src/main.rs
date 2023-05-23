#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const BUFFER_WIDTH: usize = 80;

#[no_mangle]
pub unsafe extern "C" fn memset(ptr: *mut u8, value: i32, num: usize) -> *mut u8 {
    let mut p = ptr;
    for _ in 0..num {
        *p = value as u8;
        p = p.offset(1);
    }
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(destination: *mut u8, source: *const u8, num: usize) -> *mut u8 {
    let mut dest = destination;
    let mut src = source;
    
    for _ in 0..num {
        *dest = *src;
        dest = dest.offset(1);
        src = src.offset(1);
    }
    
    destination
}

#[macro_export]
macro_rules! println {
    () => (_print(b"Default"));
    ($arg:expr) => (_print($arg,0x0));
    ($($arg:tt)+) => (_print($($arg)+));
    // ($($arg:tt)*) => (print($($arg)*));
}

fn _print(name: &str, color: u8) {
    let vga_buffer = 0xb8000 as *mut u8;

    let name_b = name.as_bytes();
    let name_b2 = "1".as_bytes();
    // name_b = name_b + name_b;

    let mut buffer: [u8; 100] = [0; 100];
    buffer[0..name_b.len()].copy_from_slice(name_b);
    buffer[name_b.len()..2].copy_from_slice(name_b2);
    // buffer[name_b2.len()..].copy_from_slice(name_b2);

    for (i, &byte) in buffer.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = if color>0 { color } else { i as u8 };
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let String global = "Hello " + "GG" ;
    _print("g", 0xa);

    loop {}
}