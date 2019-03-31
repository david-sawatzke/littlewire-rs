use crate::*;
use embedded_hal as hal;
use littlewire_sys as sys;
use nb;

pub struct LittleWireSpi(pub(crate) LittleWire, pub(crate) Option<u8>);

impl hal::spi::FullDuplex<u8> for LittleWireSpi {
    type Error = LwError;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        match self.1 {
            Some(data) => {
                self.1 = None;
                Ok(data)
            }
            None => Err(nb::Error::WouldBlock),
        }
    }

    fn send(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        let mut data = vec![byte];
        let mut receive = vec![0x00];
        unsafe { sys::spi_sendMessage(self.0.dev, data.as_mut_ptr(), receive.as_mut_ptr(), 1, 0) };
        check_error().map_err(|e| nb::Error::Other(e))?;
        self.1 = Some(receive[0]);
        Ok(())
    }
}

// TODO implement this more efficiently
impl hal::blocking::spi::transfer::Default<u8> for LittleWireSpi {}
impl hal::blocking::spi::write::Default<u8> for LittleWireSpi {}
