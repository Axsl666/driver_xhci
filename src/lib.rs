#![no_std]

use core::ptr::NonNull;
mod registers;

mod rings;
mod trb;

use crate::registers::capability::CapRegs;


struct XHCI {
    // immutable
    cap: NonNull<CapRegs>
}