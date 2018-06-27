#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![feature(asm)]

use core::panic::PanicInfo;

extern "C" {
    #[cfg(any(target_arch = "x86"))]
    pub fn _io_hlt();
    pub fn _write_mem8(i:i32, j:i32);
}

#[no_mangle]
#[start]
pub fn hari_main() {
    for i in 0xa0000..0xaffff {
        unsafe {
            _write_mem8(i, i & 0x0f);
        }
    }
    unsafe {
        _io_hlt();
    }
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_impl"]
extern fn panic_impl(_:&PanicInfo) -> ! { loop {} }

