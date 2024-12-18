/// Saved registers when a trap (interrupt or exception) occurs.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TrapFrame {
    /// All general registers.
    pub regs: [usize; 32],
    /// Supervisor Exception Program Counter.
    pub sepc: usize,
    /// Supervisor Status Register.
    pub sstatus: usize,
    /// Supervisor Cause Register
    pub scause: usize,
    /// Supervisor Trap Value
    pub stval: usize,
    pub kernel_sp: usize,
}
