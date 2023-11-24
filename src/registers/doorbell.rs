use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};


register_bitfields!{
    u32,
    DOORBELL [
        DB_TARGET OFFSET(0) NUMBITS(8) [],
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
    ],
}

register_structs!{
    pub DoorbellReg {
        (0x00 => pub doorbell: ReadWrite<u32, DOORBELL::Register>),
        (0x04 => @END),
    }
}

