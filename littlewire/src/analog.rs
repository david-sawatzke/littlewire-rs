use crate::*;
use embedded_hal::adc as hal;
use littlewire_sys as sys;

// TODO We could probably use digital & analog at the same time

pub struct LittleWireAnalog(pub(crate) LittleWire);

impl LittleWireAnalog {
    pub fn base(self) -> LittleWire {
        self.0
    }
}

macro_rules! analog_pin {
    ($($PIN:ident: $channel:expr),+) => {
        $(
            pub struct $PIN {}
            impl hal::Channel<LittleWireAnalog> for $PIN {
                type ID = u8;
                fn channel() -> u8 {
                    $channel
                }
            }
        )+
    };
}

analog_pin!(AdcPin3: 0,
            AdcPin2: 1,
            AdcTempSens: 2);

impl<PIN> hal::OneShot<LittleWireAnalog, u16, PIN> for LittleWireAnalog
where
    PIN: hal::Channel<LittleWireAnalog, ID = u8>,
{
    type Error = LwError;
    /// A 10 bit value is returned
    /// This is a blocking operation
    fn read(&mut self, _pin: &mut PIN) -> nb::Result<u16, Self::Error> {
        let result = unsafe { sys::analogRead(self.0.dev, PIN::channel()) } as u16;
        check_error().map_err(|e| nb::Error::Other(e))?;
        Ok(result)
    }
}
