use crate::{asm, asm_handler};
use crate::int;

const ADR_GDT: u32 = 0x00270000;
const LIMIT_GDT: u32 = 0x0000ffff;
const ADR_IDT: u32 = 0x0026f800;
const LIMIT_IDT: u32 = 0x000007ff;
const ADR_BOTPAK: u32 = 0x00280000;
const LIMIT_BOTPAK: u32 = 0x0007ffff;
const AR_INTGATE32: u32 = 0x008e;
const AR_DATA32_RW: u32 = 0x4092;
const AR_CODE32_ER: u32 = 0x409a;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct SegmentDescriptor {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_mid: u8,
    pub access_right: u8,
    pub limit_high: u8,
    pub base_high: u8
}

impl SegmentDescriptor {
    pub fn new(mut limit: u32, base: u32, mut ar: u32) -> SegmentDescriptor {
        if limit > 0xfffff {
            ar |= 0x8000;
            limit /= 0x1000;
        }
        SegmentDescriptor {
            limit_low: (limit & 0xffff) as u16,
            base_low: (base & 0xffff) as u16,
            base_mid: ((base >> 16) & 0xff) as u8,
            access_right: (ar & 0xff) as u8,
            limit_high: (((limit >> 16) & 0x0f) | (ar >> 8) & 0x0f) as u8,
            base_high: ((base >> 24) & 0xff) as u8
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct GateDescriptor {
    offset_low: u16,
    selector: u16,
    dw_count: u8,
    access_right: u8,
    offset_high: u16
}

impl GateDescriptor {
    pub fn new(offset: u32, selector: u32, ar: u32) -> GateDescriptor {
        GateDescriptor {
            offset_low: (offset & 0xffff) as u16,
            selector: selector as u16,
            dw_count: ((ar >> 8) & 0xff) as u8,
            access_right: (ar & 0xff) as u8,
            offset_high: ((offset >> 16) & 0xffff) as u16
        }
    }
}

pub fn init_gdtidt() {
    for i in 0..=(LIMIT_GDT / 8) {
        let gdt = unsafe { &mut *((ADR_GDT + i * 8) as *mut SegmentDescriptor) };
        *gdt = SegmentDescriptor::new(0, 0, 0);
    }
    let gdt = unsafe { &mut *((ADR_GDT + 1 * 8) as *mut SegmentDescriptor) };
    *gdt = SegmentDescriptor::new(0xffffffff, 0, AR_DATA32_RW);
    let gdt = unsafe { &mut *((ADR_GDT + 2 * 8) as *mut SegmentDescriptor) };
    *gdt = SegmentDescriptor::new(LIMIT_BOTPAK, ADR_BOTPAK, AR_CODE32_ER);
    asm::load_gdtr(LIMIT_GDT, ADR_GDT);

    for i in 0..=(LIMIT_IDT / 8) {
        let idt = unsafe { &mut *((ADR_IDT + i * 8) as *mut GateDescriptor) };
        *idt = GateDescriptor::new(0, 0, 0);
    }
    use crate::int::{inthandler21, inthandler2c, inthandler27};
    let idt = unsafe { &mut *((ADR_IDT + 0x21 * 8) as *mut GateDescriptor) };
    *idt = GateDescriptor::new(asm_handler!(inthandler21) as u32, 2 * 8, AR_INTGATE32);
    let idt = unsafe { &mut *((ADR_IDT + 0x2c * 8) as *mut GateDescriptor) };
    *idt = GateDescriptor::new(asm_handler!(inthandler2c) as u32, 2 * 8, AR_INTGATE32);
    let idt = unsafe { &mut *((ADR_IDT + 0x27 * 8) as *mut GateDescriptor) };
    *idt = GateDescriptor::new(asm_handler!(inthandler27) as u32, 2 * 8, AR_INTGATE32);
    asm::load_idtr(LIMIT_IDT, ADR_IDT);
}
