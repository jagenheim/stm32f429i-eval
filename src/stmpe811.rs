// A quick driver hack for the STMPE811 IO expander with a resistive touch controller
// https://media.digikey.com/pdf/data%20sheets/st%20microelectronics%20pdfs/stmpe811.pdf
//
// This could probably be expanded and moved to a separate crate, but
// for now it's just a quick hack to get the touch controller working.

// Usage:
// let mut stmpe811 = stmpe811::Stmpe811::new(i2c, addr);

// Top left corner of the screen reports around 274x3797
// Bottom right corner of the screen reports around 3861x321

use core::marker::PhantomData;
use embedded_hal::blocking::i2c;

pub struct Stmpe811<I2C> {
    i2c: PhantomData<I2C>,
    addr: u8,
}

pub struct Stmpe811Sample {
    pub x: u16,
    pub y: u16,
    //pub z: u8,
}

impl<I2C, E> Stmpe811<I2C>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
    E: core::fmt::Debug,
{
    pub fn new(_i2c: &I2C, addr: u8) -> Self {
        Stmpe811 {
            i2c: PhantomData,
            addr,
        }
    }

    pub fn firmware_id(&mut self, i2c: &mut I2C) -> Result<u16, E> {
        let mut buf = [0u8; 2];

        // Magical number, should move to a const
        i2c.write_read(self.addr, &[0x00], &mut buf)?;
        let ret: u16 = (buf[0] as u16) << 8 | buf[1] as u16;
        Ok(ret)
    }

    pub fn touch_setup(&mut self, i2c: &mut I2C) -> Result<(), E> {
        // Disable the clock gating for TSC and ADC in the SYS_CFG2 register
        i2c.write(self.addr, &[0x04, 0x0f])?;

        // XY acquisition mode, 16 window tracking
        i2c.write(self.addr, &[0x40, 0x32])?;

        // Enable the touch detect interrupt, else the device won't wake up during hibernation
        i2c.write(self.addr, &[0x0A, 0x01])?;

        // Enable the clock gating
        i2c.write(self.addr, &[0x04, 0x0c])?;

        // Enable TSC
        i2c.write(self.addr, &[0x40, 0x33])?;

        Ok(())
    }

    /// Reads the FIFO size and returns the number of samples
    pub fn touch_samples_available(&mut self, i2c: &mut I2C) -> Result<u8, E> {
        let mut buf = [0u8; 1];

        // More magic numbers
        i2c.write_read(self.addr, &[0x4c], &mut buf)?;

        Ok(buf[0])
    }

    pub fn touch_get_sample(&mut self, i2c: &mut I2C) -> Result<Stmpe811Sample, E> {
        let mut buf = [0u8; 3];

        // More magic numbers
        i2c.write_read(self.addr, &[0xd7], &mut buf)?;

        let x = ((buf[0] as u16) << 4) | ((buf[1] as u16) >> 4);
        let y = ((buf[1] as u16 & 0x0f) << 8) | buf[2] as u16;
        //let z = buf[3];

        //Ok(Stmpe811Sample { x, y, z })
        Ok(Stmpe811Sample { x, y })
    }
}
