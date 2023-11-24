//! The Capability Registers specify read-only limits, restrictions and capabilities
//! of the host controller implementation. These values are used as parameters to
//! the host controller driver.
use tock_registers::{
    register_bitfields, register_structs,
    registers::ReadOnly,
};

register_bitfields![
    u8,
    // Capability Register Length
    CAPLENGTH [
        LEN OFFSET(0) NUMBITS(8) [],
    ],
];

register_bitfields![
    u16,
    // Interface Version Number
    HCIVERSION [
        VER OFFSET(0) NUMBITS(16) [],
    ],
];

register_bitfields![
    u32,
    // Structural Parameters 1
    HCSPARAMS1 [
        MAX_SLOTS OFFSET(0) NUMBITS(8) [],
        MAX_INTRS OFFSET(8) NUMBITS(11) [],
        MAX_PORTS OFFSET(24) NUMBITS(8) [],
    ],
    // Structural Parameters 2
    HCSPARAMS2 [
        IST OFFSET(0) NUMBITS(4) [],
        ERST_MAX OFFSET(4) NUMBITS(8) [],
        MAX_SCRATCHPAD_BUFS_HI OFFSET(21) NUMBITS(5) [],
        SPR OFFSET(26) NUMBITS(1) [],
        MAX_SCRATCHPAD_BUFS_LO OFFSET(27) NUMBITS(5) [],
    ],
    // Structural Parameters 3
    HCSPARAMS3 [
        U1_DEVICE_EXIT_LATENCY OFFSET(0) NUMBITS(8) [],
        U2_DEVICE_EXIT_LATENCY OFFSET(16) NUMBITS(16) [],
    ],
    // Capability Parameters 1
    HCCPARAMS1 [
        AC64 OFFSET(0) NUMBITS(1) [],
        BNC OFFSET(1) NUMBITS(1) [],
        CSZ OFFSET(2) NUMBITS(1) [],
        PPC OFFSET(3) NUMBITS(1) [],
        PIND OFFSET(4) NUMBITS(1) [],
        LHRC OFFSET(5) NUMBITS(1) [],
        LTC OFFSET(6) NUMBITS(1) [],
        NSS OFFSET(7) NUMBITS(1) [],
        PAE OFFSET(8) NUMBITS(1) [],
        SPC OFFSET(9) NUMBITS(1) [],
        SEC OFFSET(10) NUMBITS(1) [],
        CFC OFFSET(11) NUMBITS(1) [],
        MAX_PSA_SIZE OFFSET(12) NUMBITS(4) [],
        X_ECP OFFSET(16) NUMBITS(16) [],
    ],
    // Doorbell Offset
    DBOFF [
        DBOFF OFFSET(2) NUMBITS(30) [],
    ],
    // Runtime Register Space Offset
    RTSOFF [
        RTSOFF OFFSET(5) NUMBITS(27) [],
    ],
    // Capability Parameters 2
    HCCPARAMS2 [
        U3C OFFSET(0) NUMBITS(1) [],
        CMC OFFSET(1) NUMBITS(1) [],
        FSC OFFSET(2) NUMBITS(1) [],
        CTC OFFSET(3) NUMBITS(1) [],
        LEC OFFSET(4) NUMBITS(1) [],
        CIC OFFSET(5) NUMBITS(1) [],
        ETC OFFSET(6) NUMBITS(1) [],
        ETC_TSC OFFSET(7) NUMBITS(1) [],
        GSC OFFSET(8) NUMBITS(1) [],
        VTC OFFSET(9) NUMBITS(1) [],
    ],
    // TODO: VTIOSOFF
];


register_structs!{
    pub CapRegs {
        (0x00 => pub len: ReadOnly<u8, CAPLENGTH::Register>),
        (0x01 => _rsvd),
        (0x02 => pub hci_ver: ReadOnly<u16, HCIVERSION::Register>),
        (0x04 => pub hcs_params1: ReadOnly<u32, HCSPARAMS1::Register>),
        (0x08 => pub hcs_params2: ReadOnly<u32, HCSPARAMS2::Register>),
        (0x0C => pub hcs_params3: ReadOnly<u32, HCSPARAMS3::Register>),
        (0x10 => pub hcc_params1: ReadOnly<u32, HCCPARAMS1::Register>),
        (0x14 => pub db_offset: ReadOnly<u32,DBOFF::Register>),
        (0x18 => pub rts_offset: ReadOnly<u32,RTSOFF::Register>),
        (0x1C => pub hcc_params2: ReadOnly<u32, HCCPARAMS2::Register>),
        (0x20 => @END),
    }
}