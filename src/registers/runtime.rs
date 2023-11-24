use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

register_bitfields!{
    u32,
    MFINDEX [
        MFINDEX OFFSET(0) NUMBITS(14) [],
    ],
}

register_bitfields!{
    u32,
    IMAN [
        PENDING OFFSET(0) NUMBITS(1) [],
        ENABLE OFFSET(1) NUMBITS(1) [],
    ],
    IMOD [
        IMODI OFFSET(0) NUMBITS(16) [],
        IMODC OFFSET(16) NUMBITS(16) [],
    ],
    ERSTSZ [
        ERSTSZ OFFSET(0) NUMBITS(16) [],
    ],
}

register_bitfields!{
    u64,
    ERSTBA [
        ERSTBA OFFSET(6) NUMBITS(58) [],
    ],
    ERDP [
        DESI OFFSET(0) NUMBITS(3) [],
        EHB OFFSET(3) NUMBITS(1) [],
        ERDP OFFSET(4) NUMBITS(60) [],
    ],
}

register_structs!{
    pub InterrupterRegs {
        (0x00 => iman: ReadWrite<u32>),
        (0x04 => imod: ReadWrite<u32>),
        (0x08 => pub erstsz: ReadWrite<u32,ERSTSZ::Register>),
        (0x0C => _rsvd),
        (0x10 => pub erstba: ReadWrite<u64,ERSTBA::Register>),
        (0x18 => pub erdp: ReadWrite<u64,ERDP::Register>),
        (0x20 => @END),
    }
}

register_structs!{
    pub RuntimeRegs{
        (0x00 => pub mfindex: ReadOnly<u32,MFINDEX::Register>),
        (0x04 => _rsvd),
        (0x20 => pub ints: [InterrupterRegs;1024]),
        (0x8020 => @END),
    }
}