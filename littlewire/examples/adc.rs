use littlewire::prelude::*;
use littlewire::*;

fn main() {
    // Connect to littleWire
    let mut lw = connect().unwrap();
    let version = lw.read_firmware_version().unwrap();
    println!("Version: {}.{}", version.0, version.1);

    let mut delay = delay::Delay;
    let mut adc = lw.into_analog();
    loop {
        let measurement = adc.read(&mut analog::AdcPin3 {}).unwrap();
        println!("Measured {}", measurement);
        delay.delay_ms(1000u16);
    }
}
