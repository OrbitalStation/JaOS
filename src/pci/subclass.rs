use crate::pci::class_name::ClassName::Unclassified;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UnclassifiedSubclass {
    NonVGA,
    VGA,

    Count = 2
}

impl UnclassifiedSubclass {
    pub fn new(subclass_code: u8) -> Option <UnclassifiedSubclass> {
        match subclass_code {
            0 => Some(UnclassifiedSubclass::NonVGA),
            1 => Some(UnclassifiedSubclass::VGA),
            _ => None
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            UnclassifiedSubclass::NonVGA => "Non-VGA-Compatible",
            UnclassifiedSubclass::VGA => "VGA-Compatible",
            _ => ""
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DiskControllerSubclass {
    SCSIBus,
    IDE,
    FloppyDisk,
    IPIBus,
    RAID,
    ATA,
    SerialATA,
    SerialAttachedSCSI,
    NonVolatileMemory,
    Other = 0x80,

    Count = 10
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NetworkInterfaceSubclass {
    Ethernet,
    TokenRing,
    FDDI,
    ATM,
    ISDN,
    WorldFip,
    PICMG214MultiComputing, //< PICMG 2.14 Multi Computing
    Infiniband,
    Fabric,
    Other = 0x80,

    Count = 10
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GraphicsAdapterSubclass {
    VGACompatible,
    XGA,
    VGAIncompatible3D,
    Other = 0x80,

    Count = 4
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MultimediaControllerSubclass {
    MultimediaVideo,
    MultimediaAudio,
    ComputerTelephony,
    AudioDevice,
    Other = 0x80,

    Count = 5
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MemoryControllerSubclass {
    RAM,
    Flash,
    Other = 0x80,

    Count = 3
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BridgeDeviceSubclass {
    Host,
    ISA,
    EISA,
    MCA,
    PCI2PCI, //< PCI-to-PCI
    PCMCIA,
    NuBus,
    CardBus,
    RACEway,
    PCI2PCI2, //< Second PCI-to-PCI
    InfiniBand2PCI, //< InfiniBand-to-PCI Host
    Other = 0x80,

    Count = 12
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CommunicationControllerSubclass {
    Serial,
    Parallel,
    MultiportSerial,
    Modem,
    IEEE48812GPIB, //< IEEE 488.1/2 (GPIB)
    SmartCard,
    Other = 0x80,

    Count = 7
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SystemDeviceSubclass {
    PIC,
    DMA,
    Timer,
    RTC,
    PCIHotPlug,
    SD,
    IOMMU,
    Other = 0x80,

    Count = 8
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InputDeviceSubclass {
    Keyboard,
    DigitizerPen,
    Mouse,
    Scanner,
    Gameport,
    Other = 0x80,

    Count = 6
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DockingStationSubclass {
    Generic,
    Other = 0x80,

    Count = 2
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CPUSubclass {
    I386,
    I486,
    Pentium,
    PentiumPro,
    Alpha = 0x10,
    PowerPC = 0x20,
    MIPS = 0x30,
    CoProcessor = 0x40,
    Other = 0x80,

    Count = 9
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SerialBusSubclass {
    FireWireIEEE1394, //< FireWire (IEEE 1394)
    ACCESS,
    SSA,
    USB,
    Fibre,
    SMBus,
    InfiniBand,
    IPMI,
    SERCOSIEC61491, //< SERCOS Interface (IEC 61491)
    CAN,
    Other = 0x80,

    Count = 11
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WirelessControllerSubclass {
    IRDACompatible,
    ConsumerIR,
    RF = 0x10,
    Bluetooth = 0x11,
    Broadband = 0x12,
    Ethernet8021a = 0x20, //< Ethernet (802.1a)
    Ethernet8021b = 0x21, //< Ethernet (802.1b)
    Other = 0x80,

    Count = 8
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IntelligentControllerSubclass {
    I20,

    Count = 1
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SatelliteControllerSubclass {
    TV,
    Audio,
    Voice,
    Data,

    Count = 4
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EncryptionControllerSubclass {
    NetworkAndComputing,
    Entertainment = 0x10,
    Other = 0x80,

    Count = 3
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SignalProcessingControllerSubclass {
    DPIOModules,
    PerformanceCounters,
    CommunicationSynchronizer = 0x10,
    SignalProcessingManagement = 0x20,
    Other = 0x80,

    Count = 5
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ProprietaryDeviceSubclass {
    Count = 0
}
