/****************************************************************/
//                            Uses                              //
/****************************************************************/

use x86_64::instructions::port::Port;
use crate::println;

/****************************************************************/
//                         Constants                            //
/****************************************************************/

pub const CONFIG_PORT:   u16 = 0xCF8;
pub const DATA_PORT:     u16 = 0xCFC;

pub const MAX_BUSES:     u8  =   255;
pub const MAX_DEVICES:   u8  =    32;
pub const MAX_FUNCTIONS: u8  =     8;

/****************************************************************/
//                            Types                             //
/****************************************************************/

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HeaderType {
    Normal,
    Bridge,
    CardBus,
    MultiFunc = 0x80
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClassName {
    BeforePCI20, //< Before PCI 2.0
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
            ClassName::BeforePCI20 => "Before PCI 2.0",
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

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Options {
    pub vendorID:     u16,
    pub deviceID:     u16,
    pub commandReg:   u16,
    pub statusReg:    u16,
    pub revisionID:    u8,
    pub progIF:        u8,
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

pub unsafe fn read_config(bus: u8, dev: u8, func: u8, reg: u8) -> u32 {
    let address = ConfigAddress::new(reg, func, dev, bus, true);
    let mut config_port = Port::new(CONFIG_PORT);
    let mut data_port = Port::new(DATA_PORT);
    config_port.write(address.val);
    data_port.read()
}

pub unsafe fn get_dev_class_name(class_code: u8) -> Option <&'static str> {
    match ClassName::new(class_code) {
        Some(value) => Some(value.to_string()),
        None => None
    }
}

pub unsafe fn read_dev_header(bus: u8, dev: u8, func: u8, device: &mut Dev) -> bool {
    for i in 0u8..(device.header.len() as u8) {
        device.header[i as usize] = read_config(bus, dev, func, i)
    }
    device.option.vendorID == 0x0000 || device.option.vendorID == 0xFFFF || device.option.deviceID == 0xFFFF
}

pub unsafe fn print_dev(bus: u8, dev: u8, func: u8, device: &mut Dev) {
    println!("bus=0x{:x} dev=0x{:x} func=0x{:x} venID=0x{:x} devID=0x{:x} className={}", bus, dev, func, device.option.vendorID, device.option.deviceID, get_dev_class_name(device.option.classCode).expect("Unknown class code"));
}

pub unsafe fn scan() {
    let mut device = Dev::new();
    let mut func: u8;
    for bus in 0u8..MAX_BUSES {
        for dev in 0u8..MAX_DEVICES {
            func = 0;
            if read_dev_header(bus, dev, func, &mut device) {
                continue;
            }
            print_dev(bus, dev, func, &mut device);
            if ((device.option.headerType as u8) & (HeaderType::MultiFunc as u8)) != 0 {
                for func in 1u8..MAX_FUNCTIONS {
                    if read_dev_header(bus, dev, func, &mut device) {
                        continue;
                    }
                    print_dev(bus, dev, func, &mut device);
                }
            }
        }
    }
}
