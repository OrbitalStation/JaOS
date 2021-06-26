use crate::pci::*;
use core::fmt;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClassNameBase <U, DC, NI, GA, MuC, MeC, BD, CC, SD, ID, DS, CPU, SB, WC, IC, SC, EC, SPC, PD> {
    Unclassified(U),
    DiskController(DC),
    NetworkInterface(NI),
    GraphicsAdapter(GA),
    MultimediaController(MuC),
    MemoryController(MeC),
    BridgeDevice(BD),
    CommunicationController(CC),
    SystemDevice(SD),
    InputDevice(ID),
    DockingStation(DS),
    CPU(CPU),
    SerialBus(SB),
    WirelessController(WC),
    IntelligentIOController(IC), //< Intelligent I/O controller
    SatelliteController(SC),
    EncryptionController(EC),
    SignalProcessingController(SPC),
    ProprietaryDevice(PD) = 0xFF,

    Count = 18
}

pub type ClassName = ClassNameBase <(), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()>;

impl PCIEnum for ClassName {
    fn new(from: u8) -> Option <ClassName> {
        if from > 17 && from != 0xFF {
            return None
        }
        unsafe { Some(*((&from as *const u8) as *const Self)) }
    }
}

impl fmt::Display for ClassName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClassNameBase::Unclassified(_)               => f.write_str("Unclassified"),
            ClassNameBase::DiskController(_)             => f.write_str("Disk Controller"),
            ClassNameBase::NetworkInterface(_)           => f.write_str("Network Interface"),
            ClassNameBase::GraphicsAdapter(_)            => f.write_str("Graphics Adapter"),
            ClassNameBase::MultimediaController(_)       => f.write_str("Multimedia Controller"),
            ClassNameBase::MemoryController(_)           => f.write_str("Memory Controller"),
            ClassNameBase::BridgeDevice(_)               => f.write_str("Bridge Device"),
            ClassNameBase::CommunicationController(_)    => f.write_str("Communication Controller"),
            ClassNameBase::SystemDevice(_)               => f.write_str("System Device"),
            ClassNameBase::InputDevice(_)                => f.write_str("Input Device"),
            ClassNameBase::DockingStation(_)             => f.write_str("Docking Station"),
            ClassNameBase::CPU(_)                        => f.write_str("CPU"),
            ClassNameBase::SerialBus(_)                  => f.write_str("Serial Bus"),
            ClassNameBase::WirelessController(_)         => f.write_str("Wireless Controller"),
            ClassNameBase::IntelligentIOController(_)    => f.write_str("Intelligent I/O Controller"),
            ClassNameBase::SatelliteController(_)        => f.write_str("Satellite Controller"),
            ClassNameBase::EncryptionController(_)       => f.write_str("Encryption Controller"),
            ClassNameBase::SignalProcessingController(_) => f.write_str("Signal Processing Controller"),
            ClassNameBase::ProprietaryDevice(_)          => f.write_str("Proprietary Device"),
            ClassNameBase::Count                         => f.write_str("ERROR"),
        }
    }
}

pub type ClassNameSub = ClassNameBase <UnclassifiedSubclass,
    DiskControllerSubclass,
    NetworkInterfaceSubclass,
    GraphicsAdapterSubclass,
    MultimediaControllerSubclass,
    MemoryControllerSubclass,
    BridgeDeviceSubclass,
    CommunicationControllerSubclass,
    SystemDeviceSubclass,
    InputDeviceSubclass,
    DockingStationSubclass,
    CPUSubclass,
    SerialBusSubclass,
    WirelessControllerSubclass,
    IntelligentControllerSubclass,
    SatelliteControllerSubclass,
    EncryptionControllerSubclass,
    SignalProcessingControllerSubclass,
    ProprietaryDeviceSubclass
>;

impl fmt::Display for ClassNameSub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClassNameBase::Unclassified(x)               => x.fmt(f),
            ClassNameBase::DiskController(x)             => x.fmt(f),
            ClassNameBase::NetworkInterface(x)           => x.fmt(f),
            ClassNameBase::GraphicsAdapter(x)            => x.fmt(f),
            ClassNameBase::MultimediaController(x)       => x.fmt(f),
            ClassNameBase::MemoryController(x)           => x.fmt(f),
            ClassNameBase::BridgeDevice(x)               => x.fmt(f),
            ClassNameBase::CommunicationController(x)    => x.fmt(f),
            ClassNameBase::SystemDevice(x)               => x.fmt(f),
            ClassNameBase::InputDevice(x)                => x.fmt(f),
            ClassNameBase::DockingStation(x)             => x.fmt(f),
            ClassNameBase::CPU(x)                        => x.fmt(f),
            ClassNameBase::SerialBus(x)                  => x.fmt(f),
            ClassNameBase::WirelessController(x)         => x.fmt(f),
            ClassNameBase::IntelligentIOController(x)    => x.fmt(f),
            ClassNameBase::SatelliteController(x)        => x.fmt(f),
            ClassNameBase::EncryptionController(x)       => x.fmt(f),
            ClassNameBase::SignalProcessingController(x) => x.fmt(f),
            ClassNameBase::ProprietaryDevice(x)          => x.fmt(f),
            ClassNameBase::Count                         => f.write_str("ERROR")
        }
    }
}

pub type ClassNamePib = ClassNameBase <
    UnclassifiedSubclassPib,
    DiskControllerSubclassPib,
    NetworkInterfaceSubclassPib,
    GraphicsAdapterSubclassPib,
    MultimediaControllerSubclassPib,
    MemoryControllerSubclassPib,
    BridgeDeviceSubclassPib,
    CommunicationControllerSubclassPib,
    SystemDeviceSubclassPib,
    InputDeviceSubclassPib,
    DockingStationSubclassPib,
    CPUSubclassPib,
    SerialBusSubclassPib,
    WirelessControllerSubclassPib,
    IntelligentControllerSubclassPib,
    SatelliteControllerSubclassPib,
    EncryptionControllerSubclassPib,
    SignalProcessingControllerSubclassPib,
    ProprietaryDeviceSubclassPib
>;

impl fmt::Display for ClassNamePib {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClassNameBase::Unclassified(x) => match x {
                UnclassifiedSubclassBase::NonVGA(y) => y.fmt(f),
                UnclassifiedSubclassBase::VGA(y) => y.fmt(f),
                _ => f.write_str("ERROR")
            },
            ClassNameBase::DiskController(x) => match x {
                DiskControllerSubclassBase::SCSIBus(y) => y.fmt(f),
                DiskControllerSubclassBase::IDE(y) => y.fmt(f),
                DiskControllerSubclassBase::FloppyDisk(y) => y.fmt(f),
                DiskControllerSubclassBase::IPIBus(y) => y.fmt(f),
                DiskControllerSubclassBase::RAID(y) => y.fmt(f),
                DiskControllerSubclassBase::ATA(y) => y.fmt(f),
                DiskControllerSubclassBase::SerialATA(y) => y.fmt(f),
                DiskControllerSubclassBase::SerialAttachedSCSI(y) => y.fmt(f),
                DiskControllerSubclassBase::NonVolatileMemory(y) => y.fmt(f),
                DiskControllerSubclassBase::Other(y) => y.fmt(f),
                _ => f.write_str("ERROR"),
            },
            ClassNameBase::NetworkInterface(x) => match x {
                NetworkInterfaceSubclassBase::Ethernet(y) => y.fmt(f),
                NetworkInterfaceSubclassBase::TokenRing(y) => y.fmt(f),
                NetworkInterfaceSubclassBase::FDDI(y) => y.fmt(f),
                NetworkInterfaceSubclassBase::ATM(y) => y.fmt(f),
                NetworkInterfaceSubclassBase::ISDN(y) => y.fmt(f),
                NetworkInterfaceSubclassBase::WorldFip(y) => y.fmt(f),
                NetworkInterfaceSubclassBase::PICMG214MultiComputing(y) => y.fmt(f),
                NetworkInterfaceSubclassBase::Infiniband(y) => y.fmt(f),
                NetworkInterfaceSubclassBase::Fabric(y) => y.fmt(f),
                NetworkInterfaceSubclassBase::Other(y) => y.fmt(f),
                NetworkInterfaceSubclassBase::Count => f.write_str("ERROR"),
            },
            ClassNameBase::GraphicsAdapter(x) => match x {
                GraphicsAdapterSubclassBase::VGACompatible(y) => y.fmt(f),
                GraphicsAdapterSubclassBase::XGA(y) => y.fmt(f),
                GraphicsAdapterSubclassBase::VGAIncompatible3D(y) => y.fmt(f),
                GraphicsAdapterSubclassBase::Other(y) => y.fmt(f),
                GraphicsAdapterSubclassBase::Count => f.write_str("ERROR"),
            },
            ClassNameBase::MultimediaController(x) => match x {
                MultimediaControllerSubclassBase::MultimediaVideo(y) => y.fmt(f),
                MultimediaControllerSubclassBase::MultimediaAudio(y) => y.fmt(f),
                MultimediaControllerSubclassBase::ComputerTelephony(y) => y.fmt(f),
                MultimediaControllerSubclassBase::AudioDevice(y) => y.fmt(f),
                MultimediaControllerSubclassBase::Other(y) => y.fmt(f),
                MultimediaControllerSubclassBase::Count => f.write_str("ERROR")
            },
            ClassNameBase::MemoryController(x) => match x {
                MemoryControllerSubclassBase::RAM(y) => y.fmt(f),
                MemoryControllerSubclassBase::Flash(y) => y.fmt(f),
                MemoryControllerSubclassBase::Other(y) => y.fmt(f),
                MemoryControllerSubclassBase::Count => f.write_str("ERROR")
            },
            ClassNameBase::BridgeDevice(x) => match x {
                BridgeDeviceSubclassBase::Host(y) => y.fmt(f),
                BridgeDeviceSubclassBase::ISA(y) => y.fmt(f),
                BridgeDeviceSubclassBase::EISA(y) => y.fmt(f),
                BridgeDeviceSubclassBase::MCA(y) => y.fmt(f),
                BridgeDeviceSubclassBase::PCI2PCI(y) => y.fmt(f),
                BridgeDeviceSubclassBase::PCMCIA(y) => y.fmt(f),
                BridgeDeviceSubclassBase::NuBus(y) => y.fmt(f),
                BridgeDeviceSubclassBase::CardBus(y) => y.fmt(f),
                BridgeDeviceSubclassBase::RACEway(y) => y.fmt(f),
                BridgeDeviceSubclassBase::PCI2PCI2(y) => y.fmt(f),
                BridgeDeviceSubclassBase::InfiniBand2PCI(y) => y.fmt(f),
                BridgeDeviceSubclassBase::Other(y) => y.fmt(f),
                BridgeDeviceSubclassBase::Count => f.write_str("ERROR")
            },
            ClassNameBase::CommunicationController(x) => match x {
                CommunicationControllerSubclassBase::Serial(y) => y.fmt(f),
                CommunicationControllerSubclassBase::Parallel(y) => y.fmt(f),
                CommunicationControllerSubclassBase::MultiportSerial(y) => y.fmt(f),
                CommunicationControllerSubclassBase::Modem(y) => y.fmt(f),
                CommunicationControllerSubclassBase::IEEE48812GPIB(y) => y.fmt(f),
                CommunicationControllerSubclassBase::SmartCard(y) => y.fmt(f),
                CommunicationControllerSubclassBase::Other(y) => y.fmt(f),
                CommunicationControllerSubclassBase::Count => f.write_str("ERROR")
            },
            ClassNameBase::SystemDevice(x) => match x {
                SystemDeviceSubclassBase::PIC(y) => y.fmt(f),
                SystemDeviceSubclassBase::DMA(y) => y.fmt(f),
                SystemDeviceSubclassBase::Timer(y) => y.fmt(f),
                SystemDeviceSubclassBase::RTC(y) => y.fmt(f),
                SystemDeviceSubclassBase::PCIHotPlug(y) => y.fmt(f),
                SystemDeviceSubclassBase::SD(y) => y.fmt(f),
                SystemDeviceSubclassBase::IOMMU(y) => y.fmt(f),
                SystemDeviceSubclassBase::Other(y) => y.fmt(f),
                SystemDeviceSubclassBase::Count => f.write_str("ERROR")
            },
            ClassNameBase::InputDevice(x) => match x {
                InputDeviceSubclassBase::Keyboard(y) => y.fmt(f),
                InputDeviceSubclassBase::DigitizerPen(y) => y.fmt(f),
                InputDeviceSubclassBase::Mouse(y) => y.fmt(f),
                InputDeviceSubclassBase::Scanner(y) => y.fmt(f),
                InputDeviceSubclassBase::Gameport(y) => y.fmt(f),
                InputDeviceSubclassBase::Other(y) => y.fmt(f),
                InputDeviceSubclassBase::Count => f.write_str("ERROR")
            },
            ClassNameBase::DockingStation(x) => match x {
                DockingStationSubclassBase::Generic(y) => y.fmt(f),
                DockingStationSubclassBase::Other(y) => y.fmt(f),
                DockingStationSubclassBase::Count => f.write_str("ERROR")
            },
            ClassNameBase::CPU(x) => match x {
                CPUSubclassBase::I386(y) => y.fmt(f),
                CPUSubclassBase::I486(y) => y.fmt(f),
                CPUSubclassBase::Pentium(y) => y.fmt(f),
                CPUSubclassBase::PentiumPro(y) => y.fmt(f),
                CPUSubclassBase::Alpha(y) => y.fmt(f),
                CPUSubclassBase::PowerPC(y) => y.fmt(f),
                CPUSubclassBase::MIPS(y) => y.fmt(f),
                CPUSubclassBase::CoProcessor(y) => y.fmt(f),
                CPUSubclassBase::Other(y) => y.fmt(f),
                CPUSubclassBase::Count => f.write_str("ERROR")
            },
            ClassNameBase::SerialBus(x) => match x {
                SerialBusSubclassBase::FireWireIEEE1394(y) => y.fmt(f),
                SerialBusSubclassBase::ACCESS(y) => y.fmt(f),
                SerialBusSubclassBase::SSA(y) => y.fmt(f),
                SerialBusSubclassBase::USB(y) => y.fmt(f),
                SerialBusSubclassBase::Fibre(y) => y.fmt(f),
                SerialBusSubclassBase::SMBus(y) => y.fmt(f),
                SerialBusSubclassBase::InfiniBand(y) => y.fmt(f),
                SerialBusSubclassBase::IPMI(y) => y.fmt(f),
                SerialBusSubclassBase::SERCOSIEC61491(y) => y.fmt(f),
                SerialBusSubclassBase::CAN(y) => y.fmt(f),
                SerialBusSubclassBase::Other(y) => y.fmt(f),
                SerialBusSubclassBase::Count => f.write_str("ERROR")
            },
            ClassNameBase::WirelessController(x) => match x {
                WirelessControllerSubclassBase::IRDACompatible(y) => y.fmt(f),
                WirelessControllerSubclassBase::ConsumerIR(y) => y.fmt(f),
                WirelessControllerSubclassBase::RF(y) => y.fmt(f),
                WirelessControllerSubclassBase::Bluetooth(y) => y.fmt(f),
                WirelessControllerSubclassBase::Broadband(y) => y.fmt(f),
                WirelessControllerSubclassBase::Ethernet8021a(y) => y.fmt(f),
                WirelessControllerSubclassBase::Ethernet8021b(y) => y.fmt(f),
                WirelessControllerSubclassBase::Other(y) => y.fmt(f),
                WirelessControllerSubclassBase::Count => f.write_str("ERROR")
            },
            ClassNameBase::IntelligentIOController(x) => match x {
                IntelligentControllerSubclassBase::I20(y) => y.fmt(f),
                IntelligentControllerSubclassBase::Count => f.write_str("ERROR")
            },
            ClassNameBase::SatelliteController(x) => match x {
                SatelliteControllerSubclassBase::TV(y) => y.fmt(f),
                SatelliteControllerSubclassBase::Audio(y) => y.fmt(f),
                SatelliteControllerSubclassBase::Voice(y) => y.fmt(f),
                SatelliteControllerSubclassBase::Data(y) => y.fmt(f),
                SatelliteControllerSubclassBase::Count => f.write_str("ERROR")
            },
            ClassNameBase::EncryptionController(x) => match x {
                EncryptionControllerSubclassBase::NetworkAndComputing(y) => y.fmt(f),
                EncryptionControllerSubclassBase::Entertainment(y) => y.fmt(f),
                EncryptionControllerSubclassBase::Other(y) => y.fmt(f),
                EncryptionControllerSubclassBase::Count => f.write_str("ERROR")
            },
            ClassNameBase::SignalProcessingController(x) => match x {
                SignalProcessingControllerSubclassBase::DPIOModules(y) => y.fmt(f),
                SignalProcessingControllerSubclassBase::PerformanceCounters(y) => y.fmt(f),
                SignalProcessingControllerSubclassBase::CommunicationSynchronizer(y) => y.fmt(f),
                SignalProcessingControllerSubclassBase::SignalProcessingManagement(y) => y.fmt(f),
                SignalProcessingControllerSubclassBase::Other(y) => y.fmt(f),
                SignalProcessingControllerSubclassBase::Count => f.write_str("ERROR")
            },
            ClassNameBase::ProprietaryDevice(x) => match x {
                ProprietaryDeviceSubclass::Count => f.write_str("ERROR")
            },
            ClassNameBase::Count => f.write_str("ERROR")
        }
    }
}
