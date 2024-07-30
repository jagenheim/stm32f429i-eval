#![no_main]
#![no_std]

use panic_probe as _;

use stm32f429i_eval as board;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use defmt_rtt as _;

use board::hal::prelude::*;
use board::led::{Color, Leds};

use board::hal::pac;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(_)) = (pac::Peripherals::take(), Peripherals::take()) {
        // Constrain clock registers
        let rcc = p.RCC.constrain();
        // Configure clock to 180 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(180.MHz()).freeze();
        //let mut timer = p.TIM2.counter_ms(&clocks);
        let mut timer = p.TIM2.delay_us(&clocks);

        //get gpio G
        let gpiog = p.GPIOG.split();
        // Initialize on-board LEDs
        let mut leds = Leds::new(gpiog);

        leds[Color::Green].toggle();
        leds[Color::Orange].toggle();
        leds[Color::Red].toggle();
        leds[Color::Blue].toggle();

        // Endlessly blink the 4 LEDs every 500 ms
        loop {
            defmt::debug!("Swapping LEDs");
            timer.delay_ms(50_u32);
            leds[Color::Green].toggle();
            timer.delay_ms(50_u32);
            leds[Color::Orange].toggle();
            timer.delay_ms(50_u32);
            leds[Color::Red].toggle();
            timer.delay_ms(50_u32);
            leds[Color::Blue].toggle();
        }
    } else {
        defmt::error!("Failed to access peripherals");
    }

    loop {}
}
