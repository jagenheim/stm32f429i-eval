use crate::hal::rcc::{Enable, Reset};
use stm32f4xx_hal::{
    gpio::{alt::fsmc as alt, PinSpeed, Speed},
    pac::{self, FMC},
};

// NOTE! There is a patch for the stm32f4xx-hal crate that implemented a
// stm32-fmc feature for external SDRAM access. Will need to rewrite this
// entire thing when that gets merged, as it will be much better than this.

pub struct SdramAddressPins {
    pub a0: alt::A0,
    pub a1: alt::A1,
    pub a2: alt::A2,
    pub a3: alt::A3,
    pub a4: alt::A4,
    pub a5: alt::A5,
    pub a6: alt::A6,
    pub a7: alt::A7,
    pub a8: alt::A8,
    pub a9: alt::A9,
    pub a10: alt::A10,
    pub a11: alt::A11,
    pub a12: alt::A12,
}

pub struct SdramDataPins {
    pub d0: alt::D0,
    pub d1: alt::D1,
    pub d2: alt::D2,
    pub d3: alt::D3,
    pub d4: alt::D4,
    pub d5: alt::D5,
    pub d6: alt::D6,
    pub d7: alt::D7,
    pub d8: alt::D8,
    pub d9: alt::D9,
    pub d10: alt::D10,
    pub d11: alt::D11,
    pub d12: alt::D12,
    pub d13: alt::D13,
    pub d14: alt::D14,
    pub d15: alt::D15,
    pub d16: alt::D16,
    pub d17: alt::D17,
    pub d18: alt::D18,
    pub d19: alt::D19,
    pub d20: alt::D20,
    pub d21: alt::D21,
    pub d22: alt::D22,
    pub d23: alt::D23,
    pub d24: alt::D24,
    pub d25: alt::D25,
    pub d26: alt::D26,
    pub d27: alt::D27,
    pub d28: alt::D28,
    pub d29: alt::D29,
    pub d30: alt::D30,
    pub d31: alt::D31,
}

pub struct SdramBankPins {
    pub ba0: alt::Ba0,
    pub ba1: alt::Ba1,
}

pub struct SdramControlPins {
    pub sdclk: alt::Sdclk,
    pub sdnwe: alt::Sdnwe,
    pub sdnras: alt::Sdnras,
    pub sdncas: alt::Sdncas,
    pub sdcke0: alt::Sdcke0,
    pub sdne0: alt::Sdne0,
}

pub struct SdramByteMaskPins {
    pub nbl0: alt::Nbl0,
    pub nbl1: alt::Nbl1,
    pub nbl2: alt::Nbl2,
    pub nbl3: alt::Nbl3,
}

impl SdramAddressPins {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        a0: impl Into<alt::A0>,
        a1: impl Into<alt::A1>,
        a2: impl Into<alt::A2>,
        a3: impl Into<alt::A3>,
        a4: impl Into<alt::A4>,
        a5: impl Into<alt::A5>,
        a6: impl Into<alt::A6>,
        a7: impl Into<alt::A7>,
        a8: impl Into<alt::A8>,
        a9: impl Into<alt::A9>,
        a10: impl Into<alt::A10>,
        a11: impl Into<alt::A11>,
        a12: impl Into<alt::A12>,
    ) -> Self {
        Self {
            a0: a0.into().speed(Speed::VeryHigh),
            a1: a1.into().speed(Speed::VeryHigh),
            a2: a2.into().speed(Speed::VeryHigh),
            a3: a3.into().speed(Speed::VeryHigh),
            a4: a4.into().speed(Speed::VeryHigh),
            a5: a5.into().speed(Speed::VeryHigh),
            a6: a6.into().speed(Speed::VeryHigh),
            a7: a7.into().speed(Speed::VeryHigh),
            a8: a8.into().speed(Speed::VeryHigh),
            a9: a9.into().speed(Speed::VeryHigh),
            a10: a10.into().speed(Speed::VeryHigh),
            a11: a11.into().speed(Speed::VeryHigh),
            a12: a12.into().speed(Speed::VeryHigh),
        }
    }
}

impl SdramDataPins {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        d0: impl Into<alt::D0>,
        d1: impl Into<alt::D1>,
        d2: impl Into<alt::D2>,
        d3: impl Into<alt::D3>,
        d4: impl Into<alt::D4>,
        d5: impl Into<alt::D5>,
        d6: impl Into<alt::D6>,
        d7: impl Into<alt::D7>,
        d8: impl Into<alt::D8>,
        d9: impl Into<alt::D9>,
        d10: impl Into<alt::D10>,
        d11: impl Into<alt::D11>,
        d12: impl Into<alt::D12>,
        d13: impl Into<alt::D13>,
        d14: impl Into<alt::D14>,
        d15: impl Into<alt::D15>,
        d16: impl Into<alt::D16>,
        d17: impl Into<alt::D17>,
        d18: impl Into<alt::D18>,
        d19: impl Into<alt::D19>,
        d20: impl Into<alt::D20>,
        d21: impl Into<alt::D21>,
        d22: impl Into<alt::D22>,
        d23: impl Into<alt::D23>,
        d24: impl Into<alt::D24>,
        d25: impl Into<alt::D25>,
        d26: impl Into<alt::D26>,
        d27: impl Into<alt::D27>,
        d28: impl Into<alt::D28>,
        d29: impl Into<alt::D29>,
        d30: impl Into<alt::D30>,
        d31: impl Into<alt::D31>,
    ) -> Self {
        Self {
            d0: d0.into().speed(Speed::VeryHigh),
            d1: d1.into().speed(Speed::VeryHigh),
            d2: d2.into().speed(Speed::VeryHigh),
            d3: d3.into().speed(Speed::VeryHigh),
            d4: d4.into().speed(Speed::VeryHigh),
            d5: d5.into().speed(Speed::VeryHigh),
            d6: d6.into().speed(Speed::VeryHigh),
            d7: d7.into().speed(Speed::VeryHigh),
            d8: d8.into().speed(Speed::VeryHigh),
            d9: d9.into().speed(Speed::VeryHigh),
            d10: d10.into().speed(Speed::VeryHigh),
            d11: d11.into().speed(Speed::VeryHigh),
            d12: d12.into().speed(Speed::VeryHigh),
            d13: d13.into().speed(Speed::VeryHigh),
            d14: d14.into().speed(Speed::VeryHigh),
            d15: d15.into().speed(Speed::VeryHigh),
            d16: d16.into().speed(Speed::VeryHigh),
            d17: d17.into().speed(Speed::VeryHigh),
            d18: d18.into().speed(Speed::VeryHigh),
            d19: d19.into().speed(Speed::VeryHigh),
            d20: d20.into().speed(Speed::VeryHigh),
            d21: d21.into().speed(Speed::VeryHigh),
            d22: d22.into().speed(Speed::VeryHigh),
            d23: d23.into().speed(Speed::VeryHigh),
            d24: d24.into().speed(Speed::VeryHigh),
            d25: d25.into().speed(Speed::VeryHigh),
            d26: d26.into().speed(Speed::VeryHigh),
            d27: d27.into().speed(Speed::VeryHigh),
            d28: d28.into().speed(Speed::VeryHigh),
            d29: d29.into().speed(Speed::VeryHigh),
            d30: d30.into().speed(Speed::VeryHigh),
            d31: d31.into().speed(Speed::VeryHigh),
        }
    }
}

impl SdramBankPins {
    pub fn new(ba0: impl Into<alt::Ba0>, ba1: impl Into<alt::Ba1>) -> Self {
        Self {
            ba0: ba0.into().speed(Speed::VeryHigh),
            ba1: ba1.into().speed(Speed::VeryHigh),
        }
    }
}

impl SdramControlPins {
    pub fn new(
        sdclk: impl Into<alt::Sdclk>,
        sdnwe: impl Into<alt::Sdnwe>,
        sdnras: impl Into<alt::Sdnras>,
        sdncas: impl Into<alt::Sdncas>,
        sdcke0: impl Into<alt::Sdcke0>,
        sdne0: impl Into<alt::Sdne0>,
    ) -> Self {
        Self {
            sdclk: sdclk.into().speed(Speed::VeryHigh),
            sdnwe: sdnwe.into().speed(Speed::VeryHigh),
            sdnras: sdnras.into().speed(Speed::VeryHigh),
            sdncas: sdncas.into().speed(Speed::VeryHigh),
            sdcke0: sdcke0.into().speed(Speed::VeryHigh),
            sdne0: sdne0.into().speed(Speed::VeryHigh),
        }
    }
}

impl SdramByteMaskPins {
    pub fn new(
        nbl0: impl Into<alt::Nbl0>,
        nbl1: impl Into<alt::Nbl1>,
        nbl2: impl Into<alt::Nbl2>,
        nbl3: impl Into<alt::Nbl3>,
    ) -> Self {
        Self {
            nbl0: nbl0.into().speed(Speed::VeryHigh),
            nbl1: nbl1.into().speed(Speed::VeryHigh),
            nbl2: nbl2.into().speed(Speed::VeryHigh),
            nbl3: nbl3.into().speed(Speed::VeryHigh),
        }
    }
}

#[allow(dead_code)]
pub struct SdramPins {
    data: SdramDataPins,
    addr: SdramAddressPins,
    bank: SdramBankPins,
    ctrl: SdramControlPins,
    mask: SdramByteMaskPins,
}

impl SdramPins {
    pub fn new(
        data: SdramDataPins,
        addr: SdramAddressPins,
        bank: SdramBankPins,
        ctrl: SdramControlPins,
        mask: SdramByteMaskPins,
    ) -> Self {
        Self {
            data,
            addr,
            bank,
            ctrl,
            mask,
        }
    }
}

pub fn sdram_init(_pins: SdramPins, fmc: &mut FMC) {
    unsafe {
        FMC::enable_unchecked();
        FMC::reset_unchecked();
    }

    // Secttion 37.7.3 of the reference manual describes the SDRAM initialization sequence

    // 1. Program features into the sdcr1 register
    unsafe {
        fmc.sdcr1().write(|w| {
            w.nc()
                .bits(1) // 9 bits
                .nr()
                .bits(1) // 12 bits
                .mwid()
                .bits(1) // 32-bit data bus
                .nb()
                .set_bit() // 4 internal banks
                .cas()
                .bits(3) // CAS latency of 3
                .wp()
                .clear_bit() // Write protection off
                .sdclk()
                .bits(2) // 2x HCLK
                .rburst()
                .set_bit() // Read burst enabled
                .rpipe()
                .bits(0) // No read pipe delay
        });
    }

    // 2. Program timings into the sdtr1 register
    fmc.sdtr1().write(|w| {
        w.tmrd()
            .set(2) // Load mode register to active
            .txsr()
            .set(7) // Exit self-refresh time
            .tras()
            .set(4) // Self-refresh time
            .trc()
            .set(7) // Row cycle delay
            .twr()
            .set(2) // Write recovery delay
            .trp()
            .set(2) // Row precharge delay
            .trcd()
            .set(2) // RAS to CAS delay
    });

    // Send SDRAM initialization sequence
    send_sdram_cmd(fmc, 0x01); // Clock configuration enable
    delay(100);
    send_sdram_cmd(fmc, 0x02); // Precharge all

    fmc.sdcmr().write(|w| w.nrfs().set(8)); // nrfs in sdcmr register is the number of auto-refresh commands to issue
    send_sdram_cmd(fmc, 0x03); // Auto refresh

    send_sdram_cmd(fmc, 0x04); // Load mode register

    // Set refresh rate
    fmc.sdrtr().write(|w| w.count().set(0x0569));
}

fn send_sdram_cmd(fmc: &mut pac::FMC, cmd: u8) {
    //
    unsafe {
        fmc.sdcmr().write(|w| w.mode().bits(cmd).ctb1().set_bit());
    }

    // Wait for the command to finish
    while fmc.sdsr().read().busy().bit_is_set() {}
}

fn delay(cycles: u32) {
    for _ in 0..cycles {
        cortex_m::asm::nop();
    }
}
