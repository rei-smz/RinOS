#[allow(dead_code)]

use crate::asm;

const TABLE_RGB: [[u8;3]; 16] = [
    [0x00, 0x00, 0x00], /*  0:黑 */
    [0xff, 0x00, 0x00],	/*  1:亮红 */
    [0x00, 0xff, 0x00],	/*  2:亮绿 */
    [0xff, 0xff, 0x00],	/*  3:亮黄 */
    [0x00, 0x00, 0xff],	/*  4:亮蓝 */
    [0xff, 0x00, 0xff],	/*  5:亮紫 */
    [0x00, 0xff, 0xff],	/*  6:浅亮蓝 */
    [0xff, 0xff, 0xff],	/*  7:白 */
    [0xc6, 0xc6, 0xc6],	/*  8:亮灰 */
    [0x84, 0x00, 0x00],	/*  9:暗红 */
    [0x00, 0x84, 0x00],	/* 10:暗绿 */
    [0x84, 0x84, 0x00],	/* 11:暗黄 */
    [0x00, 0x00, 0x84],	/* 12:暗青 */
    [0x84, 0x00, 0x84],	/* 13:暗紫 */
    [0x00, 0x84, 0x84],	/* 14:浅暗蓝 */
    [0x84, 0x84, 0x84]	/* 15:暗灰 */
];

#[derive(Clone, Copy)]
pub enum Color {
    Black = 0,
    LightRed = 1,
    LightGreen = 2,
    LightYellow = 3,
    LightBlue = 4,
    LightPurple = 5,
    LightCyan = 6,
    White = 7,
    LightGray = 8,
    DarkRed = 9,
    DarkGreen = 10,
    DarkYellow = 11,
    DarkBlue = 12,
    DarkPurple = 13,
    DarkCyan = 14,
    DarkGray = 15,
}

pub fn set_palette() {
    let eflags = asm::io_load_eflags();
    asm::io_cli();
    asm::io_out8(0x03c8, 0);
    for i in 0..16 {
        asm::io_out8(0x03c9, TABLE_RGB[i][0] / 4);
        asm::io_out8(0x03c9, TABLE_RGB[i][1] / 4);
        asm::io_out8(0x03c9, TABLE_RGB[i][2] / 4);
    }
    asm::io_store_eflags(eflags);
}

pub fn boxfill8(vram: *mut u8, xsize: isize, c: Color, x0: isize, y0: isize, x1: isize, y1: isize) {
    for y in y0..=y1 {
        for x in x0..=x1 {
            let vram = unsafe { &mut *(vram.offset(y * xsize + x)) };
            *vram = c as u8;
        }
    }
}
