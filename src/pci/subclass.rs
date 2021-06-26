use core::fmt::{Display, Formatter, Result};
use crate::pci::*;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UnclassifiedSubclassBase <N, V> {
    NonVGA(N),
    VGA(V),

    Count = 2
}

pub type UnclassifiedSubclass = UnclassifiedSubclassBase <(), ()>;

impl Display for UnclassifiedSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::NonVGA(_) => f.write_str("Non VGA-compatible"),
            Self::VGA(_) => f.write_str("VGA-compatible"),
            _ => f.write_str("ERROR")
        }
    }
}

impl PCIEnum for UnclassifiedSubclass {
    fn new(from: u8) -> Option <Self> {
        match from {
            0 => Some(Self::NonVGA(())),
            1 => Some(Self::VGA(())),
            _ => None
        }
    }
}

pub type UnclassifiedSubclassPib = UnclassifiedSubclassBase <
    EmptyPib,
    EmptyPib
>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DiskControllerSubclassBase <SCSI, IDE, F, IPI, RAID, ATA, SATA, SASCSI, NVM, O> {
    SCSIBus(SCSI),
    IDE(IDE),
    FloppyDisk(F),
    IPIBus(IPI),
    RAID(RAID),
    ATA(ATA),
    SerialATA(SATA),
    SerialAttachedSCSI(SASCSI),
    NonVolatileMemory(NVM),
    Other(O) = 0x80,

    Count = 10
}

pub type DiskControllerSubclass = DiskControllerSubclassBase <(), (), (), (), (), (), (), (), (), ()>;

impl Display for DiskControllerSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::SCSIBus(_) => f.write_str("SCSI"),
            Self::IDE(_) => f.write_str("IDE"),
            Self::FloppyDisk(_) => f.write_str("Floppy"),
            Self::IPIBus(_) => f.write_str("IPI"),
            Self::RAID(_) => f.write_str("RAID"),
            Self::ATA(_) => f.write_str("ATA"),
            Self::SerialATA(_) => f.write_str("Serial ATA"),
            Self::SerialAttachedSCSI(_) => f.write_str("Serial attached SCSI"),
            Self::NonVolatileMemory(_) => f.write_str("Non-volatile memory"),
            Self::Other(_) => f.write_str("Other"),
            Self::Count => f.write_str("ERROR")
        }
    }
}

impl PCIEnum for DiskControllerSubclass {
    fn new(from: u8) -> Option <Self> {
        if from > 8 && from != 0x80 {
            return None
        }
        unsafe { Some(*((&from as *const u8) as *const Self)) }
    }
}

pub type DiskControllerSubclassPib = DiskControllerSubclassBase <
    EmptyPib,
    DiskControllerIDEPib,
    EmptyPib,
    EmptyPib,
    EmptyPib,
    DiskControllerATAPib,
    DiskControllerSerialATAPib,
    DiskControllerSerialAttachedSCSIPib,
    DiskControllerNonVolatileMemoryPib,
    EmptyPib
>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NetworkInterfaceSubclassBase <E> { //< There's no one pib, so only one generic
    Ethernet(E),
    TokenRing(E),
    FDDI(E),
    ATM(E),
    ISDN(E),
    WorldFip(E),
    PICMG214MultiComputing(E), //< PICMG 2.14 Multi Computing
    Infiniband(E),
    Fabric(E),
    Other(E) = 0x80,

    Count = 10
}

pub type NetworkInterfaceSubclass = NetworkInterfaceSubclassBase <()>;

impl Display for NetworkInterfaceSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Ethernet(_) => f.write_str("Ethernet"),
            Self::TokenRing(_) => f.write_str("Token ring"),
            Self::FDDI(_) => f.write_str("FDDI"),
            Self::ATM(_) => f.write_str("ATM"),
            Self::ISDN(_) => f.write_str("ISDN"),
            Self::WorldFip(_) => f.write_str("World fip"),
            Self::PICMG214MultiComputing(_) => f.write_str("PICMG 2.14 Multi Computing"),
            Self::Infiniband(_) => f.write_str("Infiniband"),
            Self::Fabric(_) => f.write_str("Fabric"),
            Self::Other(_) => f.write_str("Other"),
            Self::Count => f.write_str("ERROR")
        }
    }
}

impl PCIEnum for NetworkInterfaceSubclass {
    fn new(from: u8) -> Option <Self> {
        if from > 9 && from != 0x80 {
            return None
        }
        unsafe { Some(*((&from as *const u8) as *const Self)) }
    }
}

pub type NetworkInterfaceSubclassPib = NetworkInterfaceSubclassBase <EmptyPib>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GraphicsAdapterSubclassBase <V, E> {
    VGACompatible(V),
    XGA(E),
    VGAIncompatible3D(E),
    Other(E) = 0x80,

    Count = 4
}

pub type GraphicsAdapterSubclass = GraphicsAdapterSubclassBase <(), ()>;

impl Display for GraphicsAdapterSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::VGACompatible(_) => f.write_str("VGA-compatible"),
            Self::XGA(_) => f.write_str("XGA"),
            Self::VGAIncompatible3D(_) => f.write_str("VGA-incompatible (3D)"),
            Self::Other(_) => f.write_str("Other"),
            _ => f.write_str("ERROR")
        }
    }
}

impl PCIEnum for GraphicsAdapterSubclass {
    fn new(from: u8) -> Option <Self> {
        if from > 2 && from != 0x80 {
            return None
        }
        unsafe { Some(*((&from as *const u8) as *const Self)) }
    }
}

pub type GraphicsAdapterSubclassPib = GraphicsAdapterSubclassBase <
    GraphicsAdapterVGACompatiblePib,
    EmptyPib
>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MultimediaControllerSubclassBase <E> {
    MultimediaVideo(E),
    MultimediaAudio(E),
    ComputerTelephony(E),
    AudioDevice(E),
    Other(E) = 0x80,

    Count = 5
}

pub type MultimediaControllerSubclass = MultimediaControllerSubclassBase <()>;

impl Display for MultimediaControllerSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::MultimediaVideo(_) => f.write_str("Multimedia (Video)"),
            Self::MultimediaAudio(_) => f.write_str("Multimedia (Audio)"),
            Self::ComputerTelephony(_) => f.write_str("Computer telephony"),
            Self::AudioDevice(_) => f.write_str("Audio device"),
            Self::Other(_) => f.write_str("Other"),
            _ => f.write_str("ERROR")
        }
    }
}

impl PCIEnum for MultimediaControllerSubclass {
    fn new(from: u8) -> Option <Self> {
        if from > 3 && from != 0x80 {
            return None
        }
        unsafe { Some(*((&from as *const u8) as *const Self)) }
    }
}

pub type MultimediaControllerSubclassPib = MultimediaControllerSubclassBase <EmptyPib>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MemoryControllerSubclassBase <E> {
    RAM(E),
    Flash(E),
    Other(E) = 0x80,

    Count = 3
}

pub type MemoryControllerSubclass = MemoryControllerSubclassBase <()>;

impl Display for MemoryControllerSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::RAM(_) => f.write_str("RAM"),
            Self::Flash(_) => f.write_str("Flash"),
            Self::Other(_) => f.write_str("Other"),
            _ => f.write_str("ERROR")
        }
    }
}

impl PCIEnum for MemoryControllerSubclass {
    fn new(from: u8) -> Option <Self> {
        match from {
            0 => Some(Self::RAM(())),
            1 => Some(Self::Flash(())),
            0x80 => Some(Self::Other(())),
            _ => None
        }
    }
}

pub type MemoryControllerSubclassPib = MemoryControllerSubclassBase <EmptyPib>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BridgeDeviceSubclassBase <P1, R, P2, E> {
    Host(E),
    ISA(E),
    EISA(E),
    MCA(E),
    PCI2PCI(P1), //< PCI-to-PCI
    PCMCIA(E),
    NuBus(E),
    CardBus(E),
    RACEway(R),
    PCI2PCI2(P2), //< Second PCI-to-PCI
    InfiniBand2PCI(E), //< InfiniBand-to-PCI Host
    Other(E) = 0x80,

    Count = 12
}

pub type BridgeDeviceSubclass = BridgeDeviceSubclassBase <(), (), (), ()>;

impl Display for BridgeDeviceSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Host(_) => f.write_str("Host"),
            Self::ISA(_) => f.write_str("ISA"),
            Self::EISA(_) => f.write_str("EISA"),
            Self::MCA(_) => f.write_str("MCA"),
            Self::PCI2PCI(_) | Self::PCI2PCI2(_) => f.write_str("PCI-to-PCI"),
            Self::PCMCIA(_) => f.write_str("PCMCIA"),
            Self::NuBus(_) => f.write_str("NuBus"),
            Self::CardBus(_) => f.write_str("CardBus"),
            Self::RACEway(_) => f.write_str("RACEway"),
            Self::InfiniBand2PCI(_) => f.write_str("Infiniband-to-PCI Host"),
            Self::Other(_) => f.write_str("Other"),
            _ => f.write_str("ERROR")
        }
    }
}

impl PCIEnum for BridgeDeviceSubclass {
    fn new(from: u8) -> Option <Self> {
        if from > 10 && from != 0x80 {
            return None
        }
        unsafe { Some(*((&from as *const u8) as *const Self)) }
    }
}

pub type BridgeDeviceSubclassPib = BridgeDeviceSubclassBase <
    BridgeDevicePCI2PCIPib,
    BridgeDeviceRACEwayPib,
    BridgeDevicePCI2PCI2Pib,
    EmptyPib
>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CommunicationControllerSubclassBase <S, P, M, E> {
    Serial(S),
    Parallel(P),
    MultiportSerial(E),
    Modem(M),
    IEEE48812GPIB(E), //< IEEE 488.1/2 (GPIB)
    SmartCard(E),
    Other(E) = 0x80,

    Count = 7
}

pub type CommunicationControllerSubclass = CommunicationControllerSubclassBase <(), (), (), ()>;

impl Display for CommunicationControllerSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            Self::Serial(_) => "Serial",
            Self::Parallel(_) => "Parallel",
            Self::MultiportSerial(_) => "Multi-port serial",
            Self::Modem(_) => "Modem",
            Self::IEEE48812GPIB(_) => "IEEE 488.1/2 (GPIB)",
            Self::SmartCard(_) => "Smart card",
            Self::Other(_) => "Other",
            _ => "ERROR"
        })
    }
}

impl PCIEnum for CommunicationControllerSubclass {
    fn new(from: u8) -> Option <Self> {
        if from > 5 && from != 0x80 {
            return None
        }
        unsafe { Some(*((&from as *const u8) as *const Self)) }
    }
}

pub type CommunicationControllerSubclassPib =  CommunicationControllerSubclassBase <
    CommunicationControllerSerialPib,
    CommunicationControllerParallelPib,
    CommunicationControllerModemPib,
    EmptyPib
>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SystemDeviceSubclassBase <P, D, T, R, E> {
    PIC(P),
    DMA(D),
    Timer(T),
    RTC(R),
    PCIHotPlug(E),
    SD(E),
    IOMMU(E),
    Other(E) = 0x80,

    Count = 8
}

pub type SystemDeviceSubclass = SystemDeviceSubclassBase <(), (), (), (), ()>;

impl Display for SystemDeviceSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            Self::PIC(_) => "PIC",
            Self::DMA(_) => "DMA",
            Self::Timer(_) => "Timer",
            Self::RTC(_) => "RTC",
            Self::PCIHotPlug(_) => "PCI hot plug",
            Self::SD(_) => "SD",
            Self::IOMMU(_) => "IOMMU",
            Self::Other(_) => "Other",
            _ => "ERROR"
        })
    }
}

impl PCIEnum for SystemDeviceSubclass {
    fn new(from: u8) -> Option <Self> {
        if from > 6 && from != 0x80 {
            return None
        }
        unsafe { Some(*((&from as *const u8) as *const Self)) }
    }
}

pub type SystemDeviceSubclassPib = SystemDeviceSubclassBase <
    SystemDevicePICPib,
    SystemDeviceDMAPib,
    SystemDeviceTimerPib,
    SystemDeviceRTCPib,
    EmptyPib
>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InputDeviceSubclassBase <G, E> {
    Keyboard(E),
    DigitizerPen(E),
    Mouse(E),
    Scanner(E),
    Gameport(G),
    Other(E) = 0x80,

    Count = 6
}

pub type InputDeviceSubclass = InputDeviceSubclassBase <(), ()>;

impl Display for InputDeviceSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            Self::Keyboard(_) => "Keyboard",
            Self::DigitizerPen(_) => "Digitizer pen",
            Self::Mouse(_) => "Mouse",
            Self::Scanner(_) => "Scanner",
            Self::Gameport(_) => "Game port",
            Self::Other(_) => "Other",
            _ => "ERROR"
        })
    }
}

impl PCIEnum for InputDeviceSubclass {
    fn new(from: u8) -> Option <Self> {
        if from > 4 && from != 0x80 {
            return None
        }
        unsafe { Some(*((&from as *const u8) as *const Self)) }
    }
}

pub type InputDeviceSubclassPib = InputDeviceSubclassBase <
    InputDeviceGameportPib,
    EmptyPib
>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DockingStationSubclassBase <E> {
    Generic(E),
    Other(E) = 0x80,

    Count = 2
}

pub type DockingStationSubclass = DockingStationSubclassBase <()>;

impl Display for DockingStationSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            Self::Generic(_) => "Generic",
            Self::Other(_) => "Other",
            _ => "ERROR"
        })
    }
}

impl PCIEnum for DockingStationSubclass {
    fn new(from: u8) -> Option <Self> {
        match from {
            0 => Some(Self::Generic(())),
            0x80 => Some(Self::Other(())),
            _ => None
        }
    }
}

pub type DockingStationSubclassPib = DockingStationSubclassBase <EmptyPib>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CPUSubclassBase <E> {
    I386(E),
    I486(E),
    Pentium(E),
    PentiumPro(E),
    Alpha(E) = 0x10,
    PowerPC(E) = 0x20,
    MIPS(E) = 0x30,
    CoProcessor(E) = 0x40,
    Other(E) = 0x80,

    Count = 9
}

pub type CPUSubclass = CPUSubclassBase <()>;

impl Display for CPUSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            Self::I386(_) => "Intel-386",
            Self::I486(_) => "Intel-486",
            Self::Pentium(_) => "Pentium",
            Self::PentiumPro(_) => "Pentium Pro",
            Self::Alpha(_) => "Alpha",
            Self::PowerPC(_) => "Power PC",
            Self::MIPS(_) => "MIPS",
            Self::CoProcessor(_) => "Co-processor",
            Self::Other(_) => "Other",
            _ => "ERROR"
        })
    }
}

impl PCIEnum for CPUSubclass {
    fn new(from: u8) -> Option <Self> {
        match from {
            0..=3 | 0x10 | 0x20 | 0x30 | 0x40 | 0x80 => unsafe { Some(*((&from as *const u8) as *const Self)) },
            _ => None
        }
    }
}

pub type CPUSubclassPib = CPUSubclassBase <EmptyPib>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SerialBusSubclassBase <F, U, I, E> {
    FireWireIEEE1394(F), //< FireWire (IEEE 1394)
    ACCESS(E),
    SSA(E),
    USB(U),
    Fibre(E),
    SMBus(E),
    InfiniBand(E),
    IPMI(I),
    SERCOSIEC61491(E), //< SERCOS Interface (IEC 61491)
    CAN(E),
    Other(E) = 0x80,

    Count = 11
}

pub type SerialBusSubclass = SerialBusSubclassBase <(), (), (), ()>;

impl Display for SerialBusSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            Self::FireWireIEEE1394(_) => "FireWire (IEEE 1394)",
            Self::ACCESS(_) => "ACCESS",
            Self::SSA(_) => "SSA",
            Self::USB(_) => "USB",
            Self::Fibre(_) => "Fibre",
            Self::SMBus(_) => "SMBus",
            Self::InfiniBand(_) => "Infiniband",
            Self::IPMI(_) => "IPMI",
            Self::SERCOSIEC61491(_) => "SERCOS Interface (IEC 61491)",
            Self::CAN(_) => "CAN",
            Self::Other(_) => "Other",
            _ => "ERROR"
        })
    }
}

impl PCIEnum for SerialBusSubclass {
    fn new(from: u8) -> Option <Self> {
        if from > 9 && from != 0x80 {
            return None
        }
        unsafe { Some(*((&from as *const u8) as *const Self)) }
    }
}

pub type SerialBusSubclassPib = SerialBusSubclassBase <
    SerialBusFireWireIEEE1394Pib,
    SerialBusUSBPib,
    SerialBusIPMIPib,
    EmptyPib
>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WirelessControllerSubclassBase <E> {
    IRDACompatible(E),
    ConsumerIR(E),
    RF(E) = 0x10,
    Bluetooth(E) = 0x11,
    Broadband(E) = 0x12,
    Ethernet8021a(E) = 0x20, //< Ethernet (802.1a)
    Ethernet8021b(E) = 0x21, //< Ethernet (802.1b)
    Other(E) = 0x80,

    Count = 8
}

pub type WirelessControllerSubclass = WirelessControllerSubclassBase <()>;

impl Display for WirelessControllerSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            Self::IRDACompatible(_) => "IRDA-compatible",
            Self::ConsumerIR(_) => "Consumer IR",
            Self::RF(_) => "RF",
            Self::Bluetooth(_) => "Bluetooth",
            Self::Broadband(_) => "Broadband",
            Self::Ethernet8021a(_) => "Ethernet (802.1a)",
            Self::Ethernet8021b(_) => "Ethernet (802.1b)",
            Self::Other(_) => "Other",
            _ => "ERROR"
        })
    }
}

impl PCIEnum for WirelessControllerSubclass {
    fn new(from: u8) -> Option <Self> {
        match from {
            0 | 1 | 0x10 | 0x11 | 0x12 | 0x20 | 0x21 | 0x80 => unsafe { Some(*((&from as *const u8) as *const Self)) },
            _ => None
        }
    }
}

pub type WirelessControllerSubclassPib = WirelessControllerSubclassBase <EmptyPib>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IntelligentControllerSubclassBase <E> {
    I20(E),

    Count = 1
}

pub type IntelligentControllerSubclass = IntelligentControllerSubclassBase <()>;

impl Display for IntelligentControllerSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            Self::I20(_) => "I20",
            _ => "ERROR"
        })
    }
}

impl PCIEnum for IntelligentControllerSubclass {
    fn new(from: u8) -> Option <Self> {
        if from == 1 { Some(Self::I20(())) } else { None }
    }
}

pub type IntelligentControllerSubclassPib = IntelligentControllerSubclassBase <EmptyPib>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SatelliteControllerSubclassBase <E> {
    TV(E),
    Audio(E),
    Voice(E),
    Data(E),

    Count = 4
}

pub type SatelliteControllerSubclass = SatelliteControllerSubclassBase <()>;

impl Display for SatelliteControllerSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            Self::TV(_) => "TV",
            Self::Audio(_) => "Audio",
            Self::Voice(_) => "Voice",
            Self::Data(_) => "Data",
            _ => "ERROR"
        })
    }
}

impl PCIEnum for SatelliteControllerSubclass {
    fn new(from: u8) -> Option <Self> {
        if from > 3 {
            return None
        }
        unsafe { Some(*((&from as *const u8) as *const Self)) }
    }
}

pub type SatelliteControllerSubclassPib = SatelliteControllerSubclassBase <EmptyPib>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EncryptionControllerSubclassBase <E> {
    NetworkAndComputing(E),
    Entertainment(E) = 0x10,
    Other(E) = 0x80,

    Count = 3
}

pub type EncryptionControllerSubclass = EncryptionControllerSubclassBase <()>;

impl Display for EncryptionControllerSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            Self::NetworkAndComputing(_) => "Network & computing",
            Self::Entertainment(_) => "Entertainment",
            Self::Other(_) => "Other",
            _ => "ERROR"
        })
    }
}

impl PCIEnum for EncryptionControllerSubclass {
    fn new(from: u8) -> Option <Self> {
        if from != 0 && from != 0x10 && from != 0x80 {
            return None
        }
        unsafe { Some(*((&from as *const u8) as *const Self)) }
    }
}

pub type EncryptionControllerSubclassPib = EncryptionControllerSubclassBase <EmptyPib>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SignalProcessingControllerSubclassBase <E> {
    DPIOModules(E),
    PerformanceCounters(E),
    CommunicationSynchronizer(E) = 0x10,
    SignalProcessingManagement(E) = 0x20,
    Other(E) = 0x80,

    Count = 5
}

pub type SignalProcessingControllerSubclass = SignalProcessingControllerSubclassBase <()>;

impl Display for SignalProcessingControllerSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            Self::DPIOModules(_) => "DPIO modules",
            Self::PerformanceCounters(_) => "Performance counters",
            Self::CommunicationSynchronizer(_) => "Communication synchronizer",
            Self::SignalProcessingManagement(_) => "Signal processing management",
            Self::Other(_) => "Other",
            _ => "ERROR"
        })
    }
}

impl PCIEnum for SignalProcessingControllerSubclass {
    fn new(from: u8) -> Option <Self> {
        if from > 1 && from != 0x80 && from != 0x10 && from != 0x20 {
            return None
        }
        unsafe { Some(*((&from as *const u8) as *const Self)) }
    }
}

pub type SignalProcessingControllerSubclassPib = SignalProcessingControllerSubclassBase <EmptyPib>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ProprietaryDeviceSubclass {
    Count = 0
}

#[allow(dead_code)]
pub type ProprietaryDeviceSubclassBase = ProprietaryDeviceSubclass; //< Backward compatibility

impl Display for ProprietaryDeviceSubclass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("UNKNOWN")
    }
}

impl PCIEnum for ProprietaryDeviceSubclass {
    fn new(_: u8) -> Option <Self> {
        None
    }
}

pub type ProprietaryDeviceSubclassPib = ProprietaryDeviceSubclass;
