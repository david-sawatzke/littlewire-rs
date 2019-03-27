use littlewire::prelude::*;
use littlewire::*;

fn main() {
    // Connect to littleWire
    let mut lw = connect().unwrap();
    let version = lw.read_firmware_version().unwrap();
    println!("Version: {}.{}", version.0, version.1);

    let mut delay = delay::Delay;
    let mut pin = lw.into_gpio().split().pin1.into_output().unwrap();
    loop {
        println!("Blink!");
        pin.set_high();
        delay.delay_ms(1000u16);
        println!("...");
        pin.set_low();
        delay.delay_ms(1000u16);
    }
}
