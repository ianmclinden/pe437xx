//! Configure an addressable PE43701 device, set an initial attenuation, and run a sweep.
//!
//! ## Wiring:
//! PE43701(A0..A2) -> GND
//! PE43701(PS) -> VCC
//! PE43701(LE) -> PA3
//! PE43701(SCK) -> PA5
//! PE43701(SI) -> PA7
//!
//! ## Usage:
//! Runs on stm32f103c8t6 (blue pill)

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use dummy_pin::DummyPin;
use embedded_hal_bus::spi::ExclusiveDevice;
use panic_halt as _;
use stm32f1xx_hal::{
    pac::{self, SPI1},
    prelude::*,
    spi::{Mode, Phase, Polarity, Spi, SpiBitFormat},
};

use pe437xx::{spi::PE43701, Address, Attenuation};

pub const MODE_0: Mode = Mode {
    polarity: Polarity::IdleLow,
    phase: Phase::CaptureOnFirstTransition,
};

const PE_ADDR: Address = match Address::new(0x00) {
    Ok(addr) => addr,
    Err(_) => panic!("Invalid address"),
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    let mut delay = cp.SYST.delay(&rcc.clocks);

    let mut gpioa = dp.GPIOA.split(&mut rcc);

    let le = gpioa.pa3.into_push_pull_output(&mut gpioa.crl);
    let sck = gpioa.pa5;
    let mosi = gpioa.pa7;

    let mut spi_bus = Spi::new(
        dp.SPI1,
        (Some(sck), SPI1::NoMiso, Some(mosi)),
        MODE_0,
        10.MHz(),
        &mut rcc,
    );
    spi_bus.bit_format(SpiBitFormat::LsbFirst);
    let spi = ExclusiveDevice::new_no_delay(spi_bus, DummyPin::new_low()).unwrap();

    let mut pe43701 = PE43701::new(spi, le, PE_ADDR).unwrap();

    let mut attenuation = Attenuation::from_db(16.25).unwrap();
    loop {
        pe43701.set_attenuation(attenuation).unwrap();
        attenuation = attenuation.wrapping_add(Attenuation::quarter_db());
        delay.delay_ms(500u32);
    }
}
