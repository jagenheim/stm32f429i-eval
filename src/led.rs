//! On-board user LEDs

//use hal::prelude::*;

use hal::gpio::gpiog::{self, PGn, PG6, PG7, PG10, PG12};
use hal::gpio::{Output, PushPull};

/// We have 4 general purpose LEDs on the board

/// Green LED
pub type LD1 = PG6<Output<PushPull>>;
/// Orange LED
pub type LD2 = PG7<Output<PushPull>>;
/// Red LED
pub type LD3 = PG12<Output<PushPull>>;
/// Blue LED
pub type LD4 = PG10<Output<PushPull>>;

/// Led Colors. Each one matches one of the user LEDs.
pub enum Color {
    Blue,
    Orange,
    Green,
    Red,
}

// Array of the on-board user LEDs
pub struct Leds {
    leds: [Led; 4],
}

impl Leds {
    pub fn new(gpiog: gpiog::Parts) -> Self {
        let green = gpiog.pg6.into_push_pull_output();
        let orange = gpiog.pg7.into_push_pull_output();
        let blue = gpiog.pg10.into_push_pull_output();
        let red = gpiog.pg12.into_push_pull_output();
        Leds {
            leds: [red.into(), orange.into(), green.into(), blue.into()],
        }
    }
}

impl core::ops::Deref for Leds {
    type Target = [Led];

    fn deref(&self) -> &[Led] {
        &self.leds
    }
}

impl core::ops::DerefMut for Leds {
    fn deref_mut(&mut self) -> &mut [Led] {
        &mut self.leds
    }
}

impl core::ops::Index<usize> for Leds {
    type Output = Led;

    fn index(&self, i: usize) -> &Led {
        &self.leds[i]
    }
}

impl core::ops::Index<Color> for Leds {
    type Output = Led;

    fn index(&self, c: Color) -> &Led {
        &self.leds[c as usize]
    }
}

impl core::ops::IndexMut<usize> for Leds {
    fn index_mut(&mut self, i: usize) -> &mut Led {
        &mut self.leds[i]
    }
}

impl core::ops::IndexMut<Color> for Leds {
    fn index_mut(&mut self, c: Color) -> &mut Led {
        &mut self.leds[c as usize]
    }
}

/// One of the on-board user LEDs
pub struct Led {
    pin: PGn<Output<PushPull>>,
    on: bool,
}

macro_rules! ctor {
	($($ldx:ident),+) => {
		$(
			impl Into<Led> for $ldx {
				fn into(self) -> Led {
					Led {
						pin: self.erase_number(),
                        on: false,
					}
				}
			}
		)+
	}
}

ctor!(LD1, LD2, LD3, LD4);

impl Led {
    /// Turns the LED off
    pub fn off(&mut self) {
        self.pin.set_low();
        self.on = false;
    }

    /// Turns the LED on
    pub fn on(&mut self) {
        self.pin.set_high();
        self.on = true;
    }

    /// Toggles the LED
    pub fn toggle(&mut self) {
        if self.on {
            self.off();
        } else {
            self.on();
        }
    }
}


