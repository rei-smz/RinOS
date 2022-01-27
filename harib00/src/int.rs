use crate::asm::io_out8;
use crate::{asm, Color, vga};
use core::fmt::Write;

const PIC0_ICW1:u16 = 0x0020;
const PIC0_OCW2:u16 = 0x0020;
const PIC0_IMR:u16 = 0x0021;
const PIC0_ICW2:u16 = 0x0021;
const PIC0_ICW3:u16 = 0x0021;
const PIC0_ICW4:u16 = 0x0021;
const PIC1_ICW1:u16 = 0x00a0;
const PIC1_OCW2:u16 = 0x00a0;
const PIC1_IMR:u16 = 0x00a1;
const PIC1_ICW2:u16 = 0x00a1;
const PIC1_ICW3:u16 = 0x00a1;
const PIC1_ICW4:u16 = 0x00a1;

pub fn init_pic() {
    //禁止所有中断
    io_out8(PIC0_IMR as u32, 0xff);
    io_out8(PIC1_IMR as u32, 0xff);

    io_out8(PIC0_ICW1 as u32, 0x11);
    io_out8(PIC0_ICW2 as u32, 0x20);
    io_out8(PIC0_ICW3 as u32, 1 << 2);
    io_out8(PIC0_ICW4 as u32, 0x01);

    io_out8(PIC1_ICW1 as u32, 0x11);
    io_out8(PIC1_ICW2 as u32, 0x28);
    io_out8(PIC1_ICW3 as u32, 2);
    io_out8(PIC1_ICW4 as u32, 0x01);

    io_out8(PIC0_IMR as u32, 0xfb);
    io_out8(PIC1_IMR as u32, 0xff);
}

//处理来自键盘的中断
pub extern "C" fn inthandler21() {
    let mut screen = vga::Screen::new();
    screen.boxfill8(Color::Black, 0, 0, 32 * 8 - 1, 15);
    let mut writer = vga::LineWriter::new(vga::Screen::new(), Color::White, 0, 0);
    write!(writer, "INT 21 (IRQ-1) : PS/2 keyboard").unwrap();
    loop {
        asm::io_hlt();
    }
}

//处理来自鼠标的中断
pub extern "C" fn inthandler2c() {
    let mut screen = vga::Screen::new();
    screen.boxfill8(Color::Black, 0, 0, 32 * 8 - 1, 15);
    let mut writer = vga::LineWriter::new(vga::Screen::new(), Color::White, 0, 0);
    write!(writer, "INT 2C (IRQ-12) : PS/2 mouse").unwrap();
    loop {
        asm::io_hlt();
    }
}

pub extern "C" fn inthandler27() {
    io_out8(PIC0_OCW2 as u32, 0x67);
}

pub fn allow_input() {
    io_out8(PIC0_IMR as u32, 0xf9);
    io_out8(PIC1_IMR as u32, 0xef);
}
