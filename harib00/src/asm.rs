use core::arch::asm;

pub fn io_hlt() {
    unsafe {
        asm!("hlt");
    }
}

pub fn io_cli() {
    unsafe {
        asm!("CLI");
    }
}

pub fn io_sti() {
    unsafe {
        asm!("STI");
    }
}

pub fn io_out8(port: u32, data: u8) {
    unsafe {
        asm!("OUT dx, al",
            in("edx") port,
            in("al") data);
    }
}

pub fn io_load_eflags() -> u32 {
    let ret;
    unsafe {
        asm!("PUSHFD");
        asm!("POP eax",
            lateout("eax") ret);
    }
    ret
}

pub fn io_store_eflags(eflags: u32) {
    unsafe {
        asm!("PUSH eax",
            in("eax") eflags);
        asm!("POPFD");
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct Dt {
    limit: u16,
    base: u32
}
pub fn load_gdtr(limit: u32, addr: u32) {
    unsafe {
        asm!("LGDT [{0}]", in(reg) &Dt { limit: limit as u16, base: addr });
        // llvm_asm!("LGDT ($0)" :: "r"(&Dtr { limit: limit as u16, base: addr } ) : "memory");
    }
}

pub fn load_idtr(limit: u32, addr: u32) {
    unsafe {
        asm!("LIDT [{0}]", in(reg) &Dt { limit: limit as u16, base: addr });
        // llvm_asm!("LIDT ($0)" :: "r"(&Dtr { limit: limit as u16, base: adr }) : "memory");
    }
}

#[macro_export]
macro_rules! asm_handler {
    ($name: ident) => {{
        pub extern "C" fn wrapper() {
            let fun = $name as extern "C" fn();
            use core::arch::asm;
            unsafe {
                asm!("PUSH es");
                asm!("PUSH ds");
                asm!("PUSHAD");
                asm!("MOV eax, esp");
                asm!("PUSH eax");
                asm!("MOV ax, ss");
                asm!("MOV ds, ax");
                asm!("MOV es, ax");
                asm!("CALL {}", in(reg) fun);
                asm!("POP eax");
                asm!("POPAD");
                asm!("POP ds");
                asm!("POP es");
                asm!("IRETD");
            }
        }
        wrapper
    }};
}

