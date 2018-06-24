#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![feature(asm)]

use core::panic::PanicInfo;

#[no_mangle]
#[start]
pub fn hari_main() {
    loop {}
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_impl"]
extern fn panic_impl(_:&PanicInfo) -> ! { loop {} }
