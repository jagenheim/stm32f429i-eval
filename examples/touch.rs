#![no_main]
#![no_std]

use stm32f429i_eval as board;

use board::hal::{i2c::*, pac, prelude::*, rcc::Rcc};
use board::stmpe811::Stmpe811;
use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let _cp = cortex_m::Peripherals::take().unwrap();

    let rcc: Rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(180.MHz()).freeze();

    let gpiob = p.GPIOB.split();

    let scl = gpiob.pb6.internal_pull_up(true);
    let sda = gpiob.pb9.internal_pull_up(true);

    let mut i2c = I2c::new(p.I2C1, (scl, sda), 400.kHz(), &clocks);

    let mut touch = Stmpe811::new(&i2c, 0x41);

    // Get the firmware id
    let firmware_id = touch.firmware_id(&mut i2c).unwrap();

    defmt::info!("Firmware ID: 0x{:04x}", firmware_id);

    // Make sure we have the right chip
    assert_eq!(firmware_id, 0x0811);

    touch.touch_setup(&mut i2c).unwrap();

    loop {
        while touch.touch_samples_available(&mut i2c).unwrap() > 0 {
            let t = touch.touch_get_sample(&mut i2c).unwrap();
            //defmt::info!("Touch: {}x{} {} ", t.x, t.y, t.z);
            defmt::info!("Touch: {}x{}", t.x, t.y);
        }
    }
}
