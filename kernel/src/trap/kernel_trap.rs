use log::info;
use riscv::register::{scause, sepc, sstatus, stval, stvec};

pub fn set_kernel_trap() {
    unsafe {
        stvec::write(__trap_from_kernel as usize, stvec::TrapMode::Direct);
    }
}

#[unsafe(link_section = ".text.trampoline")]
#[unsafe(no_mangle)]
#[naked]
pub unsafe extern "C" fn __trap_from_kernel() {
    unsafe {
        core::arch::naked_asm!(
            "
                # only need to save caller-saved regs
                # note that we don't save sepc & stvec here
                addi sp, sp, -17*8
                sd  ra,  1*8(sp)
                sd  t0,  2*8(sp)
                sd  t1,  3*8(sp)
                sd  t2,  4*8(sp)
                sd  t3,  5*8(sp)
                sd  t4,  6*8(sp)
                sd  t5,  7*8(sp)
                sd  t6,  8*8(sp)
                sd  a0,  9*8(sp)
                sd  a1, 10*8(sp)
                sd  a2, 11*8(sp)
                sd  a3, 12*8(sp)
                sd  a4, 13*8(sp)
                sd  a5, 14*8(sp)
                sd  a6, 15*8(sp)
                sd  a7, 16*8(sp)
                call kernel_trap_handler
                ld  ra,  1*8(sp)
                ld  t0,  2*8(sp)
                ld  t1,  3*8(sp)
                ld  t2,  4*8(sp)
                ld  t3,  5*8(sp)
                ld  t4,  6*8(sp)
                ld  t5,  7*8(sp)
                ld  t6,  8*8(sp)
                ld  a0,  9*8(sp)
                ld  a1, 10*8(sp)
                ld  a2, 11*8(sp)
                ld  a3, 12*8(sp)
                ld  a4, 13*8(sp)
                ld  a5, 14*8(sp)
                ld  a6, 15*8(sp)
                ld  a7, 16*8(sp)
                addi sp, sp, 17*8
                sret
        "
        );
    }
}

#[unsafe(no_mangle)]
pub fn kernel_trap_handler() {
    let scause = scause::read();
    info!("strap_handler cause: {:?}", scause.cause());
    match scause.cause() {
        scause::Trap::Exception(scause::Exception::StorePageFault) => {
            let stval = stval::read();
            let sepc = sepc::read();
            if stval == 0 {
                info!(
                    "This exception should be kernel trap test, sepc: {:#x}",
                    sepc
                );
                sepc::write(sepc::read() + 4);
            }
        }
        _ => {
            panic_on_unknown_trap();
        }
    }
}

fn panic_on_unknown_trap() {
    panic!(
        "[kernel] sstatus sum {}, {:?}(scause:{}) in application, bad addr = {:#x}, bad instruction = {:#x}, kernel panicked!!",
        sstatus::read().sum(),
        scause::read().cause(),
        scause::read().bits(),
        stval::read(),
        sepc::read(),
    );
}

pub fn kernel_trap_test() {
    unsafe {
        core::ptr::null_mut::<u8>().write_volatile(0);
    }
}
