#![no_std]
#![feature(start)]

use core::panic::PanicInfo;
use crate::vga::Color;

mod asm;
mod vga;
mod font;

#[no_mangle]
#[start]
pub extern "C" fn hari_main() -> ! {
    let mut screen = vga::Screen::new();
    screen.init();
    let mut writer = vga::LineWriter::new(screen, Color::White, 8, 8);
    use core::fmt::Write;
    write!(writer, "@#ABC 123\n456").unwrap(); //字符串会吞掉换行后面的字符
    writer.x = 31;
    writer.y = 31;
    writer.color = Color::Black;
    write!(writer, "Rin OS.");
    writer.x = 30;
    writer.y = 30;
    writer.color = Color::White;
    write!(writer, "Rin OS.");
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
