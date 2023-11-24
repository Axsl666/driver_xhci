use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};


register_bitfields!{
    u32,
    USBCMD [
        R_S OFFSET(0) NUMBITS(1)[],
        HCRST OFFSET(0) NUMBITS(1)[],
    ],
    USBSTS [
        TODO  OFFSET(0) NUMBITS(1)[],
    ],
    PAGESIZE [
        TODO  OFFSET(0) NUMBITS(1)[],

    ],
    DNCTRL [
        TODO OFFSET(0) NUMBITS(1)[],
    ],
    CONFIG [
        TODO OFFSET(0) NUMBITS(1)[],
    ]
}
register_bitfields!{
    u64,
    CRCR [
        TODO OFFSET(0) NUMBITS(1)[],
    ],
    DCBAAP [
        TODO OFFSET(0) NUMBITS(1)[],
    ],

}

register_structs!{
    pub OperationalRegs {
        (0x00 => pub usb_cmd: ReadWrite<u32,USBCMD::Register>),
        (0x04 => pub usb_sts: ReadWrite<u32,USBSTS::Register>),
        (0x08 => pub page_size: ReadWrite<u32,PAGESIZE::Register>),
        (0x0C => _rsvd),
        (0x14 => pub dn_ctrl: ReadWrite<u32,DNCTRL::Register>),
        (0x18 => pub crcr: ReadWrite<u64,CRCR::Register>),
        (0x20 => _rsvd2),
        (0x30 => pub dcbaap_low: ReadWrite<u64,DCBAAP::Register>),
        (0x38 => pub config: ReadWrite<u32,CONFIG::Register>),
        (0x3C => _rsvd3),
        (0x1400 => @END),
    }
}

