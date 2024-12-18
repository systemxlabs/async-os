#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(alloc_error_handler)]

extern crate alloc;

mod allocator;
mod config;
mod dtb;
mod error;
mod hart;
mod lang_items;
mod logging;
mod mem;
mod trap;

pub use error::*;

use core::sync::atomic::{AtomicBool, Ordering};

use config::{KERNEL_STACK_SIZE, MAX_HARTS};
use dtb::MACHINE_META;
use log::info;

#[unsafe(link_section = ".bss.stack")]
static BOOT_STACK: [u8; KERNEL_STACK_SIZE * MAX_HARTS] = [0u8; KERNEL_STACK_SIZE * MAX_HARTS];

#[unsafe(link_section = ".text.entry")]
#[unsafe(no_mangle)]
#[naked]
pub unsafe extern "C" fn _start(hart_id: usize, dtb_addr: usize) -> ! {
    // PC = 0x8020_0000
    // a0 = har_tid
    // a1 = dtb
    unsafe {
        core::arch::naked_asm!(
            "
                addi    t0, a0, 1
                slli    t0, t0, 16              // t0 = (hart_id + 1) * 64KB
                la      sp, {boot_stack}
                add     sp, sp, t0              // set boot stack
                call rust_main
            ",
            boot_stack = sym BOOT_STACK,
        )
    }
}

static FIRST_HART: AtomicBool = AtomicBool::new(true);

#[unsafe(no_mangle)]
fn rust_main(hart_id: usize, dtb: usize) {
    if FIRST_HART
        .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        clear_bss();
        logging::init();

        dtb::parse(dtb);
        hart::init(hart_id);

        allocator::init();

        mem::init();
        trap::init();

        info!("Main hart {} started!", hart_id);

        start_other_harts(hart_id);
    } else {
        hart::init(hart_id);
        mem::swich_kernel_space();
        trap::init();
        info!("Other hart {} started!", hart_id);
    }
}

/// clear BSS segment
pub fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

pub fn start_other_harts(main_hart_id: usize) {
    let harts = MACHINE_META.get().expect("dtb parsed").harts.len();
    for i in 0..harts {
        if i == main_hart_id {
            continue;
        }
        let status = sbi_rt::hart_start(i, _start as usize, 0);
        info!("Start to wake up hart {}... status {:?}", i, status);
    }
}
