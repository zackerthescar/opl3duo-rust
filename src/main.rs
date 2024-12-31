#![no_std]
#![no_main]

use core::any::Any;
use core::cell::RefCell;

use arduino_hal::prelude::*;
use arduino_hal::spi;
use panic_halt as _;
use embedded_sdmmc::{Error, Mode, VolumeIdx};
mod serial;
use serial::SerialWrapper;

#[arduino_hal::entry]
fn main() -> ! {
    // Initialize Arduino pins
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // Initialize devices
    let mut serial = SerialWrapper {serial: arduino_hal::default_serial!(dp, pins, 57600)};
    let (spi, cs) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings {
            data_order: spi::DataOrder::MostSignificantFirst,
            clock: spi::SerialClockRate::OscfOver128,
            mode: embedded_hal::spi::MODE_0, 
        },
    );
    let spi = RefCell::new(spi);
    let sdmmc_spi = embedded_hal_bus::spi::RefCellDevice::new(&spi, cs, arduino_hal::Delay::new()).unwrap();
    let sdcard = embedded_sdmmc::SdCard::new(sdmmc_spi, arduino_hal::Delay::new());
    serial.println("Init complete");
    ufmt::uwriteln!(&mut serial.serial, "Card is {} bytes", sdcard.num_bytes().unwrap()).unwrap_infallible();
    loop {
        arduino_hal::delay_ms(1000);
    }
}
