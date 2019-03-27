use crate::*;
use littlewire_sys as sys;
use smart_leds_trait::*;

impl SmartLedsWrite for gpio::OutputPin {
    type Error = LwError;
    fn write<T>(&mut self, iterator: T) -> Result<(), LwError>
    where
        T: Iterator<Item = Color>,
    {
        for (num, item) in iterator.enumerate() {
            unsafe { sys::ws2812_preload(self.0.dev, item.r, item.g, item.b) };
            if num > 63 {
                panic!("Too many leds");
            }
        }
        unsafe { sys::ws2812_flush(self.0.dev, self.0.num) };
        Ok(())
    }
}
