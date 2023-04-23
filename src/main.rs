#![no_std]
#![no_main] // error: requires `start` lang_item

// error: `#[panic_handler]` function required, but not found
// error: language item required, but not found: `eh_personality`
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//  disable name mangling 
#[no_mangle] 

// main doesn’t make sense without an underlying runtime that calls it
// this function is the entry point, since the linker looks for a function
// named `_start` by default
pub extern "C" fn _start() -> ! {   
    loop {}
}