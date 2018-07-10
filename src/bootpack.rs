#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![feature(asm)]

use core::panic::PanicInfo;
mod hankaku;

#[link(name = "hankaku")]
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
    let bootinfo = BootInfo::new();
    init_palette();
    init_screen(&bootinfo);

	init_palette();
	put_fonts(bootinfo.vram, bootinfo.screenx, 10, 10, Color::Black, "DEKOOS");
	put_fonts(bootinfo.vram, bootinfo.screenx, 9, 9, Color::White, "DEKOOS");

    unsafe {
        _io_hlt();
    }
}

fn init_screen(bootinfo: &BootInfo) {
    let screen = Screen::new(bootinfo.screenx, bootinfo.screeny);

    boxfill(bootinfo.vram, screen.xsize, Color::DarkCyan, 0, 0, screen.xsize - 1, screen.ysize - 29);
    boxfill(bootinfo.vram, screen.xsize, Color::LightGray, 0, screen.ysize - 28, screen.xsize - 1, screen.ysize - 28);
    boxfill(bootinfo.vram, screen.xsize, Color::White, 0, screen.ysize - 27, screen.xsize - 1, screen.ysize - 27);
    boxfill(bootinfo.vram, screen.xsize, Color::LightGray, 0, screen.ysize - 26, screen.xsize - 1, screen.ysize - 1);

    boxfill(bootinfo.vram, screen.xsize, Color::White, 3, screen.ysize - 24, 59, screen.ysize - 24);
    boxfill(bootinfo.vram, screen.xsize, Color::White, 2, screen.ysize - 24, 2, screen.ysize - 4);
    boxfill(bootinfo.vram, screen.xsize, Color::DarkGray, 3, screen.ysize - 4, 59, screen.ysize - 4);
    boxfill(bootinfo.vram, screen.xsize, Color::DarkGray, 59, screen.ysize - 23, 59, screen.ysize - 5);
    boxfill(bootinfo.vram, screen.xsize, Color::Black, 2, screen.ysize - 3, 59, screen.ysize - 3);
    boxfill(bootinfo.vram, screen.xsize, Color::Black, 60, screen.ysize - 24, 60, screen.ysize - 3);

    boxfill(bootinfo.vram, screen.xsize, Color::DarkGray, screen.xsize - 47, screen.ysize - 24, screen.xsize - 4, screen.ysize - 24);
    boxfill(bootinfo.vram, screen.xsize, Color::DarkGray, screen.xsize - 47, screen.ysize - 23, screen.xsize - 47, screen.ysize - 4);
    boxfill(bootinfo.vram, screen.xsize, Color::White, screen.xsize - 47, screen.ysize - 3, screen.xsize - 4, screen.ysize - 3);
    boxfill(bootinfo.vram, screen.xsize, Color::White, screen.xsize - 3, screen.ysize - 24, screen.xsize - 3, screen.ysize - 3);

}

fn boxfill(vram: u32, xsize: u16, c: Color, x0: u16, y0: u16, x1: u16, y1: u16) {
    let vram_start = vram;
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
struct BootInfo {
    #[allow(dead_code)]
    cyls: u8,
    #[allow(dead_code)]
    leds: u8,
    #[allow(dead_code)]
    vmode: u8,
    screenx: u16,
    screeny: u16,
    vram: u32
}

impl BootInfo {
    fn new() -> BootInfo {
        // from asmhead.asm
        let cyls_addr = 0x0ff0;
        let leds_addr = 0x0ff1;
        let vmode_addr = 0x0ff2;
        let screenx_addr = 0x0ff4;
        let screeny_addr = 0x0ff6;
        let vram_addr = 0x0ff8;
        return BootInfo{
            cyls: unsafe{ *(cyls_addr as *const u8)},
            leds: unsafe{ *(leds_addr as *const u8)},
            vmode: unsafe{*(vmode_addr as *const u8)},
            screenx: unsafe{*(screenx_addr as * const u16)},
            screeny: unsafe{*(screeny_addr as * const u16)},
            vram: unsafe{*(vram_addr as * const u32)}
        }
    }
}

struct Screen {
    xsize: u16,
    ysize: u16,
}

impl Screen {
    fn new(xsize: u16, ysize: u16) -> Screen {
        return Screen{xsize: xsize, ysize: ysize};
    }
}

#[derive(Clone, Copy)]
enum Color {
    Black = 0,
    #[allow(dead_code)]
    LightRed = 1,
    #[allow(dead_code)]
    LightGreen = 2,
    #[allow(dead_code)]
    LightYellow = 3,
    #[allow(dead_code)]
    LightBlue = 4,
    #[allow(dead_code)]
    LightPurple = 5,
    #[allow(dead_code)]
    LightCyan = 6,
    White = 7,
    LightGray = 8,
    #[allow(dead_code)]
    DarkRed = 9,
    #[allow(dead_code)]
    DarkGreen = 10,
    #[allow(dead_code)]
    DarkYellow = 11,
    #[allow(dead_code)]
    DarkBlue = 12,
    #[allow(dead_code)]
    DarkPurple = 13,
    DarkCyan = 14,
    DarkGray = 15
}
impl Color {
    fn palette() -> [u16; 48] {
    return [
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

    }
}

fn put_fonts(vram: u32, xsize: u16, x: u16, y: u16, color: Color, string: &str) {
    for (i, c) in string.chars().enumerate() {
        put_font(vram, xsize, x + (i * 8) as u16, y, color, hankaku::HANKAKU[c as usize])
    }
}
fn put_font(vram: u32, xsize: u16, x: u16, y: u16, color: Color, font: [u8; 16]) {
    for (i, f) in font.iter().enumerate() {
        let p = vram + ((y + i as u16) * xsize + x) as u32;
        if (f & 0x80) != 0 { unsafe { *((p + 0) as *mut u8) = color as u8; } }
        if (f & 0x40) != 0 { unsafe { *((p + 1) as *mut u8) = color as u8; } }
        if (f & 0x20) != 0 { unsafe { *((p + 2) as *mut u8) = color as u8; } }
        if (f & 0x10) != 0 { unsafe { *((p + 3) as *mut u8) = color as u8; } }
        if (f & 0x08) != 0 { unsafe { *((p + 4) as *mut u8) = color as u8; } }
        if (f & 0x04) != 0 { unsafe { *((p + 5) as *mut u8) = color as u8; } }
        if (f & 0x02) != 0 { unsafe { *((p + 6) as *mut u8) = color as u8; } }
        if (f & 0x01) != 0 { unsafe { *((p + 7) as *mut u8) = color as u8; } }
    }
}

fn init_palette() {
    let rgb = Color::palette();
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
