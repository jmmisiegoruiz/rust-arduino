// main.rs

#![no_std]
#![no_main]

extern crate panic_halt;
use shared::stutter_blink; // for write!

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        peripherals.PORTB,
        peripherals.PORTC,
        peripherals.PORTD,
    );

    let mut led = pins.d13.into_output(&mut pins.ddr);

    loop {
        stutter_blink(&mut led, 25);
    }
}