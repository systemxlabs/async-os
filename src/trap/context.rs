/// Saved registers when a trap (interrupt or exception) occurs.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TrapContext {
    /// All general registers.
    pub user_x: [usize; 32],
    /// Supervisor Status Register.
    pub sstatus: usize,
    /// Supervisor Exception Program Counter.
    pub sepc: usize,
    pub kernel_satp: usize,
    pub kernel_sp: usize,
    pub user_trap_handler: usize,
}
