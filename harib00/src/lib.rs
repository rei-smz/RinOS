#![no_std]
#![feature(start)]

use core::arch::asm;
use core::panic::PanicInfo;
use crate::vga::Color;

mod asm;
mod vga;

#[no_mangle]
fn draw_background() {
    let p = unsafe { &mut *(0xa0000 as *mut u8) };
    let xsize = 320;
    let ysize = 200;
    use vga::boxfill8;
    use vga::Color;
    //绘制桌面背景和任务栏
    boxfill8(p, xsize, Color::DarkCyan, 0, 0, xsize - 1, ysize - 29);
    boxfill8(p, xsize, Color::LightGray, 0, ysize - 28, xsize - 1, ysize - 28);
    boxfill8(p, xsize, Color::White, 0, ysize - 27, xsize - 1, ysize - 27);
    boxfill8(p, xsize, Color::LightGray, 0, ysize - 26, xsize - 1, ysize - 1);
    //绘制开始按钮
    boxfill8(p, xsize, Color::White, 3, ysize - 24, 59, ysize - 24);
    boxfill8(p, xsize, Color::White, 2, ysize - 24, 2, ysize - 4);
    boxfill8(p, xsize, Color::DarkGray, 3, ysize - 4, 59, ysize - 4);
    boxfill8(p, xsize, Color::DarkGray, 59, ysize - 23, 59, ysize - 5);
    boxfill8(p, xsize, Color::Black, 2, ysize - 3, 59, ysize - 3);
    boxfill8(p, xsize, Color::Black, 60, ysize - 24, 60, ysize - 3);
    //绘制时间显示区
    boxfill8(p, xsize, Color::DarkGray, xsize - 47, ysize - 24, xsize - 4, ysize - 24);
    boxfill8(p, xsize, Color::DarkGray, xsize - 47, ysize - 23, xsize - 47, ysize - 4);
    boxfill8(p, xsize, Color::White, xsize - 47, ysize - 3, xsize - 4, ysize - 3);
    boxfill8(p, xsize, Color::White, xsize - 3, ysize - 24, xsize - 3, ysize - 3);
}

#[no_mangle]
#[start]
pub extern "C" fn hari_main() -> ! {
    vga::set_palette();
    // let p = unsafe { &mut *(0xa0000 as *mut u8) };
    // vga::boxfill8(p, 320, vga::Color::LightRed, 20, 20, 120, 120);
    // vga::boxfill8(p, 320, vga::Color::LightGreen, 70, 50, 170, 150);
    // vga::boxfill8(p, 320, vga::Color::LightBlue, 120, 80, 220, 180);
    draw_background();
    // for i in 0xa0000..0xaffff {
    //     write_mem8(i, (i & 0x0f) as u8);
    // }
    loop {
        asm::io_hlt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        asm::io_hlt();
    }
}
