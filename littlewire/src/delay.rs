use embedded_hal as hal;
use std::thread;
use std::time::Duration;

/// Empty struct that provides delay functionality on top of `thread::sleep`
pub struct Delay;

impl hal::blocking::delay::DelayUs<u8> for Delay {
    fn delay_us(&mut self, n: u8) {
        thread::sleep(Duration::new(0, n as u32 * 1000))
    }
}

impl hal::blocking::delay::DelayUs<u16> for Delay {
    fn delay_us(&mut self, n: u16) {
        thread::sleep(Duration::new(0, n as u32 * 1000))
    }
}

impl hal::blocking::delay::DelayUs<u32> for Delay {
    fn delay_us(&mut self, n: u32) {
        let secs = n / 1_000_000;
        let nsecs = (n % 1_000_000) * 1_000;

        thread::sleep(Duration::new(secs as u64, nsecs))
    }
}

impl hal::blocking::delay::DelayUs<u64> for Delay {
    fn delay_us(&mut self, n: u64) {
        let secs = n / 1_000_000;
        let nsecs = ((n % 1_000_000) * 1_000) as u32;

        thread::sleep(Duration::new(secs, nsecs))
    }
}

impl hal::blocking::delay::DelayMs<u8> for Delay {
    fn delay_ms(&mut self, n: u8) {
        thread::sleep(Duration::from_millis(n as u64))
    }
}

impl hal::blocking::delay::DelayMs<u16> for Delay {
    fn delay_ms(&mut self, n: u16) {
        thread::sleep(Duration::from_millis(n as u64))
    }
}

impl hal::blocking::delay::DelayMs<u32> for Delay {
    fn delay_ms(&mut self, n: u32) {
        thread::sleep(Duration::from_millis(n as u64))
    }
}

impl hal::blocking::delay::DelayMs<u64> for Delay {
    fn delay_ms(&mut self, n: u64) {
        thread::sleep(Duration::from_millis(n))
    }
}
