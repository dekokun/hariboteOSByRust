#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![feature(asm)]

use core::panic::PanicInfo;

extern "C" {
    #[cfg(any(target_arch = "x86"))]
    fn _io_cli();
    fn _io_hlt();
    fn _io_out8(i: u32, i: u16);
    fn _io_load_eflags() -> u32;
    fn _io_store_eflags(i: u32);
}

#[no_mangle]
#[start]
pub fn hari_main() {
    init_palette();
    for i in 0xa0000..0xaffff {
        let vram: *mut i32 = i as *mut i32;
        unsafe {
            *vram = i & 0x0f;
        }
    }
    unsafe {
        _io_hlt();
    }
}

fn init_palette() {
    let rgb = [
        0x00,
        0x00,
        0x00, /*  0:黒 */
        0xff,
        0x00,
        0x00, /*  1:明るい赤 */
        0x00,
        0xff,
        0x00, /*  2:明るい緑 */
        0xff,
        0xff,
        0x00, /*  3:明るい黄色 */
        0x00,
        0x00,
        0xff, /*  4:明るい青 */
        0xff,
        0x00,
        0xff, /*  5:明るい紫 */
        0x00,
        0xff,
        0xff, /*  6:明るい水色 */
        0xff,
        0xff,
        0xff, /*  7:白 */
        0xc6,
        0xc6,
        0xc6, /*  8:明るい灰色 */
        0x84,
        0x00,
        0x00, /*  9:暗い赤 */
        0x00,
        0x84,
        0x00, /* 10:暗い緑 */
        0x84,
        0x84,
        0x00, /* 11:暗い黄色 */
        0x00,
        0x00,
        0x84, /* 12:暗い青 */
        0x84,
        0x00,
        0x84, /* 13:暗い紫 */
        0x00,
        0x84,
        0x84, /* 14:暗い水色 */
        0x84,
        0x84,
        0x84, /* 15:暗い灰色 */
    ];
    unsafe {
        let eflags = _io_load_eflags();
        _io_out8(0x03c8, 0);
        for i in 0..rgb.len() - 1 {
            _io_out8(0x03c9, rgb[i] / 4);
        }
        _io_store_eflags(eflags); /* 割り込み許可フラグを元に戻す */
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_impl"]
#[no_mangle]
pub extern "C" fn panic_impl(_: &PanicInfo) -> ! {
    loop {}
}
