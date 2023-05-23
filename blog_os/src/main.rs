#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

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

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let b1 = *s1.add(i);
        let b2 = *s2.add(i);
        if b1 != b2 {
            return b1 as i32 - b2 as i32;
        }
        i += 1;
    }
    0
}


#[macro_export]
macro_rules! println {
    () => (_print(b"Default"));
    ($arg:expr) => (_print($arg,0x0));
    ($($arg:tt)+) => (_print($($arg)+));
    // ($($arg:tt)*) => (print($($arg)*));
}

static mut count: isize = 0;

fn _print(name: &str, color: u8) {
    let vga_buffer = 0xb8000 as *mut u8;
    let name_b = name.as_bytes();
    let temp = name_b.len() as isize *2;
    // temp = 20 ;
    // let name_b2 = name2.as_bytes();

    // let mut buffer: [u8; 100] = [0; 100];
    // buffer[0..name_b.len()].copy_from_slice(name_b);
    // buffer[name_b.len()..name_b.len() + name_b2.len()].copy_from_slice(name_b2);   

    for (i, &byte) in name_b.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2 + count ) = byte;
            *vga_buffer.offset(i as isize * 2 + 1 + count) = if color>0 { color } else { i as u8 };
        }
    }
    unsafe { count = count + temp; }
}

fn _println(){
    unsafe { count = count + 250;  }
}


use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT");
}

pub fn init_idt() {
    IDT.load();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init_idt();


    _print("12345", 0xa);
    _print("aaa", 0xa);
    _print("bbb ", 0xa);
    _println();
    _print("ccc", 0xa);
    _print("dd ", 0xa);
    _print("ee", 0xa);
    
    x86_64::instructions::interrupts::int3();
    _print("It did not crash", 0xa);


    loop {}
}