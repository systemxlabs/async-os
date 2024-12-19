// TODO
#[unsafe(link_section = ".text.trampoline")]
#[unsafe(no_mangle)]
#[naked]
pub unsafe extern "C" fn __trap_from_user() {
    unsafe {
        core::arch::naked_asm!(
            "
                sret
        "
        );
    }
}

#[unsafe(no_mangle)]
pub fn user_trap_handler() {}