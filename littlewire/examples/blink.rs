use littlewire_sys::*;
use std::ffi::CString;
use std::os::raw::c_char;
use std::thread::sleep_ms;

fn main() {
    let lw_count = unsafe { littlewire_search() };
    if lw_count == 0 {
        panic!("Couldn't find a littleWire device");
    }
    for i in 0..lw_count as usize {
        let sn = unsafe { lwResults[i].serialNumber };
        println!("Found device with serial number {}", sn);
    }
    // Connect to first device
    let lw = unsafe { littleWire_connect() };
    if lw.is_null() {
        panic!("Couldn't connect");
    }
    let version = unsafe { readFirmwareVersion(lw) };
    println!("Version: {}.{}", (version & 0xF0) >> 4, version & 0x0F);
    let pin = PIN1 as u8;
    unsafe { pinMode(lw, pin, OUTPUT as u8) };
    loop {
        println!("Blink!");
        unsafe { digitalWrite(lw, pin, HIGH as u8) };
        sleep_ms(1000);
        println!("...");
        unsafe { digitalWrite(lw, pin, LOW as u8) };
        sleep_ms(1000);
        if unsafe { lwStatus < 0 } {
            panic!("Connection error {} {}", unsafe { lwStatus }, unsafe {
                CString::from_raw(littleWire_errorName() as *mut c_char)
                    .into_string()
                    .unwrap()
            });
        }
    }
}
