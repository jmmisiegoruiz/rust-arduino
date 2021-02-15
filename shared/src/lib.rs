#![no_std]
use arduino_uno::prelude::*;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::hal::port::mode::Output;

pub fn stutter_blink(led: &mut PB5<Output>, times: usize) {
    (0..times).map(|i| i * 10).for_each(|i| {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(i as u16);
    });
}

pub fn extract_bit(data: u8, bit_index: usize) -> bool {
    data >> bit_index & 1 > 0
}

mod tests {
    #![allow(unused_imports)]
    use crate::extract_bit;

    #[test]
    fn test_extract_bit() {
        let mut data:u8 = 0b00000000;
        assert_eq!(extract_bit(data, 0), false);

        data = 0b00000001;
        assert_eq!(extract_bit(data, 0), true);

        data = 0b10000000;
        assert_eq!(extract_bit(data, 7), true);
    }
}
