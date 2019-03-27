use littlewire::prelude::*;
use littlewire::*;
use smart_leds::*;

fn main() {
    // Connect to littleWire
    let mut lw = connect().unwrap();
    let version = lw.read_firmware_version().unwrap();
    println!("Version: {}.{}", version.0, version.1);

    let mut ws2812 = lw.into_gpio().split().pin1.into_output().unwrap();

    let mut delay = delay::Delay;
    const NUM_LEDS: usize = 10;
    let mut data = [Color::default().into(); NUM_LEDS];

    loop {
        for j in 0..(256 * 5) {
            for i in 0..NUM_LEDS {
                data[i] = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
            }
            ws2812.write(brightness(data.iter().cloned(), 32)).unwrap();
            delay.delay_ms(10u8);
        }
    }
}

/// Input a value 0 to 255 to get a color value
/// The colours are a transition r - g - b - back to r.
fn wheel(mut wheel_pos: u8) -> Color {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}
