#![no_std]

pub mod interrupts;
pub mod timer;

use log::info;
pub use volatile;
use volatile::{
    VolatileFieldAccess, VolatilePtr,
    access::{ReadOnly, ReadWrite, WriteOnly},
};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, transmute_mut, transmute_ref};

#[repr(C)]
#[derive(VolatileFieldAccess, Clone, Copy, Debug)]
pub struct Mailbox {
    #[access(ReadOnly)]
    read_reg: u32,
    _reserved: [u32; 3],
    poll: [u8; 0x4],
    sender: [u8; 0x4],
    #[access(ReadOnly)]
    status: u32,
    config: [u8; 0x4],
    #[access(WriteOnly)]
    write_reg: u32,
}

const MBOX_RESPONSE: u32 = 0x80000000;
const MBOX_FULL: u32 = 0x80000000;
const MBOX_EMPTY: u32 = 0x40000000;

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, Default, FromBytes, IntoBytes, Immutable, KnownLayout)]
pub struct SharedBufferSegment(pub [u32; 4]);

/// Returns true if success, false if error.
pub fn call(
    mbox: VolatilePtr<'_, Mailbox, ReadWrite>,
    channel: u8,
    buffer: &mut [SharedBufferSegment],
) -> bool {
    let buffer: &mut [u32] = transmute_mut!(buffer);
    loop {
        if mbox.status().read() & MBOX_FULL == 0 {
            break;
        }
    }
    let r = (buffer.as_ptr().addr() as u32 & !0xF) | (channel as u32);
    mbox.write_reg().write(r);
    info!("waiting for not empty");
    loop {
        loop {
            if mbox.status().read() & MBOX_EMPTY == 0 {
                break;
            }
        }
        // info!("not empty");
        if (mbox.read_reg().read() & 0xF) as u8 == channel {
            break buffer[1] == MBOX_RESPONSE;
        }
    }
}

pub fn get_board_revision() -> [SharedBufferSegment; 2] {
    let mut buffer = [Default::default(); _];
    {
        let buffer: &mut [u32] = transmute_mut!(&mut buffer);
        // Fits in one chunk of 16 bytes
        buffer[0] = 7 * size_of::<u32>() as u32;
        // Process request
        buffer[1] = 0;
        // Get board model
        buffer[2] = 0x00010002;
        // Value buffer size
        buffer[3] = 4;
        // Request code: request
        buffer[4] = 0x0;
        // Data space
        buffer[5] = 0;
        // End tag
        buffer[6] = 0x0;
        info!("buffer: {:X?}", buffer);
    }
    buffer
}

pub fn board_revision(buffer: &[SharedBufferSegment; 2]) -> u32 {
    let buffer: &[u32] = transmute_ref!(buffer);
    buffer[5]
}
