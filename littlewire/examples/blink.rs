use littlewire::prelude::*;
use littlewire::*;
use std::thread::sleep_ms;

fn main() {
    // Connect to littleWire
    let mut lw = connect().unwrap();
    let version = lw.read_firmware_version().unwrap();
    println!("Version: {}.{}", version.0, version.1);

    let mut pin = lw.into_gpio().split().pin1.into_output().unwrap();
    loop {
        println!("Blink!");
        pin.set_high();
        sleep_ms(1000);
        println!("...");
        pin.set_low();
        sleep_ms(1000);
    }
}
