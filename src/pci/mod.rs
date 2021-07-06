/****************************************************************/
//                          Modules                             //
/****************************************************************/

#[cfg(feature = "pci")]
pub mod class_name;

#[cfg(feature = "pci")]
pub mod subclass;

#[cfg(feature = "pci")]
pub mod pib;

#[cfg(feature = "pci")]
mod private {

    /****************************************************************/
    //                            Uses                              //
    /****************************************************************/

    use x86_64::instructions::port::Port;
    use alloc::fmt::{Display, Debug};
    use super::{
        class_name::*,
        subclass::*,
        pib::*
    };
    use crate::println;

    /****************************************************************/
    //                         Constants                            //
    /****************************************************************/

    pub const CONFIG_ADDRESS: u16 = 0xCF8;
    pub const CONFIG_DATA:    u16 = 0xCFC;

    pub const MAX_BUSES:      u8  =   255;
    pub const MAX_DEVICES:    u8  =    32;
    pub const MAX_FUNCTIONS:  u8  =     8;

    /****************************************************************/
    //                            Types                             //
    /****************************************************************/

    pub trait PCIEnum {
        fn new(from: u8) -> Option <Self> where Self: Sized + Display + Clone + Copy + PartialEq + Eq;
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum HeaderType {
        Normal,
        Bridge,
        CardBus,
        MultiFunc = 0x80
    }

    impl HeaderType {
        pub fn has(&self, t: HeaderType) -> bool {
            ((*self as u8) & (t as u8)) != 0
        }
    }

    #[allow(non_snake_case)]
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Options {
        pub vendorID:     u16,
        pub deviceID:     u16,
        pub commandReg:   u16,
        pub statusReg:    u16,
        pub revisionID:    u8,
        pub pib:           u8, //< Should be 'progIF', but I prefer 'pib'(programmable interface byte)
        pub subClassCode:  u8,
        pub classCode:     u8,
        pub cacheLineSize: u8,
        pub latency:       u8,
        pub headerType:    HeaderType,
        pub bist:          u8
    }

    #[repr(C)]
    pub union Dev {
        pub option: Options,
        pub header: [u32; 4]
    }

    impl Dev {
        pub fn new() -> Dev {
            Dev {
                header: [0, 0, 0, 0]
            }
        }

        pub fn pib(&self) -> ClassNamePib {
            unsafe {
                match self.subclass() {
                    ClassNameSub::Unclassified(x) => ClassNamePib::Unclassified(match x {
                        UnclassifiedSubclassBase::NonVGA(_) => UnclassifiedSubclassPib::NonVGA(EmptyPib),
                        UnclassifiedSubclassBase::VGA(_) => UnclassifiedSubclassPib::VGA(EmptyPib),
                        UnclassifiedSubclassBase::Count => UnclassifiedSubclassPib::Count
                    }),
                    ClassNameSub::DiskController(x) => ClassNamePib::DiskController(match x {
                        DiskControllerSubclassBase::SCSIBus(_) => DiskControllerSubclassPib::SCSIBus(EmptyPib),
                        DiskControllerSubclassBase::IDE(_) => DiskControllerSubclassPib::IDE(DiskControllerIDEPib::new(self.option.pib).expect("Wrong 'pib'")),
                        DiskControllerSubclassBase::FloppyDisk(_) => DiskControllerSubclassPib::FloppyDisk(EmptyPib),
                        DiskControllerSubclassBase::IPIBus(_) => DiskControllerSubclassPib::IPIBus(EmptyPib),
                        DiskControllerSubclassBase::RAID(_) => DiskControllerSubclassPib::RAID(EmptyPib),
                        DiskControllerSubclassBase::ATA(_) => DiskControllerSubclassPib::ATA(DiskControllerATAPib::new(self.option.pib).expect("Wrong 'pib'")),
                        DiskControllerSubclassBase::SerialATA(_) => DiskControllerSubclassPib::SerialATA(DiskControllerSerialATAPib::new(self.option.pib).expect("Wrong 'pib'")),
                        DiskControllerSubclassBase::SerialAttachedSCSI(_) => DiskControllerSubclassPib::SerialAttachedSCSI(DiskControllerSerialAttachedSCSIPib::new(self.option.pib).expect("Wrong 'pib'")),
                        DiskControllerSubclassBase::NonVolatileMemory(_) => DiskControllerSubclassPib::NonVolatileMemory(DiskControllerNonVolatileMemoryPib::new(self.option.pib).expect("Wrong 'pib'")),
                        DiskControllerSubclassBase::Other(_) => DiskControllerSubclassPib::Other(EmptyPib),
                        DiskControllerSubclassBase::Count => DiskControllerSubclassPib::Count,
                    }),
                    ClassNameSub::NetworkInterface(x) => ClassNamePib::NetworkInterface(match x {
                        NetworkInterfaceSubclassBase::Ethernet(_) => NetworkInterfaceSubclassPib::Ethernet(EmptyPib),
                        NetworkInterfaceSubclassBase::TokenRing(_) => NetworkInterfaceSubclassPib::TokenRing(EmptyPib),
                        NetworkInterfaceSubclassBase::FDDI(_) => NetworkInterfaceSubclassPib::FDDI(EmptyPib),
                        NetworkInterfaceSubclassBase::ATM(_) => NetworkInterfaceSubclassPib::ATM(EmptyPib),
                        NetworkInterfaceSubclassBase::ISDN(_) => NetworkInterfaceSubclassPib::ISDN(EmptyPib),
                        NetworkInterfaceSubclassBase::WorldFip(_) => NetworkInterfaceSubclassPib::WorldFip(EmptyPib),
                        NetworkInterfaceSubclassBase::PICMG214MultiComputing(_) => NetworkInterfaceSubclassPib::PICMG214MultiComputing(EmptyPib),
                        NetworkInterfaceSubclassBase::Infiniband(_) => NetworkInterfaceSubclassPib::Infiniband(EmptyPib),
                        NetworkInterfaceSubclassBase::Fabric(_) => NetworkInterfaceSubclassPib::Fabric(EmptyPib),
                        NetworkInterfaceSubclassBase::Other(_) => NetworkInterfaceSubclassPib::Other(EmptyPib),
                        NetworkInterfaceSubclassBase::Count => NetworkInterfaceSubclassPib::Count,
                    }),
                    ClassNameSub::GraphicsAdapter(x) => ClassNamePib::GraphicsAdapter(match x {
                        GraphicsAdapterSubclassBase::VGACompatible(_) => GraphicsAdapterSubclassPib::VGACompatible(GraphicsAdapterVGACompatiblePib::new(self.option.pib).expect("Wrong 'pib'")),
                        GraphicsAdapterSubclassBase::XGA(_) => GraphicsAdapterSubclassPib::XGA(EmptyPib),
                        GraphicsAdapterSubclassBase::VGAIncompatible3D(_) => GraphicsAdapterSubclassPib::VGAIncompatible3D(EmptyPib),
                        GraphicsAdapterSubclassBase::Other(_) => GraphicsAdapterSubclassPib::Other(EmptyPib),
                        GraphicsAdapterSubclassBase::Count => GraphicsAdapterSubclassPib::Count
                    }),
                    ClassNameBase::MultimediaController(x) => ClassNamePib::MultimediaController(match x {
                        MultimediaControllerSubclassBase::MultimediaVideo(_) => MultimediaControllerSubclassPib::MultimediaVideo(EmptyPib),
                        MultimediaControllerSubclassBase::MultimediaAudio(_) => MultimediaControllerSubclassPib::MultimediaAudio(EmptyPib),
                        MultimediaControllerSubclassBase::ComputerTelephony(_) => MultimediaControllerSubclassPib::ComputerTelephony(EmptyPib),
                        MultimediaControllerSubclassBase::AudioDevice(_) => MultimediaControllerSubclassPib::AudioDevice(EmptyPib),
                        MultimediaControllerSubclassBase::Other(_) => MultimediaControllerSubclassPib::Other(EmptyPib),
                        MultimediaControllerSubclassBase::Count => MultimediaControllerSubclassPib::Count
                    }),
                    ClassNameBase::MemoryController(x) => ClassNamePib::MemoryController(match x {
                        MemoryControllerSubclassBase::RAM(_) => MemoryControllerSubclassPib::RAM(EmptyPib),
                        MemoryControllerSubclassBase::Flash(_) => MemoryControllerSubclassPib::Flash(EmptyPib),
                        MemoryControllerSubclassBase::Other(_) => MemoryControllerSubclassPib::Other(EmptyPib),
                        MemoryControllerSubclassBase::Count => MemoryControllerSubclassPib::Count
                    }),
                    ClassNameBase::BridgeDevice(x) => ClassNamePib::BridgeDevice(match x {
                        BridgeDeviceSubclassBase::Host(_) => BridgeDeviceSubclassPib::Host(EmptyPib),
                        BridgeDeviceSubclassBase::ISA(_) => BridgeDeviceSubclassPib::ISA(EmptyPib),
                        BridgeDeviceSubclassBase::EISA(_) => BridgeDeviceSubclassPib::EISA(EmptyPib),
                        BridgeDeviceSubclassBase::MCA(_) => BridgeDeviceSubclassPib::MCA(EmptyPib),
                        BridgeDeviceSubclassBase::PCI2PCI(_) => BridgeDeviceSubclassPib::PCI2PCI(BridgeDevicePCI2PCIPib::new(self.option.pib).expect("Wrong 'pib'")),
                        BridgeDeviceSubclassBase::PCMCIA(_) => BridgeDeviceSubclassPib::PCMCIA(EmptyPib),
                        BridgeDeviceSubclassBase::NuBus(_) => BridgeDeviceSubclassPib::NuBus(EmptyPib),
                        BridgeDeviceSubclassBase::CardBus(_) => BridgeDeviceSubclassPib::CardBus(EmptyPib),
                        BridgeDeviceSubclassBase::RACEway(_) => BridgeDeviceSubclassPib::RACEway(BridgeDeviceRACEwayPib::new(self.option.pib).expect("Wrong 'pib'")),
                        BridgeDeviceSubclassBase::PCI2PCI2(_) => BridgeDeviceSubclassPib::PCI2PCI2(BridgeDevicePCI2PCI2Pib::new(self.option.pib).expect("Wrong 'pib'")),
                        BridgeDeviceSubclassBase::InfiniBand2PCI(_) => BridgeDeviceSubclassPib::InfiniBand2PCI(EmptyPib),
                        BridgeDeviceSubclassBase::Other(_) => BridgeDeviceSubclassPib::Other(EmptyPib),
                        BridgeDeviceSubclassBase::Count => BridgeDeviceSubclassPib::Count
                    }),
                    ClassNameBase::CommunicationController(x) => ClassNamePib::CommunicationController(match x {
                        CommunicationControllerSubclassBase::Serial(_) => CommunicationControllerSubclassPib::Serial(CommunicationControllerSerialPib::new(self.option.pib).expect("Wrong 'pib'")),
                        CommunicationControllerSubclassBase::Parallel(_) => CommunicationControllerSubclassPib::Parallel(CommunicationControllerParallelPib::new(self.option.pib).expect("Wrong 'pib'")),
                        CommunicationControllerSubclassBase::MultiportSerial(_) => CommunicationControllerSubclassPib::MultiportSerial(EmptyPib),
                        CommunicationControllerSubclassBase::Modem(_) => CommunicationControllerSubclassPib::Modem(CommunicationControllerModemPib::new(self.option.pib).expect("Wrong 'pib'")),
                        CommunicationControllerSubclassBase::IEEE48812GPIB(_) => CommunicationControllerSubclassPib::IEEE48812GPIB(EmptyPib),
                        CommunicationControllerSubclassBase::SmartCard(_) => CommunicationControllerSubclassPib::SmartCard(EmptyPib),
                        CommunicationControllerSubclassBase::Other(_) => CommunicationControllerSubclassPib::Other(EmptyPib),
                        CommunicationControllerSubclassBase::Count => CommunicationControllerSubclassPib::Count
                    }),
                    ClassNameBase::SystemDevice(x) => ClassNamePib::SystemDevice(match x {
                        SystemDeviceSubclassBase::PIC(_) => SystemDeviceSubclassPib::PIC(SystemDevicePICPib::new(self.option.pib).expect("Wrong 'pib'")),
                        SystemDeviceSubclassBase::DMA(_) => SystemDeviceSubclassPib::DMA(SystemDeviceDMAPib::new(self.option.pib).expect("Wrong 'pib'")),
                        SystemDeviceSubclassBase::Timer(_) => SystemDeviceSubclassPib::Timer(SystemDeviceTimerPib::new(self.option.pib).expect("Wrong 'pib'")),
                        SystemDeviceSubclassBase::RTC(_) => SystemDeviceSubclassPib::RTC(SystemDeviceRTCPib::new(self.option.pib).expect("Wrong 'pib'")),
                        SystemDeviceSubclassBase::PCIHotPlug(_) => SystemDeviceSubclassPib::PCIHotPlug(EmptyPib),
                        SystemDeviceSubclassBase::SD(_) => SystemDeviceSubclassPib::SD(EmptyPib),
                        SystemDeviceSubclassBase::IOMMU(_) => SystemDeviceSubclassPib::IOMMU(EmptyPib),
                        SystemDeviceSubclassBase::Other(_) => SystemDeviceSubclassPib::Other(EmptyPib),
                        SystemDeviceSubclassBase::Count => SystemDeviceSubclassPib::Count
                    }),
                    ClassNameBase::InputDevice(x) => ClassNamePib::InputDevice(match x {
                        InputDeviceSubclassBase::Keyboard(_) => InputDeviceSubclassPib::Keyboard(EmptyPib),
                        InputDeviceSubclassBase::DigitizerPen(_) => InputDeviceSubclassPib::DigitizerPen(EmptyPib),
                        InputDeviceSubclassBase::Mouse(_) => InputDeviceSubclassPib::Mouse(EmptyPib),
                        InputDeviceSubclassBase::Scanner(_) => InputDeviceSubclassPib::Scanner(EmptyPib),
                        InputDeviceSubclassBase::Gameport(_) => InputDeviceSubclassPib::Gameport(InputDeviceGameportPib::new(self.option.pib).expect("Wrong 'pib'")),
                        InputDeviceSubclassBase::Other(_) => InputDeviceSubclassPib::Other(EmptyPib),
                        InputDeviceSubclassBase::Count => InputDeviceSubclassPib::Count
                    }),
                    ClassNameBase::DockingStation(x) => ClassNamePib::DockingStation(match x {
                        DockingStationSubclassBase::Generic(_) => DockingStationSubclassBase::Generic(EmptyPib),
                        DockingStationSubclassBase::Other(_) => DockingStationSubclassBase::Other(EmptyPib),
                        DockingStationSubclassBase::Count => DockingStationSubclassBase::Count
                    }),
                    ClassNameBase::CPU(x) => ClassNamePib::CPU(match x {
                        CPUSubclassBase::I386(_) => CPUSubclassBase::I386(EmptyPib),
                        CPUSubclassBase::I486(_) => CPUSubclassBase::I486(EmptyPib),
                        CPUSubclassBase::Pentium(_) => CPUSubclassBase::Pentium(EmptyPib),
                        CPUSubclassBase::PentiumPro(_) => CPUSubclassBase::PentiumPro(EmptyPib),
                        CPUSubclassBase::Alpha(_) => CPUSubclassBase::Alpha(EmptyPib),
                        CPUSubclassBase::PowerPC(_) => CPUSubclassBase::PowerPC(EmptyPib),
                        CPUSubclassBase::MIPS(_) => CPUSubclassBase::MIPS(EmptyPib),
                        CPUSubclassBase::CoProcessor(_) => CPUSubclassBase::CoProcessor(EmptyPib),
                        CPUSubclassBase::Other(_) => CPUSubclassBase::Other(EmptyPib),
                        CPUSubclassBase::Count => CPUSubclassBase::Count
                    }),
                    ClassNameBase::SerialBus(x) => ClassNamePib::SerialBus(match x {
                        SerialBusSubclassBase::FireWireIEEE1394(_) => SerialBusSubclassPib::FireWireIEEE1394(SerialBusFireWireIEEE1394Pib::new(self.option.pib).expect("Wrong 'pib'")),
                        SerialBusSubclassBase::ACCESS(_) => SerialBusSubclassPib::ACCESS(EmptyPib),
                        SerialBusSubclassBase::SSA(_) => SerialBusSubclassPib::SSA(EmptyPib),
                        SerialBusSubclassBase::USB(_) => SerialBusSubclassPib::USB(SerialBusUSBPib::new(self.option.pib).expect("Wrong 'pib'")),
                        SerialBusSubclassBase::Fibre(_) => SerialBusSubclassPib::Fibre(EmptyPib),
                        SerialBusSubclassBase::SMBus(_) => SerialBusSubclassPib::SMBus(EmptyPib),
                        SerialBusSubclassBase::InfiniBand(_) => SerialBusSubclassPib::InfiniBand(EmptyPib),
                        SerialBusSubclassBase::IPMI(_) => SerialBusSubclassPib::IPMI(SerialBusIPMIPib::new(self.option.pib).expect("Wrong 'pib'")),
                        SerialBusSubclassBase::SERCOSIEC61491(_) => SerialBusSubclassPib::SERCOSIEC61491(EmptyPib),
                        SerialBusSubclassBase::CAN(_) => SerialBusSubclassPib::CAN(EmptyPib),
                        SerialBusSubclassBase::Other(_) => SerialBusSubclassPib::Other(EmptyPib),
                        SerialBusSubclassBase::Count => SerialBusSubclassPib::Count
                    }),
                    ClassNameBase::WirelessController(x) => ClassNamePib::WirelessController(match x {
                        WirelessControllerSubclassBase::IRDACompatible(_) => WirelessControllerSubclassPib::IRDACompatible(EmptyPib),
                        WirelessControllerSubclassBase::ConsumerIR(_) => WirelessControllerSubclassPib::ConsumerIR(EmptyPib),
                        WirelessControllerSubclassBase::RF(_) => WirelessControllerSubclassPib::RF(EmptyPib),
                        WirelessControllerSubclassBase::Bluetooth(_) => WirelessControllerSubclassPib::Bluetooth(EmptyPib),
                        WirelessControllerSubclassBase::Broadband(_) => WirelessControllerSubclassPib::Broadband(EmptyPib),
                        WirelessControllerSubclassBase::Ethernet8021a(_) => WirelessControllerSubclassPib::Ethernet8021a(EmptyPib),
                        WirelessControllerSubclassBase::Ethernet8021b(_) => WirelessControllerSubclassPib::Ethernet8021b(EmptyPib),
                        WirelessControllerSubclassBase::Other(_) => WirelessControllerSubclassPib::Other(EmptyPib),
                        WirelessControllerSubclassBase::Count => WirelessControllerSubclassPib::Count
                    }),
                    ClassNameBase::IntelligentIOController(x) => ClassNamePib::IntelligentIOController(match x {
                        IntelligentControllerSubclassBase::I20(_) => IntelligentControllerSubclassPib::I20(EmptyPib),
                        IntelligentControllerSubclassBase::Count => IntelligentControllerSubclassPib::Count
                    }),
                    ClassNameBase::SatelliteController(x) => ClassNamePib::SatelliteController(match x {
                        SatelliteControllerSubclassBase::TV(_) => SatelliteControllerSubclassPib::TV(EmptyPib),
                        SatelliteControllerSubclassBase::Audio(_) => SatelliteControllerSubclassPib::Audio(EmptyPib),
                        SatelliteControllerSubclassBase::Voice(_) => SatelliteControllerSubclassPib::Voice(EmptyPib),
                        SatelliteControllerSubclassBase::Data(_) => SatelliteControllerSubclassPib::Data(EmptyPib),
                        SatelliteControllerSubclassBase::Count => SatelliteControllerSubclassPib::Count
                    }),
                    ClassNameBase::EncryptionController(x) => ClassNamePib::EncryptionController(match x {
                        EncryptionControllerSubclassBase::NetworkAndComputing(_) => EncryptionControllerSubclassPib::NetworkAndComputing(EmptyPib),
                        EncryptionControllerSubclassBase::Entertainment(_) => EncryptionControllerSubclassPib::Entertainment(EmptyPib),
                        EncryptionControllerSubclassBase::Other(_) => EncryptionControllerSubclassPib::Other(EmptyPib),
                        EncryptionControllerSubclassBase::Count => EncryptionControllerSubclassPib::Count
                    }),
                    ClassNameBase::SignalProcessingController(x) => ClassNamePib::SignalProcessingController(match x {
                        SignalProcessingControllerSubclassBase::DPIOModules(_) => SignalProcessingControllerSubclassPib::DPIOModules(EmptyPib),
                        SignalProcessingControllerSubclassBase::PerformanceCounters(_) => SignalProcessingControllerSubclassPib::PerformanceCounters(EmptyPib),
                        SignalProcessingControllerSubclassBase::CommunicationSynchronizer(_) => SignalProcessingControllerSubclassPib::CommunicationSynchronizer(EmptyPib),
                        SignalProcessingControllerSubclassBase::SignalProcessingManagement(_) => SignalProcessingControllerSubclassPib::SignalProcessingManagement(EmptyPib),
                        SignalProcessingControllerSubclassBase::Other(_) => SignalProcessingControllerSubclassPib::Other(EmptyPib),
                        SignalProcessingControllerSubclassBase::Count => SignalProcessingControllerSubclassPib::Count
                    }),
                    ClassNameBase::ProprietaryDevice(x) => ClassNamePib::ProprietaryDevice(match x {
                        ProprietaryDeviceSubclass::Count => ProprietaryDeviceSubclassPib::Count
                    }),
                    ClassNameBase::Count => ClassNamePib::Count
                }
            }
        }

        pub fn subclass(&self) -> ClassNameSub {
            unsafe {
                match self.classname() {
                    ClassName::Unclassified(_)               => ClassNameSub::Unclassified(UnclassifiedSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::DiskController(_)             => ClassNameSub::DiskController(DiskControllerSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::NetworkInterface(_)           => ClassNameSub::NetworkInterface(NetworkInterfaceSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::GraphicsAdapter(_)            => ClassNameSub::GraphicsAdapter(GraphicsAdapterSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::MultimediaController(_)       => ClassNameSub::MultimediaController(MultimediaControllerSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::MemoryController(_)           => ClassNameSub::MemoryController(MemoryControllerSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::BridgeDevice(_)               => ClassNameSub::BridgeDevice(BridgeDeviceSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::CommunicationController(_)    => ClassNameSub::CommunicationController(CommunicationControllerSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::SystemDevice(_)               => ClassNameSub::SystemDevice(SystemDeviceSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::InputDevice(_)                => ClassNameSub::InputDevice(InputDeviceSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::DockingStation(_)             => ClassNameSub::DockingStation(DockingStationSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::CPU(_)                        => ClassNameSub::CPU(CPUSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::SerialBus(_)                  => ClassNameSub::SerialBus(SerialBusSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::WirelessController(_)         => ClassNameSub::WirelessController(WirelessControllerSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::IntelligentIOController(_)    => ClassNameSub::IntelligentIOController(IntelligentControllerSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::SatelliteController(_)        => ClassNameSub::SatelliteController(SatelliteControllerSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::EncryptionController(_)       => ClassNameSub::EncryptionController(EncryptionControllerSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::SignalProcessingController(_) => ClassNameSub::SignalProcessingController(SignalProcessingControllerSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::ProprietaryDevice(_)          => ClassNameSub::ProprietaryDevice(ProprietaryDeviceSubclass::new(self.option.subClassCode).expect("Wrong 'classCode'")),
                    ClassName::Count                         => ClassNameSub::Count
                }
            }
        }

        pub fn classname(&self) -> ClassName {
            unsafe {
                return ClassName::new(self.option.classCode).expect("Wrong 'classCode'");
            };
        }
    }

    pub struct ConfigAddress {
        pub val: u32
    }

    impl ConfigAddress {
        pub fn new(reg_num: u8, func_num: u8, dev_num: u8, bus_num: u8, enable: bool) -> ConfigAddress {
            ConfigAddress {
                val: (((reg_num as u32) << 2) | ((func_num as u32) << 8) | ((dev_num as u32) << 11) | ((bus_num as u32) << 16) | ((enable as u32) << 31)) as u32
            }
        }

        pub fn zero(&self) -> u8 {
            (self.val & 0b11) as u8
        }

        pub fn reg_num(&self) -> u8 {
            ((self.val >> 2) & 0b111111) as u8
        }

        pub fn func_num(&self) -> u8 {
            ((self.val >> 8) & 0b111) as u8
        }

        pub fn dev_num(&self) -> u8 {
            ((self.val >> 11) & 0b11111) as u8
        }

        pub fn bus_num(&self) -> u8 {
            ((self.val >> 16) & 0b11111111) as u8
        }

        pub fn reserved(&self) -> u8 {
            ((self.val >> 24) & 0b1111111) as u8
        }

        pub fn enable(&self) -> bool {
            (self.val >> 31) == 1
        }
    }

    /****************************************************************/
    //                     Other functions                          //
    /****************************************************************/

    pub unsafe fn read_config(bus: u8, dev: u8, fun: u8, reg: u8) -> u32 {
        let address = ConfigAddress::new(reg, fun, dev, bus, true);
        let mut config_port = Port::new(CONFIG_ADDRESS);
        let mut data_port = Port::new(CONFIG_DATA);
        config_port.write(address.val);
        data_port.read()
    }

    pub unsafe fn read_dev_header(bus: u8, dev: u8, fun: u8, device: &mut Dev) -> bool {
        for i in 0u8..(device.header.len() as u8) {
            device.header[i as usize] = read_config(bus, dev, fun, i)
        }
        device.option.vendorID == 0x0000 || device.option.vendorID == 0xFFFF || device.option.deviceID == 0xFFFF
    }

    pub unsafe fn print_dev(bus: u8, dev: u8, fun: u8, device: &Dev) {
        println!("bus=0x{:x} dev=0x{:x} fun=0x{:x} venID=0x{:x} devID=0x{:x} class={} subClass={} pib={}", bus, dev, fun, device.option.vendorID, device.option.deviceID, device.classname(), device.subclass(), device.pib());
    }

    pub unsafe fn check_bus(bus: u8, device: &mut Dev) {
        for dev in 0u8..MAX_DEVICES {
            check_fun(bus, dev, device);
        }
    }

    pub unsafe fn check_fun(bus: u8, dev: u8, device: &mut Dev) {
        let fun: u8 = 0;
        if read_dev_header(bus, dev, fun, device) {
            return;
        }
        check_device(bus, dev, fun, device);
        if device.option.headerType.has(HeaderType::MultiFunc) {
            for fun in 1..MAX_FUNCTIONS {
                if read_dev_header(bus, dev, fun, device) {
                    continue;
                }
                check_device(bus, dev, fun, device);
            }
        }
    }

    pub unsafe fn check_device(bus: u8, dev: u8, fun: u8, device: &Dev) {
        if device.option.classCode == 0x06 && device.option.subClassCode == 0x04 {
            //< Just skip secondary busses...
        }
        print_dev(bus, dev, fun, device); //< Now just print
    }

    pub unsafe fn scan() {
        let mut device = Dev::new();
        read_dev_header(0, 0, 0, &mut device);
        if !device.option.headerType.has(HeaderType::MultiFunc) {
            check_bus(0, &mut device);
        } else {
            for fun in 0..MAX_FUNCTIONS {
                if read_dev_header(0, 0, fun, &mut device) {
                    break;
                }
                check_bus(fun, &mut device);
            }
        }
    }

}

#[cfg(feature = "pci")]
pub use private::*;
