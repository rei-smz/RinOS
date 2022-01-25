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
            out("eax") ret);
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
