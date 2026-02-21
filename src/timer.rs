use arbitrary_int::u2;
use bitbybit::bitfield;
use volatile::{
    VolatileFieldAccess, VolatilePtr, VolatileRef,
    access::{ReadOnly, ReadWrite},
};

#[bitfield(u32, debug, forbid_overlaps)]
struct ControlStatus {
    #[bit(0, rw)]
    m: [bool; 4],
}

/// From https://forums.raspberrypi.com/viewtopic.php?t=9882, it says that timers 2 and 4 are
/// reserved for the GPU to use. We can use timers 1 and 3.
#[repr(C)]
#[derive(Debug, VolatileFieldAccess)]
pub struct Timer {
    #[access(ReadWrite)]
    control_status: ControlStatus,
    #[access(ReadOnly)]
    counter_low: u32,
    #[access(ReadOnly)]
    counter_high: u32,
    #[access(ReadWrite)]
    compare: [u32; 4],
}

impl Timer {
    /// Offset relative to the base address
    pub const ADDRESS: usize = 0x3000;
}

pub struct TimerRef<'a>(pub VolatileRef<'a, Timer, ReadWrite>);

impl TimerRef<'_> {
    pub fn clear_interrupt(&self, timer_number: u2) {
        let ptr = {
            let ptr = self.0.as_ptr().control_status().as_raw_ptr();
            // Safety, we are only writing a u32 which is atomic
            unsafe { VolatilePtr::new(ptr) }
        };
        ptr.write(
            ControlStatus::builder()
                .with_m({
                    let mut m = [false; _];
                    m[timer_number.value() as usize] = true;
                    m
                })
                .value,
        );
    }

    pub fn counter(&self) -> u64 {
        let mut counter = 0;
        counter |= self.0.as_ptr().counter_low().read() as u64;
        counter |= (self.0.as_ptr().counter_high().read() as u64) << 32;
        counter
    }

    pub fn counter_lo(&self) -> u32 {
        self.0.as_ptr().counter_low().read()
    }

    pub fn write_compare_value(&self, timer_number: u2, compare_value: u32) {
        let ptr = {
            let ptr = self
                .0
                .as_ptr()
                .compare()
                .as_slice()
                .index(timer_number.value() as usize)
                .as_raw_ptr();
            // Safety: writing a u32 is atomic
            unsafe { VolatilePtr::new(ptr) }
        };
        ptr.write(compare_value);
    }
}
