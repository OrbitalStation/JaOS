use crate::pci::PCIEnum;
use core::fmt::Debug;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClassName {
    Unclassified,
    DiskController,
    NetworkInterface,
    GraphicsAdapter,
    MultimediaController,
    MemoryController,
    BridgeDevice,
    CommunicationController,
    SystemDevice,
    InputDevice,
    DockingStation,
    CPU,
    SerialBus,
    WirelessController,
    IntelligentIOController, //< Intelligent I/O controller
    SatelliteController,
    EncryptionController,
    SignalProcessingController,
    ProprietaryDevice = 0xFF,

    Count = 18
}

impl PCIEnum for ClassName {
    fn new(from: u8) -> Option <ClassName> {
        if from > 17 && from != 0xFF {
            return None
        }
        unsafe { Some(*((&from as *const u8) as *const Self)) }
    }
}
