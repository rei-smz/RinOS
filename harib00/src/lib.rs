#![no_std]
#![feature(start)]
#![feature(asm_sym)]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]

use core::panic::PanicInfo;
use crate::int::{KEYBUF, MOUSEBUF};
use crate::vga::{Color, Screen};

mod asm;
mod vga;
mod font;
mod dsctbl;
mod int;
mod fifo;
//mod interrupts;

#[no_mangle]
#[start]
pub extern "C" fn hari_main() -> ! {
    dsctbl::init_gdtidt();
    int::init_pic();
    asm::io_sti();

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

    int::allow_input();
    int::enable_mouse();
    loop {
        asm::io_cli();
        if KEYBUF.lock().status() != 0 {
            let key = KEYBUF.lock().get().unwrap();
            asm::io_sti();
            (vga::Screen::new()).boxfill8(Color::DarkCyan, 0, 0, 16, 16);
            let mut writer = vga::LineWriter::new(Screen::new(), Color::White, 0, 0);
            write!(writer, "{:x}", key).unwrap();
        } else if MOUSEBUF.lock().status() != 0 {
            let i = MOUSEBUF.lock().get().unwrap();
            asm::io_sti();
            (vga::Screen::new()).boxfill8(Color::DarkCyan, 32, 0, 48, 16);
            let mut writer = vga::LineWriter::new(Screen::new(), Color::White, 0, 0);
            write!(writer, "{:x}", i).unwrap();
        } else {
            asm::io_stihlt();
        }
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
