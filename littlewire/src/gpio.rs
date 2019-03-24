use crate::*;
use embedded_hal::digital as hal;
use littlewire_sys as sys;

pub struct LittleWireGpio(pub(crate) LittleWire);

pub struct Pin {
    dev: *mut sys::littleWire,
    num: u8,
}

pub struct InputPin(Pin);
// Just save the output pin state for the stateful traits
// We're not hurting for ram anyway
pub struct OutputPin(Pin, bool);

pub struct Pins {
    pub pin1: Pin,
    pub pin2: Pin,
    pub pin3: Pin,
    pub pin4: Pin,
}

impl LittleWireGpio {
    pub fn base(self) -> LittleWire {
        self.0
    }

    pub fn split(self) -> Pins {
        Pins {
            pin1: Pin {
                dev: self.0.dev,
                num: sys::PIN1 as u8,
            },
            pin2: Pin {
                dev: self.0.dev,
                num: sys::PIN2 as u8,
            },
            pin3: Pin {
                dev: self.0.dev,
                num: sys::PIN3 as u8,
            },
            pin4: Pin {
                dev: self.0.dev,
                num: sys::PIN4 as u8,
            },
        }
    }
}

impl Pins {
    pub fn unsplit(self) -> LittleWireGpio {
        LittleWireGpio(LittleWire { dev: self.pin1.dev })
    }
}

impl Pin {
    pub fn into_input(self) -> Result<InputPin, LwError> {
        unsafe { sys::pinMode(self.dev, self.num, sys::INPUT as u8) };
        check_error()?;
        Ok(InputPin(self))
    }
    pub fn into_output(self) -> Result<OutputPin, LwError> {
        unsafe { sys::pinMode(self.dev, self.num, sys::OUTPUT as u8) };
        let state = unsafe { sys::digitalRead(self.dev, self.num) == 1 };
        check_error()?;
        Ok(OutputPin(self, state))
    }
}

impl InputPin {
    pub fn into_unconfigured(self) -> Pin {
        self.0
    }
    pub fn set_pullup(&mut self, pullup: bool) -> Result<(), LwError> {
        unsafe { sys::internalPullup(self.0.dev, self.0.num, pullup as u8) }
        check_error()?;
        Ok(())
    }
}

impl hal::InputPin for InputPin {
    fn is_high(&self) -> bool {
        let state = unsafe { sys::digitalRead(self.0.dev, self.0.num) == 1 };
        check_error().unwrap();
        state
    }
    fn is_low(&self) -> bool {
        !self.is_high()
    }
}

impl OutputPin {
    pub fn into_unconfigured(self) -> Pin {
        self.0
    }
}

impl hal::OutputPin for OutputPin {
    fn set_low(&mut self) {
        self.1 = false;
        unsafe { sys::digitalWrite(self.0.dev, self.0.num, 0) }
        check_error().unwrap();
    }
    fn set_high(&mut self) {
        self.1 = true;
        unsafe { sys::digitalWrite(self.0.dev, self.0.num, 1) }
        check_error().unwrap();
    }
}

impl hal::StatefulOutputPin for OutputPin {
    fn is_set_high(&self) -> bool {
        self.1
    }
    fn is_set_low(&self) -> bool {
        !self.1
    }
}

impl hal::toggleable::Default for OutputPin {}
