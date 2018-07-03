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
    let screen = Screen{xsize:320, ysize:200};

    boxfill(screen.xsize, Color::DarkLightBlue, 0, 0, screen.xsize - 1, screen.ysize - 29);
    boxfill(screen.xsize, Color::LightGray, 0, screen.ysize - 28, screen.xsize - 1, screen.ysize - 28);
    boxfill(screen.xsize, Color::White, 0, screen.ysize - 27, screen.xsize - 1, screen.ysize - 27);
    boxfill(screen.xsize, Color::LightGray, 0, screen.ysize - 26, screen.xsize - 1, screen.ysize - 1);

    boxfill(screen.xsize, Color::White, 3, screen.ysize - 24, 59, screen.ysize - 24);
    boxfill(screen.xsize, Color::White, 2, screen.ysize - 24, 2, screen.ysize - 4);
    boxfill(screen.xsize, Color::DarkGray, 3, screen.ysize - 4, 59, screen.ysize - 4);
    boxfill(screen.xsize, Color::DarkGray, 59, screen.ysize - 23, 59, screen.ysize - 5);
    boxfill(screen.xsize, Color::Black, 2, screen.ysize - 3, 59, screen.ysize - 3);
    boxfill(screen.xsize, Color::Black, 60, screen.ysize - 24, 60, screen.ysize - 3);

    boxfill(screen.xsize, Color::DarkGray, screen.xsize - 47, screen.ysize - 24, screen.xsize - 4, screen.ysize - 24);
    boxfill(screen.xsize, Color::DarkGray, screen.xsize - 47, screen.ysize - 23, screen.xsize - 47, screen.ysize - 4); boxfill(screen.xsize, Color::White, screen.xsize - 47, screen.ysize - 3, screen.xsize - 4, screen.ysize - 3);
    boxfill(screen.xsize, Color::White, screen.xsize - 3, screen.ysize - 24, screen.xsize - 3, screen.ysize - 3);

    unsafe {
        _io_hlt();
    }
}

fn boxfill(xsize: u16, c: Color, x0: u16, y0: u16, x1: u16, y1: u16) {
    let vram_start = 0xa0000;
    for y in y0..y1+1 {
        for x in x0..x1+1 {
            let offset = (y * xsize + x) as u32;
            let vram = (vram_start + offset) as *mut u8;
            unsafe {
                *vram = c as u8;
            }
        }
    }
}

struct Screen {
    xsize: u16,
    ysize: u16,
}

#[derive(Clone, Copy)]
enum Color {
    Black = 0,
    LightRed = 1,
    LightGreen = 2,
    LightYellow = 3,
    LightBlue = 4,
    LightPurple = 5,
    LightLightBlue =6,
    White = 7,
    LightGray = 8,
    DarkRed = 9,
    DarkGreen = 10,
    DarkYellow = 11,
    DarkBlue = 12,
    DarkPurple = 13,
    DarkLightBlue = 14,
    DarkGray = 15
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
        for i in 0..rgb.len() {
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
