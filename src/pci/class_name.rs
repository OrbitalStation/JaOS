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

impl ClassName {
    pub fn new(class_code: u8) -> Option <ClassName> {
        if class_code > 17 && class_code != 0xFF {
            return None
        }
        unsafe { Some(*((&class_code as *const u8) as *const ClassName)) }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            ClassName::Unclassified => "Unclassified",
            ClassName::DiskController => "Disk controller",
            ClassName::NetworkInterface => "Network interface",
            ClassName::GraphicsAdapter => "Graphics adapter",
            ClassName::MultimediaController => "Multimedia controller",
            ClassName::MemoryController => "Memory controller",
            ClassName::BridgeDevice => "Bridge device",
            ClassName::CommunicationController => "Communication controller",
            ClassName::SystemDevice => "System device",
            ClassName::InputDevice => "Input device",
            ClassName::DockingStation => "Docking station",
            ClassName::CPU => "CPU",
            ClassName::SerialBus => "Serial bus",
            ClassName::WirelessController => "Wireless controller",
            ClassName::IntelligentIOController => "Intelligent I/O controller",
            ClassName::SatelliteController => "Satellite controller",
            ClassName::EncryptionController => "Encryption controller",
            ClassName::SignalProcessingController => "Signal processing controller",
            ClassName::ProprietaryDevice => "Proprietary device",
            _ => ""
        }
    }
}
