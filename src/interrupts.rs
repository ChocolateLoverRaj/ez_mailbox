use arbitrary_int::u6;
use volatile::{
    VolatileFieldAccess, VolatilePtr, VolatileRef,
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
    pub fn enable_irq(&self, irq_number: u6) {
        let ptr = {
            let ptr = self.0.as_ptr().as_raw_ptr();
            unsafe { VolatilePtr::new(ptr) }
        };
        if irq_number.value() < 32 {
            ptr.enable_irqs_1().write(1 << irq_number.value());
        } else {
            ptr.enable_irqs_2()
                .write(1 << (irq_number - u6::new(32)).value());
        }
    }

    pub fn disable_irq(&self, irq_number: u6) {
        let ptr = {
            let ptr = self.0.as_ptr().as_raw_ptr();
            unsafe { VolatilePtr::new(ptr) }
        };
        if irq_number.value() < 32 {
            ptr.disable_irqs_1().write(1 << irq_number.value());
        } else {
            ptr.disable_irqs_2()
                .write(1 << (irq_number - u6::new(32)).value());
        }
    }

    pub fn pending_interrupts_irq_0_32(&self) -> u32 {
        self.0.as_ptr().irq_1_pending().read()
    }
}
