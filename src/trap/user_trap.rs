use riscv::register::stvec;

use crate::config::TRAMPOLINE;

use super::set_kernel_trap;

pub fn set_user_trap() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, stvec::TrapMode::Direct);
    }
}

#[unsafe(link_section = ".text.trampoline")]
#[unsafe(no_mangle)]
#[naked]
pub unsafe extern "C" fn __trap_from_user() {
    unsafe {
        core::arch::naked_asm!(
            "
                csrrw sp, sscratch, sp
                sd x1, 1*8(sp)
                
                # skip sp(x2), we will save it later
                
                sd x3, 3*8(sp)
                
                # skip tp(x4), application does not use it
                
                sd x5, 5*8(sp)
                sd x6, 6*8(sp)
                sd x7, 7*8(sp)
                sd x8, 8*8(sp)
                sd x9, 9*8(sp)
                sd x10, 10*8(sp)
                sd x11, 11*8(sp)
                sd x12, 12*8(sp)
                sd x13, 13*8(sp)
                sd x14, 14*8(sp)
                sd x15, 15*8(sp)
                sd x16, 16*8(sp)
                sd x17, 17*8(sp)
                sd x18, 18*8(sp)
                sd x19, 19*8(sp)
                sd x20, 20*8(sp)
                sd x21, 21*8(sp)
                sd x22, 22*8(sp)
                sd x23, 23*8(sp)
                sd x24, 24*8(sp)
                sd x25, 25*8(sp)
                sd x26, 26*8(sp)
                sd x27, 27*8(sp)
                sd x28, 28*8(sp)
                sd x29, 29*8(sp)
                sd x30, 30*8(sp)
                sd x31, 31*8(sp)
                
                # we can use t0/t1/t2 freely, because they have been saved in TrapContext
                csrr t0, sstatus
                csrr t1, sepc
                sd t0, 32*8(sp)
                sd t1, 33*8(sp)

                # read user stack from sscratch and save it in TrapContext
                csrr t2, sscratch
                sd t2, 2*8(sp)

                # load kernel_satp into t0
                ld t0, 34*8(sp)
                
                # load trap_handler into t1
                ld t1, 36*8(sp)
                
                # move to kernel_sp
                ld sp, 35*8(sp)
                
                # switch to kernel space
                csrw satp, t0
                sfence.vma

                # jump to trap_handler
                jr t1
            "
        );
    }
}

#[unsafe(no_mangle)]
pub fn user_trap_handler() {
    set_kernel_trap();

}

#[unsafe(no_mangle)]
pub fn user_trap_return() -> ! {
    set_user_trap();
    todo!()
}