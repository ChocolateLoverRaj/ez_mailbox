use volatile::{VolatileFieldAccess, access::ReadWrite};

#[repr(C)]
#[derive(VolatileFieldAccess)]
pub struct Interrupts {
    #[access(ReadWrite)]
    irq_basic_pending: u32,
    #[access(ReadWrite)]
    irq_1_pending: u32,
    #[access(ReadWrite)]
    irq_2_pending: u32,
    #[access(ReadWrite)]
    fiq_control: u32,
    #[access(ReadWrite)]
    enable_irqs_1: u32,
    #[access(ReadWrite)]
    enable_irqs_2: u32,
    #[access(ReadWrite)]
    enable_basic_irqs: u32,
    #[access(ReadWrite)]
    disable_irqs_1: u32,
    #[access(ReadWrite)]
    disable_irqs_2: u32,
    #[access(ReadWrite)]
    disable_basic_irqs: u32,
}
