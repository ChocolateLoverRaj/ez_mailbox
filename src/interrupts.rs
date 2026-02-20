use arbitrary_int::u6;
use volatile::{
    VolatileFieldAccess, VolatileRef,
    access::{ReadOnly, ReadWrite},
};

/// IRQ 1 is Timer 1 IRQ
#[repr(C)]
#[derive(VolatileFieldAccess)]
pub struct Interrupts {
    #[access(ReadOnly)]
    pub irq_basic_pending: u32,
    #[access(ReadOnly)]
    pub irq_1_pending: u32,
    #[access(ReadOnly)]
    pub irq_2_pending: u32,
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

impl Interrupts {
    /// Address relative to the base address
    pub const ADDRESS: usize = 0xB200;
}

pub struct InterruptsRef<'a>(pub VolatileRef<'a, Interrupts, ReadWrite>);

impl InterruptsRef<'_> {
    pub fn enable_irq(&mut self, irq_number: u6) {
        if irq_number.value() < 32 {
            self.0.as_mut_ptr().enable_irqs_1().update(|mut n| {
                n |= 1 << irq_number.value();
                n
            });
        } else {
            self.0.as_mut_ptr().enable_irqs_2().update(|mut n| {
                n |= 1 << (irq_number.value() - 32);
                n
            });
        }
    }

    pub fn disable_irq(&mut self, irq_number: u6) {
        if irq_number < u6::new(32) {
            self.0.as_mut_ptr().disable_irqs_1().update(|mut n| {
                n |= 1 << irq_number.value();
                n
            });
        } else {
            self.0.as_mut_ptr().disable_irqs_2().update(|mut n| {
                n |= 1 << (irq_number - u6::new(32)).value();
                n
            });
        }
    }
}
