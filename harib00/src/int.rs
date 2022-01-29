use crate::asm::{io_in8, io_out8};
use crate::{asm, Color, vga};
use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
use crate::fifo::Fifo;

const PIC0_ICW1:u32 = 0x0020;
const PIC0_OCW2:u32 = 0x0020;
const PIC0_IMR:u32 = 0x0021;
const PIC0_ICW2:u32 = 0x0021;
const PIC0_ICW3:u32 = 0x0021;
const PIC0_ICW4:u32 = 0x0021;
const PIC1_ICW1:u32 = 0x00a0;
const PIC1_OCW2:u32 = 0x00a0;
const PIC1_IMR:u32 = 0x00a1;
const PIC1_ICW2:u32 = 0x00a1;
const PIC1_ICW3:u32 = 0x00a1;
const PIC1_ICW4:u32 = 0x00a1;

const PORT_KEYDAT: u32 = 0x0060;
const PORT_KEYSTA: u32 = 0x0064;
const PORT_KEYCMD: u32 = 0x0064;
const KEYSTA_SEND_NOTREADY: u8 = 0x02;
const KEYCMD_WRITE_MODE: u8 = 0x60;
const KBC_MODE: u8 = 0x47;
const KEYCMD_SENDTO_MOUSE: u8 = 0xd4;
const MOUSECMD_ENABLE: u8 = 0xf4;

lazy_static! {
    pub static ref KEYBUF: Mutex<Fifo> = Mutex::new(Fifo::new(32));
    pub static ref MOUSEBUF: Mutex<Fifo> = Mutex::new(Fifo::new(128));
}

pub fn init_pic() {
    //禁止所有中断
    io_out8(PIC0_IMR, 0xff);
    io_out8(PIC1_IMR, 0xff);

    io_out8(PIC0_ICW1, 0x11);
    io_out8(PIC0_ICW2, 0x20);
    io_out8(PIC0_ICW3, 1 << 2);
    io_out8(PIC0_ICW4, 0x01);

    io_out8(PIC1_ICW1, 0x11);
    io_out8(PIC1_ICW2, 0x28);
    io_out8(PIC1_ICW3, 2);
    io_out8(PIC1_ICW4, 0x01);

    io_out8(PIC0_IMR, 0xfb);
    io_out8(PIC1_IMR, 0xff);
}

fn wait_kbc_sendready() {
    loop {
        if(io_in8(PORT_KEYSTA) & KEYSTA_SEND_NOTREADY) == 0 {
            break;
        }
    }
}

fn init_keyboad() {
    wait_kbc_sendready();
    io_out8(PORT_KEYCMD, KEYCMD_WRITE_MODE);
    wait_kbc_sendready();
    io_out8(PORT_KEYDAT, KBC_MODE);
}

pub fn enable_mouse() {
    wait_kbc_sendready();
    io_out8(PORT_KEYCMD, KEYCMD_SENDTO_MOUSE);
    wait_kbc_sendready();
    io_out8(PORT_KEYDAT, MOUSECMD_ENABLE);
}

//处理来自键盘的中断
#[no_mangle]
pub extern "x86-interrupt" fn inthandler21() {
    // let mut screen = vga::Screen::new();
    // screen.boxfill8(Color::Black, 0, 0, 32 * 8 - 1, 15);
    // let mut writer = vga::LineWriter::new(vga::Screen::new(), Color::White, 0, 0);
    // write!(writer, "INT 21 (IRQ-1) : PS/2 keyboard").unwrap();
    // loop {
    //     asm::io_hlt();
    // }
    io_out8(PIC0_OCW2, 0x61);
    //以下这部分会导致CPU接收到无效的OPCODE，原因未知
    let data = io_in8(PORT_KEYDAT);
    KEYBUF.lock().put(data).unwrap();
}

//处理来自鼠标的中断
#[no_mangle]
pub extern "x86-interrupt" fn inthandler2c() {
    io_out8(PIC1_OCW2, 0x64);
    io_out8(PIC0_OCW2, 0x62);
    let data = io_in8(PORT_KEYDAT);
    MOUSEBUF.lock().put(data).unwrap();
}

#[no_mangle]
pub extern "x86-interrupt" fn inthandler27() {
    io_out8(PIC0_OCW2, 0x67);
}

pub fn allow_input() {
    io_out8(PIC0_IMR, 0xf9);
    io_out8(PIC1_IMR, 0xef);
    init_keyboad();
}
