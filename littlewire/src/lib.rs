use derive_more::Display;
use littlewire_sys as sys;
use std::error::Error;
use std::ffi::CStr;
use std::os::raw::c_char;

pub mod gpio;
pub mod prelude {
    pub use embedded_hal::prelude::*;
}

#[derive(Debug, Display)]
#[display(fmt = "Status {}, {}", status_code, error)]
pub struct LwError {
    pub error: String,
    pub status_code: i32,
}

impl Error for LwError {}

fn check_error() -> Result<(), LwError> {
    if unsafe { sys::lwStatus < 0 } {
        Err(LwError {
            status_code: unsafe { sys::lwStatus },
            error: unsafe {
                CStr::from_ptr(sys::littleWire_errorName() as *mut c_char)
                    .to_string_lossy()
                    .to_string()
            },
        })
    } else {
        Ok(())
    }
}

// We heavilly relly on lw *not* being used by two functions at the same time
// As this (and all other structs) can't be shared over threads, this is currently guaranteed
pub struct LittleWire {
    dev: *mut sys::littleWire,
}

pub fn connect() -> Option<LittleWire> {
    // Not sure if we can get multiple references to the same device
    let dev = unsafe { sys::littleWire_connect() };
    if dev.is_null() {
        None
    } else {
        Some(LittleWire { dev })
    }
}

impl LittleWire {
    pub fn read_firmware_version(&mut self) -> Result<(u8, u8), LwError> {
        let version = unsafe { sys::readFirmwareVersion(self.dev) };
        check_error()?;
        Ok(((version & 0xF0) >> 4, version & 0x0F))
    }
    pub fn into_gpio(self) -> gpio::LittleWireGpio {
        gpio::LittleWireGpio(self)
    }
}
