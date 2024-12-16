#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(alloc_error_handler)]

mod config;
mod lang_items;
mod logging;
mod trap;

use config::BOOT_STACK_SIZE;
use log::info;

#[unsafe(link_section = ".bss.stack")]
static BOOT_STACK: [u8; BOOT_STACK_SIZE] = [0u8; BOOT_STACK_SIZE];

#[unsafe(link_section = ".text.entry")]
#[unsafe(no_mangle)]
#[naked]
pub unsafe extern "C" fn _start() -> ! {
    // PC = 0x8020_0000
    // a0 = hartid
    // a1 = dtb
    unsafe {
        core::arch::naked_asm!(
            "la sp, {boot_stack}",  // load addr of the symbol `BOOT_STACK` to sp
            "li t0, {boot_stack_size}",  // load immediate `BOOT_STACK_SIZE` to t0
            "add sp, sp, t0",  // setup boot stack
            "call rust_main",
            boot_stack = sym BOOT_STACK,
            boot_stack_size = const BOOT_STACK_SIZE,
        )
    }
}

#[unsafe(no_mangle)]
fn rust_main(hart_id: usize, dtb: usize) {
    clear_bss();
    logging::init();
    info!("Hello, world!");
}

/// clear BSS segment
pub fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}