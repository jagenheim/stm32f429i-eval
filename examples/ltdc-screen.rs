#![no_main]
#![no_std]

// This is adapted from the stm32f4xx-hal crate screen example

use cortex_m_rt::entry;
use defmt_rtt as _;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    pixelcolor::{Rgb565, RgbColor},
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
    text::Text,
};
use panic_probe as _;
use stm32f429i_eval::screen::Stm32F4EvalDisplay as screen;
use stm32f429i_eval::sdram::{
    self, SdramAddressPins, SdramBankPins, SdramByteMaskPins, SdramControlPins, SdramDataPins,
    SdramPins,
};
use stm32f4xx_hal::{
    ltdc::{BluePins, GreenPins, Layer, LtdcPins, PixelFormat, RedPins},
    pac,
    prelude::*,
};

// DIMENSIONS
const WIDTH: u16 = 480;
const HEIGHT: u16 = 272;

// Graphics framebuffer
const FB_GRAPHICS_SIZE: usize = (WIDTH as usize) * (HEIGHT as usize);
#[link_section = ".sdram"]
static mut FB_LAYER1: [u16; FB_GRAPHICS_SIZE] = [0; FB_GRAPHICS_SIZE];

#[entry]
fn main() -> ! {
    let perif = pac::Peripherals::take().unwrap();
    let _cp = cortex_m::Peripherals::take().unwrap();

    let _gpiob = perif.GPIOB.split();
    let _gpioc = perif.GPIOC.split();
    let gpiod = perif.GPIOD.split();
    let gpioe = perif.GPIOE.split();
    let gpiof = perif.GPIOF.split();
    let gpiog = perif.GPIOG.split();
    let gpioh = perif.GPIOH.split();
    let gpioi = perif.GPIOI.split();
    let gpioj = perif.GPIOJ.split();
    let gpiok = perif.GPIOK.split();
    let rcc_hal = perif.RCC.constrain();
    let mut fmc = perif.FMC;

    let pins = SdramPins::new(
        SdramDataPins::new(
            gpiod.pd14, gpiod.pd15, gpiod.pd0, gpiod.pd1, gpioe.pe7, gpioe.pe8, gpioe.pe9,
            gpioe.pe10, gpioe.pe11, gpioe.pe12, gpioe.pe13, gpioe.pe14, gpioe.pe15, gpiod.pd8,
            gpiod.pd9, gpiod.pd10, gpioh.ph8, gpioh.ph9, gpioh.ph10, gpioh.ph11, gpioh.ph12,
            gpioh.ph13, gpioh.ph14, gpioh.ph15, gpioi.pi0, gpioi.pi1, gpioi.pi2, gpioi.pi3,
            gpioi.pi6, gpioi.pi7, gpioi.pi9, gpioi.pi10,
        ),
        SdramAddressPins::new(
            gpiof.pf0, gpiof.pf1, gpiof.pf2, gpiof.pf3, gpiof.pf4, gpiof.pf5, gpiof.pf12,
            gpiof.pf13, gpiof.pf14, gpiof.pf15, gpiog.pg0, gpiog.pg1, gpiog.pg2,
        ),
        SdramBankPins::new(gpiog.pg4, gpiog.pg5),
        SdramControlPins::new(
            gpiog.pg8, gpioh.ph5, gpiof.pf11, gpiog.pg15, gpioh.ph2, gpioh.ph3,
        ),
        SdramByteMaskPins::new(gpioe.pe0, gpioe.pe1, gpioi.pi4, gpioi.pi5),
    );

    // Setup SDRAM asap
    sdram::sdram_init(pins, &mut fmc);

    let pins = LtdcPins::new(
        RedPins::new(
            gpioi.pi15, gpioj.pj0, gpioj.pj1, gpioj.pj2, gpioj.pj3, gpioj.pj4, gpioj.pj5, gpioj.pj6,
        ),
        GreenPins::new(
            gpioj.pj7, gpioj.pj8, gpioj.pj9, gpioj.pj10, gpioj.pj11, gpiok.pk0, gpiok.pk1,
            gpiok.pk2,
        ),
        BluePins::new(
            gpioe.pe4, gpioj.pj13, gpioj.pj14, gpioj.pj15, gpiog.pg12, gpiok.pk4, gpiok.pk5,
            gpiok.pk6,
        ),
        gpioi.pi12, // hsync
        gpioi.pi13, // vsync
        gpiok.pk7,  // de
        gpioi.pi14, // clk
    );

    // HSE osc out in High Z
    gpioh.ph1.into_floating_input();
    let _clocks = rcc_hal.cfgr.sysclk(180.MHz()).hclk(180.MHz()).freeze();

    // LCD backlight enable
    // To enable this, we need to desolder R279 and replace R283 with a 0 ohm resistor
    // let mut backlight = gpioa.pa8.into_push_pull_output();
    // backlight.set_high();

    let mut display = screen::new(perif.LTDC, perif.DMA2D, Some(pins));
    display
        .controller
        .config_layer(Layer::L1, unsafe { &mut FB_LAYER1 }, PixelFormat::RGB565);

    display.controller.enable_layer(Layer::L1);
    display.controller.reload();

    let display = &mut display;

    Rectangle::new(Point::new(0, 0), Size::new(479, 271))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::new(0, 0b11110, 0b11011)))
        .draw(display)
        .ok();

    let c1 = Circle::new(Point::new(20, 40), 2 * 8)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::new(0, 63, 0)));

    let c2 = Circle::new(Point::new(50, 70), 2 * 8)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::new(31, 0, 0)));

    let t = Text::new(
        "Hello Rust!",
        Point::new(100, 100),
        MonoTextStyle::new(&FONT_6X9, RgbColor::WHITE),
    );

    c1.draw(display).ok();
    c2.draw(display).ok();
    t.draw(display).ok();

    for i in 0..300 {
        Circle::new(Point::new(20 + i, 20), 2 * 8)
            .into_styled(PrimitiveStyle::with_fill(RgbColor::GREEN))
            .draw(display)
            .ok();
    }

    #[allow(clippy::empty_loop)]
    loop {}
}
