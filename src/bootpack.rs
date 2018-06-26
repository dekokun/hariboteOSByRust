#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![feature(asm)]

use core::panic::PanicInfo;

extern "C" {
    #[cfg(any(target_arch = "x86"))]
    pub fn _io_hlt();
}

#[no_mangle]
#[start]
pub fn hari_main() {
    unsafe {
        _io_hlt();
    }
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_impl"]
extern fn panic_impl(_:&PanicInfo) -> ! { loop {} }

