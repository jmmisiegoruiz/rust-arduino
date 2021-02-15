// main.rs

#![no_std]
#![no_main]

extern crate panic_halt;

use hd44780_driver::{HD44780, DisplayMode, Cursor, CursorBlink, Display};
use avr_delay::delay_us;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use arduino_uno::delay_ms;

struct Delay;

impl DelayUs<u16> for Delay {
    fn delay_us(&mut self, us: u16) {
        delay_us(us as u32);
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        delay_ms(ms as u16)
    }
}

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        peripherals.PORTB,
        peripherals.PORTC,
        peripherals.PORTD,
    );

    let mut delay = Delay{};

    let mut lcd = HD44780::new_4bit(
        pins.d11.into_output(&mut pins.ddr), // Register Select pin
        pins.d10.into_output(&mut pins.ddr), // Enable pin

        pins.d6.into_output(&mut pins.ddr),  // d4
        pins.d5.into_output(&mut pins.ddr), // d5
        pins.d4.into_output(&mut pins.ddr), // d6
        pins.d3.into_output(&mut pins.ddr), // d7
        &mut delay,
    ).unwrap();

    let display_mode = DisplayMode {
        cursor_visibility: Cursor::Invisible,
        cursor_blink: CursorBlink::Off,
        display: Display::On
    };
    lcd.set_display_mode(display_mode, &mut delay);

    // Unshift display and set cursor to 0
    lcd.reset(&mut delay);

    // Clear existing characters
    lcd.clear(&mut delay);

    // Display the following string
    lcd.write_str("Hello, world!", &mut delay);

    // Move the cursor to the second line
    lcd.set_cursor_pos(40, &mut delay);

    // Display the following string on the second line
    lcd.write_str("I'm on line 2!", &mut delay);

    loop{}
}