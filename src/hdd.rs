#[cfg(all(feature = "hdd", feature = "allocator", feature = "enum", feature = "time"))]
mod private {
    /****************************************************************/
    //                            Uses                              //
    /****************************************************************/

    use crate::{
        enum_flags,
        println,
        time::{sleep, Time},
    };
    use x86_64::instructions::port::Port;
    use core::convert::{TryFrom, From};
    use alloc::{
        string::String,
        vec::Vec
    };

    /****************************************************************/
    //                            Types                             //
    /****************************************************************/

    enum_flags! {
        pub enum Status: u8 {
            Error             = 0,
            Index             = 1,
            CorrectedData     = 2,
            DataRequestReady  = 3,
            DriveSeekComplete = 4,
            DriveWriteFault   = 5,
            DriveReady        = 6,
            Busy              = 7,
        } of #[repr(u8)]
    }

    enum_flags! {
        pub enum Error: u8 {
            NoAddressMask      = 0,
            Track0NotFound     = 1,
            CommandAborted     = 2,
            MediaChangeRequest = 3,
            IDMarkNotFound     = 4,
            MediaChanged       = 5,
            UncorrectableData  = 6,
            BadBlock           = 7,
        } of #[repr(u8)]
    }

    #[allow(dead_code)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum Command {
        ReadPIO        = 0x20,
        ReadPIOExt     = 0x24,
        ReadDMA        = 0xC8,
        ReadDMAExt     = 0x25,

        WritePIO       = 0x30,
        WritePIOExt    = 0x34,
        WriteDMA       = 0xCA,
        WriteDMAExt    = 0x35,

        CacheFlush     = 0xE7,
        CacheFlushExt  = 0xEA,

        Packet         = 0xA0,
        IdentifyPacket = 0xA1,
        Identify       = 0xEC,

        ATAPIRead      = 0xA8,
        ATAPIEject     = 0x1B
    }

    #[allow(dead_code)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum IDSpace {
        DeviceType   = 0,
        Cylinders    = 2,
        Heads        = 6,
        Sectors      = 12,
        Serial       = 20,
        Model        = 54,
        Capabilities = 98,
        FieldValid   = 106,
        MaxLBA       = 120,
        CommandSets  = 164,
        MaxLBAExt    = 200
    }

    #[allow(dead_code)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u16)]
    pub enum InterfaceType {
        ATA,
        ATAPI
    }

    #[allow(dead_code)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum Owning {
        Master,
        Slave
    }

    impl TryFrom <u8> for Owning {
        type Error = ();

        fn try_from(value: u8) -> Result <Self, Self::Error> {
            match value {
                0 => Ok(Self::Master),
                1 => Ok(Self::Slave),
                _ => Err(())
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum Register {
        Data,
        ErrorAndFeatures, //< Same register for errors and features
        SecCount0,
        LBA0,
        LBA1,
        LBA2,
        HddEvSel,
        CommandAndStatus, //< Same registers for commands and statuses
        SecCount1,
        LBA3,
        LBA4,
        LBA5,
        Control,
        DevAddress
    }

    #[allow(dead_code)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum Channel {
        Primary,
        Secondary,

        Count
    }

    ///
    /// ```
    /// for i in Chanel::Primary {
    ///     // `i` is Primary and Secondary
    /// }
    /// ```
    ///
    impl Iterator for Channel {
        type Item = Self;

        fn next(&mut self) -> Option <Self::Item> {
            match self {
                Self::Primary => {
                    *self = Self::Secondary;
                    Some(Self::Primary)
                },
                Self::Secondary => {
                    *self = Self::Count;
                    Some(Self::Secondary)
                },
                Self::Count => None
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum Direction {
        Read,
        Write
    }

    #[allow(dead_code)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum PollingError {
        DeviceFault,
        Common,
        DRQ
    }

    pub type PollingResult = Result <(), PollingError>;

    #[allow(dead_code)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum LBAMode {
        CHS,
        LBA28,
        LBA48
    }

    // impl fmt::Display for PollingError {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         match self {
    //             PollingError::DeviceFault => f.write_str("Device fault\n")?,
    //             PollingError::Common => {
    //                 let st = read(d)
    //             },
    //             PollingError::DRQ => {}
    //             PollingError::Ok => {}
    //         }
    //         Ok(())
    //     }
    // }

    #[repr(packed)]
    #[derive(Copy, Clone)]
    pub struct IDEChannelRegisters {
        pub base:           u16,
        pub control:        u16,
        pub bus_master_ide: u16,
        pub no_int:         u8
    }

    impl IDEChannelRegisters {
        pub const fn new() -> Self {
            const NEW: IDEChannelRegisters = IDEChannelRegisters {
                base: 0,
                control: 0,
                bus_master_ide: 0,
                no_int: 0
            };
            NEW
        }
    }

    #[repr(packed)]
    #[derive(Copy, Clone)]
    pub struct IDEDevice {
        pub reserved:     bool,
        pub channel:      Channel,
        pub drive:        Owning,
        pub r#type:       InterfaceType,
        pub signature:    u16,
        pub capabilities: u16,
        pub command_sets: u32,
        pub size:         u32,
        pub model:        [u8; 41]
    }

    impl IDEDevice {
        pub const fn new() -> Self {
            const NEW: IDEDevice = IDEDevice {
                reserved: false,
                channel: Channel::Primary,
                drive: Owning::Master,
                r#type: InterfaceType::ATA,
                signature: 0,
                capabilities: 0,
                command_sets: 0,
                size: 0,
                model: [0; 41]
            };
            NEW
        }
    }

    /****************************************************************/
    //                           Statics                            //
    /****************************************************************/

    static mut CHANNELS: [IDEChannelRegisters; 2] = [IDEChannelRegisters::new(); 2];
    static mut BUFFER: [u8; 512] = [0; 512];
    static mut IRQ_INVOKED: bool = false;
    static mut ATAPI_PACKET: [u8; 12] = [0xA8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    static mut DEVICES: [IDEDevice; 4] = [IDEDevice::new(); 4];

    /****************************************************************/
    //                     Other functions                          //
    /****************************************************************/

    unsafe fn align_reg(channel: usize, reg: u16) -> u16 {
        if reg < 0x08 {
            CHANNELS[channel].base + reg
        } else if reg < 0x0C {
            CHANNELS[channel].base + reg - 0x06
        } else if reg < 0x0E {
            CHANNELS[channel].control + reg - 0x0A
        } else if reg < 0x16 {
            CHANNELS[channel].bus_master_ide + reg - 0x0E
        } else {
            0
        }
    }

    pub unsafe fn read(channel: Channel, reg: Register) -> u8 {
        let reg = reg as u16;
        let result: u8;

        if reg > 0x07 && reg < 0x0C {
            write(channel, Register::Control, 0x80 | CHANNELS[channel as usize].no_int)
        }

        result = Port::new(align_reg(channel as usize, reg)).read();

        if reg > 0x07 && reg < 0x0C {
            write(channel, Register::Control, CHANNELS[channel as usize].no_int)
        }

        result
    }

    pub unsafe fn write(channel: Channel, reg: Register, data: u8) {
        let reg = reg as u16;

        if reg > 0x07 && reg < 0x0C {
            write(channel, Register::Control, 0x80 | CHANNELS[channel as usize].no_int)
        }

        Port::new(align_reg(channel as usize, reg)).write(data);

        if reg > 0x07 && reg < 0x0C {
            write(channel, Register::Control, CHANNELS[channel as usize].no_int)
        }
    }

    pub unsafe fn polling(channel: Channel, advanced: bool) -> PollingResult {
        sleep(Time::from(1u64));
        while read(channel, Register::CommandAndStatus) & Status::Busy as u8 != 0 {}
        if advanced {
            let state = read(channel, Register::CommandAndStatus);
            if state & Status::Error as u8 != 0 {
                return Err(PollingError::Common)
            } else if state & Status::DriveWriteFault as u8 != 0 {
                return Err(PollingError::DeviceFault)
            } else if state & Status::DataRequestReady as u8 != 0 {
                return Err(PollingError::DRQ)
            }
        }
        Ok(())
    }

    pub unsafe fn read_buffer(channel: Channel, reg: Register) {
        let reg = reg as u16;

        if reg > 0x07 && reg < 0x0C {
            write(channel, Register::Control, 0x80 | CHANNELS[channel as usize].no_int)
        }

        asm!("insd", in("rdi") &BUFFER as *const u8 as u64, in("dx") align_reg(channel as usize, reg), in("cx") 128);

        if reg > 0x07 && reg < 0x0C {
            write(channel, Register::Control, CHANNELS[channel as usize].no_int)
        }
    }

    pub unsafe fn ata_access(direction: Direction, drive: u8, lba: u32, count: u8, buf: &[u8]) -> PollingResult {
        let mut mode;
        let mut io = [0u8; 6];
        let mut head;
        let channel = DEVICES[drive as usize].channel;
        let slave = (DEVICES[drive as usize].drive as u8) << 4;
        let bus = CHANNELS[channel as usize].base;
        let dma = false;
        let mut buf = buf.as_ptr() as *const u8 as u64;

        IRQ_INVOKED = false;
        CHANNELS[channel as usize].no_int = 2;
        write(channel, Register::Control, 2);

        if lba >= 0x10000000 {
            mode = LBAMode::LBA48;
            io[0] = ((lba & 0x000000FF) >> 0)  as u8;
            io[1] = ((lba & 0x0000FF00) >> 8)  as u8;
            io[2] = ((lba & 0x00FF0000) >> 16) as u8;
            io[3] = ((lba & 0xFF000000) >> 24) as u8;
            io[4] = 0;
            io[5] = 0;
            head  = 0;
        } else if (DEVICES[drive as usize].capabilities & 0x200) != 0 {
            mode = LBAMode::LBA28;
            io[0] = ((lba & 0x00000FF) >> 0)  as u8;
            io[1] = ((lba & 0x000FF00) >> 8)  as u8;
            io[2] = ((lba & 0x0FF0000) >> 16) as u8;
            io[3] = 0;
            io[4] = 0;
            io[5] = 0;
            head  = ((lba & 0xF000000) >> 24) as u8;
        } else {
            mode = LBAMode::CHS;
            let sector = (lba % 63) as u8 + 1;
            let tmp = lba + 1 - sector as u32;
            let cylinder = (tmp / 0x3F0) as u16;
            let tmp2 = (tmp % 0x3F0) as u16;
            io[0] = sector;
            io[1] = ((cylinder >> 0) & 0xFF) as u8;
            io[2] = ((cylinder >> 8) & 0xFF) as u8;
            io[3] = 0;
            io[4] = 0;
            io[5] = 0;
            head = (tmp2 / 63) as u8;
        }

        while (read(channel, Register::CommandAndStatus) & Status::Busy as u8) != 0 {}

        write(channel, Register::HddEvSel, match mode {
            LBAMode::CHS => 0xA0,
            _ => 0xE0
        } | slave | head);

        if mode == LBAMode::LBA48 {
            write(channel, Register::SecCount1, 0);
            write(channel, Register::LBA3, io[3]);
            write(channel, Register::LBA4, io[4]);
            write(channel, Register::LBA5, io[5]);
        }
        write(channel, Register::SecCount0, count);
        write(channel, Register::LBA0, io[0]);
        write(channel, Register::LBA1, io[1]);
        write(channel, Register::LBA2, io[2]);

        write(channel, Register::CommandAndStatus, if dma {
            if direction == Direction::Read {
                match mode {
                    LBAMode::LBA48 => Command::ReadDMAExt,
                    _ => Command::ReadDMA
                }
            } else {
                match mode {
                    LBAMode::LBA48 => Command::WriteDMAExt,
                    _ => Command::WriteDMA
                }
            }
        } else {
            if direction == Direction::Read {
                match mode {
                    LBAMode::LBA48 => Command::ReadPIOExt,
                    _ => Command::ReadPIO
                }
            } else {
                match mode {
                    LBAMode::LBA48 => Command::WritePIOExt,
                    _ => Command::WritePIO
                }
            }
        } as u8);

        if !dma {
            if direction == Direction::Read {
                for _ in 0..count {
                    polling(channel, true)?;
                    asm!("insw", in("rdi") buf, in("dx") bus, in("cx") 256);
                    buf += 512;
                }
            } else {
                for _ in 0..count {
                    let _ = polling(channel, false);
                    asm!("outsw", in("rsi") buf, in("dx") bus, in("cx") 256);
                    buf += 512;
                }
                write(channel, Register::CommandAndStatus, match mode {
                    LBAMode::LBA48 => Command::CacheFlushExt,
                    _ => Command::CacheFlush
                } as u8);
                let _ = polling(channel, false);
            }
        }

        Ok(())
    }



    pub unsafe fn scan() {
        CHANNELS[Channel::Primary as usize].base = 0x1F0;
        CHANNELS[Channel::Primary as usize].control = 0x3F4;
        CHANNELS[Channel::Primary as usize].bus_master_ide = 0;
        CHANNELS[Channel::Secondary as usize].base = 0x170;
        CHANNELS[Channel::Secondary as usize].control = 0x370;
        CHANNELS[Channel::Secondary as usize].bus_master_ide = 8;

        write(Channel::Primary, Register::Control, 2);
        write(Channel::Secondary, Register::Control, 2);

        let mut err;
        let mut ty;
        let mut status;
        let mut count = 0;

        for i in Channel::Primary {
            for j in 0..2 {
                err = false;
                ty = InterfaceType::ATA;
                DEVICES[count].reserved = false;

                write(i, Register::HddEvSel, 0xA0 | (j << 4));
                sleep(Time::from(1u32));

                write(i, Register::CommandAndStatus, Command::Identify as u8);
                sleep(Time::from(1u32));

                if read(i, Register::CommandAndStatus) == 0 { continue }

                loop {
                    status = Status::from(read(i, Register::CommandAndStatus));
                    if status.contain(Status::Error) {
                        err = true;
                        break
                    } else if !status.contain(Status::Busy) && status.contain(Status::DataRequestReady) {
                        break
                    }
                }

                if err {
                    let cl = read(i, Register::LBA1);
                    let ch = read(i, Register::LBA2);

                    if (cl == 0x14 && ch == 0xEB) || (cl == 0x69 && ch == 0x96) {
                        ty = InterfaceType::ATAPI
                    } else {
                        continue
                    }

                    write(i, Register::CommandAndStatus, Command::Identify as u8);
                    sleep(Time::from(1u32))
                }

                read_buffer(i, Register::Data);

                DEVICES[count].reserved = true;
                DEVICES[count].r#type = ty;
                DEVICES[count].channel = i;
                DEVICES[count].drive = Owning::try_from(j).unwrap();
                DEVICES[count].signature = *((&BUFFER as *const u8 as u64 + IDSpace::DeviceType as u64) as *const u16);
                DEVICES[count].capabilities = *((&BUFFER as *const u8 as u64 + IDSpace::Capabilities as u64) as *const u16);
                DEVICES[count].command_sets = *((&BUFFER as *const u8 as u64 + IDSpace::CommandSets as u64) as *const u32);

                if (DEVICES[count].command_sets & (1 << 26)) != 0 {
                    DEVICES[count].size = *((&BUFFER as *const u8 as u64 + IDSpace::MaxLBAExt as u64) as *const u32)
                } else {
                    DEVICES[count].size = *((&BUFFER as *const u8 as u64 + IDSpace::MaxLBA as u64) as *const u32)
                }

                let mut k = 0;
                while k < 40 {
                    DEVICES[count].model[k] = BUFFER[IDSpace::Model as usize + k + 1];
                    DEVICES[count].model[k + 1] = BUFFER[IDSpace::Model as usize + k];
                    k += 2;
                }
                DEVICES[count].model[40] = 0;

                count += 1;
            }
        }
        for i in 0..4 {
            if DEVICES[i].reserved {
                println!("Found {:?} Drive ({} bytes) at {}.{} - {}", DEVICES[i].r#type, DEVICES[i].size, DEVICES[i].channel as u8, DEVICES[i].drive as u8, String::from_utf8(Vec::from(DEVICES[i].model)).unwrap().as_str())
            }
        }
    }

}

#[cfg(all(feature = "hdd", feature = "allocator", feature = "enum", feature = "time"))]
pub use private::*;
