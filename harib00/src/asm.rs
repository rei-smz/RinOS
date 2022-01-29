use core::arch::asm;

pub fn io_hlt() {
    unsafe {
        asm!("HLT");
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

pub fn io_stihlt() {
    unsafe {
        asm!("STI", "HLT");
    }
}

pub fn io_out8(port: u32, data: u8) {
    unsafe {
        asm!(
            // "MOV EDX, [{0}]",
            // "MOV AL, [{1}]",
            "OUT dx, al",
            in("edx") port,
            in("al") data
        );
    }
}

pub fn io_in8(port: u32) -> u8 {
    let mut ret: u8;
    unsafe {
        asm!(
            "MOV eax, 0",
            "IN al, dx",
            in("edx") port, lateout("al") ret
        );
    }
    ret
}

pub fn io_load_eflags() -> u32 {
    let ret;
    unsafe {
        asm!(
            "PUSHFD",
            "POP eax",
            lateout("eax") ret
        );
    }
    ret
}

pub fn io_store_eflags(eflags: u32) {
    unsafe {
        asm!(
            "PUSH eax",
            "POPFD",
            in("eax") eflags
        );
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
        pub extern "x86-interrupt" fn wrapper() {
            use core::arch::asm;
            unsafe {
                asm!(
                    "PUSH es",
                    "PUSH ds",
                    "PUSHAD",
                    "MOV eax, esp",
                    "PUSH eax",
                    "MOV ax, ss",
                    "MOV ds, ax",
                    "MOV es, ax",
                    "CALL {}",
                    "POP eax",
                    "POPAD",
                    "POP ds",
                    "POP es",
                    "IRETD", in(reg) $name as extern "x86-interrupt" fn()
                );
                // asm!("PUSH ES
                //       PUSH DS
                //       PUSHAD
                //       MOV EAX,ESP
                //       PUSH EAX
                //       MOV AX,SS
                //       MOV DS,AX
                //       MOV ES,AX" : : : : "intel", "volatile");
                // asm!("CALL $0" : : "r"($name as extern "C" fn()) : : "intel");
                // asm!("POP EAX
                //     POPAD
                //     POP DS
                //     POP ES
                //     IRETD" : : : : "intel", "volatile");
            }
        }
        wrapper
    }};
}

