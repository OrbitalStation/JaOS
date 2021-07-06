#[cfg(feature = "pci")]
mod private {
    use crate::pci::private::*;
    use core::fmt::{Display, Formatter, Result, Write, Error};

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct EmptyPib;

    impl Display for EmptyPib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str("-")
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum DiskControllerIDEPib {
        OnlyISACompatible,
        OnlyNativePCI = 0x05,
        ISACompatibleWithNativePCI = 0x0A,
        NativePCIWithISACompatible = 0x0F,
        ISACompatibleWithBusMastering = 0x80,
        NativePCIWithBusMastering = 0x85,
        ISACompatibleWithNativePCIAndBusMastering = 0x8A,
        NativePCIWithISACompatibleAndBusMastering = 0x8F
    }

    impl Display for DiskControllerIDEPib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str(match self {
                Self::OnlyISACompatible => "ISA Compatibility mode-only",
                Self::OnlyNativePCI => "PCI Native mode-only",
                Self::ISACompatibleWithNativePCI => "ISA Compatibility mode, supports switch to PCI Native mode",
                Self::NativePCIWithISACompatible => "PCI Native mode, supports switch to ISA Compatibility mode",
                Self::ISACompatibleWithBusMastering => "ISA Compatibility mode-only, supports bus mastering",
                Self::NativePCIWithBusMastering => "PCI Native mode-only, supports bus mastering",
                Self::ISACompatibleWithNativePCIAndBusMastering => "ISA Compatibility mode, supports switch to PCI Native mode and bus mastering",
                Self::NativePCIWithISACompatibleAndBusMastering => "PCI Native mode, supports switch to ISA Compatibility mode and bus mastering"
            })
        }
    }

    impl PCIEnum for DiskControllerIDEPib {
        fn new(from: u8) -> Option <Self> {
            if from != 0x00 &&
               from != 0x05 &&
               from != 0x0A &&
               from != 0x0F &&
               from != 0x80 &&
               from != 0x85 &&
               from != 0x8A &&
               from != 0x8F {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum DiskControllerATAPib {
        SingleDMA = 0x20,
        ChainedDMA = 0x30
    }

    impl Display for DiskControllerATAPib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str(match self {
                Self::SingleDMA => "Single DMA",
                Self::ChainedDMA => "Chained DMA"
            })
        }
    }

    impl PCIEnum for DiskControllerATAPib {
        fn new(from: u8) -> Option <Self> {
            match from {
                0x20 => Some(Self::SingleDMA),
                0x30 => Some(Self::ChainedDMA),
                _ => None
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum DiskControllerSerialATAPib {
        VendorSpecific,
        AHCI10, //< AHCI 1.0
        SerialStorageBus
    }

    impl Display for DiskControllerSerialATAPib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str(match self {
                Self::VendorSpecific => "Vendor Specific Interface",
                Self::AHCI10 => "AHCI 1.0",
                Self::SerialStorageBus => "Serial Storage Bus"
            })
        }
    }

    impl PCIEnum for DiskControllerSerialATAPib {
        fn new(from: u8) -> Option <Self> {
            if from > 2 {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum DiskControllerSerialAttachedSCSIPib {
        SAS,
        SerialStorageBus
    }

    impl Display for DiskControllerSerialAttachedSCSIPib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str(match self {
                Self::SAS => "SAS",
                Self::SerialStorageBus => "Serial Storage Bus"
            })
        }
    }

    impl PCIEnum for DiskControllerSerialAttachedSCSIPib {
        fn new(from: u8) -> Option <Self> {
            if from > 1 {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum DiskControllerNonVolatileMemoryPib {
        NVMHCI = 1,
        NVMExpress
    }

    impl Display for DiskControllerNonVolatileMemoryPib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str(match self {
                Self::NVMHCI => "NVMHCI",
                Self::NVMExpress => "NVM Express"
            })
        }
    }

    impl PCIEnum for DiskControllerNonVolatileMemoryPib {
        fn new(from: u8) -> Option <Self> {
            if from != 1 && from != 2 {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum GraphicsAdapterVGACompatiblePib {
        VGA,
        Compatible8514 //< 8514-Compatible
    }

    impl Display for GraphicsAdapterVGACompatiblePib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str(match self {
                Self::VGA => "VGA",
                Self::Compatible8514 => "8514-Compatible"
            })
        }
    }

    impl PCIEnum for GraphicsAdapterVGACompatiblePib {
        fn new(from: u8) -> Option <Self> {
            if from > 1 {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum BridgeDevicePCI2PCIPib {
        Normal,
        Subtractive
    }

    impl Display for BridgeDevicePCI2PCIPib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str(match self {
                Self::Normal => "Normal Decode",
                Self::Subtractive => "Subtractive Decode"
            })
        }
    }

    impl PCIEnum for BridgeDevicePCI2PCIPib {
        fn new(from: u8) -> Option <Self> {
            if from > 1 {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum BridgeDeviceRACEwayPib {
        Transparent,
        Endpoint
    }

    impl Display for BridgeDeviceRACEwayPib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str(match self {
                Self::Transparent => "Transparent mode",
                Self::Endpoint => "Endpoint mode"
            })
        }
    }

    impl PCIEnum for BridgeDeviceRACEwayPib {
        fn new(from: u8) -> Option <Self> {
            if from > 1 {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum BridgeDevicePCI2PCI2Pib {
        Primary = 0x40,
        Secondary = 0x80
    }

    impl Display for BridgeDevicePCI2PCI2Pib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str(match self {
                Self::Primary => "Semi-Transparent, Primary bus towards host CPU",
                Self::Secondary => "Semi-Transparent, Secondary bus towards host CPU"
            })
        }
    }

    impl PCIEnum for BridgeDevicePCI2PCI2Pib {
        fn new(from: u8) -> Option <Self> {
            if from != 0x40 && from != 0x80 {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum CommunicationControllerSerialPib {
        Compatible8250, //< Generic XT
        Compatible16450,
        Compatible16550,
        Compatible16650,
        Compatible16750,
        Compatible16850,
        Compatible16950
    }

    impl Display for CommunicationControllerSerialPib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str(match self {
                Self::Compatible8250 => "8250-Compatible (Generic XT)",
                Self::Compatible16450 => "16450-Compatible",
                Self::Compatible16550 => "16550-Compatible",
                Self::Compatible16650 => "16650-Compatible",
                Self::Compatible16750 => "16750-Compatible",
                Self::Compatible16850 => "16850-Compatible",
                Self::Compatible16950 => "16950-Compatible"
            })
        }
    }

    impl PCIEnum for CommunicationControllerSerialPib {
        fn new(from: u8) -> Option <Self> {
            if from > 6 {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum CommunicationControllerParallelPib {
        Standard,
        BiDirectional,
        ECP1XCompliant, //< ECP 1.X Compliant
        IEEE1284Controller,
        IEEE1284TargetDevice = 0xFE
    }

    impl Display for CommunicationControllerParallelPib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str(match self {
                Self::Standard => "Standard Parallel Port",
                Self::BiDirectional => "Bi-Directional Parallel Port",
                Self::ECP1XCompliant => "ECP 1.X Compliant Parallel Port",
                Self::IEEE1284Controller => "IEEE 1284 Controller",
                Self::IEEE1284TargetDevice => "IEEE 1284 Target Device"
            })
        }
    }

    impl PCIEnum for CommunicationControllerParallelPib {
        fn new(from: u8) -> Option <Self> {
            if from > 3 && from != 0xFE {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum CommunicationControllerModemPib {
        Generic,
        Hayes16450Compatible,
        Hayes16550Compatible,
        Hayes16650Compatible,
        Hayes16750Compatible
    }

    impl Display for CommunicationControllerModemPib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Self::Generic => f.write_str("Generic Modem"),
                hayes => {
                    if f.write_str("Hayes 16").is_err() {
                        return Err(Error)
                    }
                    unsafe {
                        if f.write_char((*((hayes as *const Self) as *const u8) + 51) as char).is_err() {
                            return Err(Error)
                        }
                    }
                    f.write_str("50-Compatible Interface")
                }
            }
        }
    }

    impl PCIEnum for CommunicationControllerModemPib {
        fn new(from: u8) -> Option <Self> {
            if from > 4 {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum SystemDevicePICPib {
        Generic8259Compatible,
        ISACompatible,
        EISACompatible,
        IOAPICInterruptController = 0x10, //< I/O APIC Interrupt Controller
        IOxAPICInterruptController = 0x20 //< I/O(x) APIC Interrupt Controller
    }

    impl Display for SystemDevicePICPib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Self::Generic8259Compatible => f.write_str("Generic 8259-Compatible"),
                Self::ISACompatible => f.write_str("ISA-Compatible"),
                Self::EISACompatible => f.write_str("EISA-Compatible"),
                io => {
                    if f.write_str("I/O").is_err() {
                        return Err(Error)
                    }
                    if *io == Self::IOxAPICInterruptController {
                        if f.write_str("(x)").is_err() {
                            return Err(Error)
                        }
                    }
                    f.write_str(" APIC Interrupt Controller")
                }
            }
        }
    }

    impl PCIEnum for SystemDevicePICPib {
        fn new(from: u8) -> Option <Self> {
            if from > 2 && from != 0x10 && from != 0x20 {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum SystemDeviceDMAPib {
        Generic8237Compatible,
        ISACompatible,
        EISACompatible
    }

    impl Display for SystemDeviceDMAPib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Self::Generic8237Compatible => f.write_str("Generic 8237-Compatible"),
                isa => {
                    if *isa == Self::EISACompatible {
                        if f.write_char('E').is_err() {
                            return Err(Error)
                        }
                    }
                    f.write_str("ISA-Compatible")
                }
            }
        }
    }

    impl PCIEnum for SystemDeviceDMAPib {
        fn new(from: u8) -> Option <Self> {
            if from > 2 {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum SystemDeviceTimerPib {
        Generic8254Compatible,
        ISACompatible,
        EISACompatible,
        HPET
    }

    impl Display for SystemDeviceTimerPib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Self::Generic8254Compatible => f.write_str("Generic 8254-Compatible"),
                Self::HPET => f.write_str("HPET"),
                isa => {
                    if *isa == Self::EISACompatible {
                        if f.write_char('E').is_err() {
                            return Err(Error)
                        }
                    }
                    f.write_str("ISA-Compatible")
                }
            }
        }
    }

    impl PCIEnum for SystemDeviceTimerPib {
        fn new(from: u8) -> Option <Self> {
            if from > 3 {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum SystemDeviceRTCPib {
        Generic,
        ISACompatible
    }

    impl Display for SystemDeviceRTCPib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str(match self {
                Self::Generic => "Generic",
                Self::ISACompatible => "ISA-Compatible"
            })
        }
    }

    impl PCIEnum for SystemDeviceRTCPib {
        fn new(from: u8) -> Option <Self> {
            if from > 1 {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum InputDeviceGameportPib {
        Generic,
        Extended = 0x10
    }

    impl Display for InputDeviceGameportPib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str(match self {
                Self::Generic => "Generic",
                Self::Extended => "Extended"
            })
        }
    }

    impl PCIEnum for InputDeviceGameportPib {
        fn new(from: u8) -> Option <Self> {
            if from != 0 && from != 0x10 {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum SerialBusFireWireIEEE1394Pib {
        Generic,
        OHCI = 0x10
    }

    impl Display for SerialBusFireWireIEEE1394Pib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str(match self {
                Self::Generic => "Generic",
                Self::OHCI => "OHCI"
            })
        }
    }

    impl PCIEnum for SerialBusFireWireIEEE1394Pib {
        fn new(from: u8) -> Option <Self> {
            if from != 0 && from != 0x10 {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum SerialBusUSBPib {
        UHCI,
        OHCI = 0x10,
        EHCI = 0x20,
        XHCI = 0x30,
        Unspecified = 0x80,
        USBDevice = 0xFE
    }

    impl Display for SerialBusUSBPib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Self::Unspecified => f.write_str("Unspecified"),
                Self::USBDevice => f.write_str("USB Device (Not a host controller)"),
                hci => {
                    if f.write_char(match hci {
                        Self::UHCI => 'U',
                        Self::OHCI => 'O',
                        Self::EHCI => 'E',
                        Self::XHCI => 'X',
                        _ => '_'
                    }).is_err() {
                        return Err(Error)
                    }
                    f.write_str("HCI")
                }
            }
        }
    }

    impl PCIEnum for SerialBusUSBPib {
        fn new(from: u8) -> Option <Self> {
            if from != 0x00 && from != 0x10 && from != 0x20 && from != 0x30 && from != 0x80 && from != 0xFE {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum SerialBusIPMIPib {
        SMIC,
        KeyboardControllerStyle,
        BlockTransfer
    }

    impl Display for SerialBusIPMIPib {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str(match self {
                Self::SMIC => "SMIC",
                Self::KeyboardControllerStyle => "Keyboard Controller Style",
                Self::BlockTransfer => "Block Transfer"
            })
        }
    }

    impl PCIEnum for SerialBusIPMIPib {
        fn new(from: u8) -> Option <Self> {
            if from > 2 {
                return None
            }
            unsafe { Some(*((&from as *const u8) as *const Self)) }
        }
    }

}

#[cfg(feature = "pci")]
pub use private::*;
