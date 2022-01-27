#![no_std]
#![feature(start)]

use core::panic::PanicInfo;
use crate::vga::Color;

mod asm;
mod vga;
mod font;
mod dsctbl;
mod int;

#[no_mangle]
#[start]
pub extern "C" fn hari_main() -> ! {
    dsctbl::init_gdtidt();
    int::init_pic();
    asm::io_sti();
    int::allow_input();

    let mut screen = vga::Screen::new();
    screen.init();
    let mut writer = vga::LineWriter::new(screen, Color::White, 8, 16);
    use core::fmt::Write;
    write!(writer, "Welcome to").unwrap(); //字符串会吞掉换行后面的字符
    writer.x = 33;
    writer.y = 33;
    writer.color = Color::Black;
    write!(writer, "Rin OS.").unwrap();
    writer.x = 32;
    writer.y = 32;
    writer.color = Color::White;
    write!(writer, "Rin OS.").unwrap();

    loop {
        asm::io_hlt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut screen = vga::Screen::new();
    screen.init();
    let mut writer = vga::LineWriter::new(screen, Color::LightRed, 0, 0);
    use core::fmt::Write;
    write!(writer, "[ERR] {:?}", _info).unwrap();
    loop {
        asm::io_hlt();
    }
}
