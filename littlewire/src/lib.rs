use derive_more::Display;
use littlewire_sys as sys;
use std::error::Error;
use std::ffi::CStr;
use std::os::raw::c_char;

pub mod analog;
pub mod delay;
pub mod gpio;
pub mod ws2812;
pub mod prelude {
    pub use embedded_hal::prelude::*;
    // These aren't yet included in the latest embedded_hal release
    pub use embedded_hal::adc::OneShot as _lw_adc_OneShot;
    pub use smart_leds_trait::SmartLedsWrite;
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
    pub fn into_analog(self) -> analog::LittleWireAnalog {
        unsafe { sys::analog_init(self.dev, sys::VREF_VCC as u8) };
        analog::LittleWireAnalog(self)
    }
    /// Escape hatch if you need to access the c library directly
    pub fn get_ptr(&self) -> *mut sys::littleWire {
        self.dev
    }
}
