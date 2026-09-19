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

use dummy_pin::DummyPin;
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts, dma,
    gpio::{Level, Output, Speed},
    peripherals,
    spi::{Config as SpiConfig, Spi, MODE_0},
    time::Hertz,
    Config,
};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, mutex::Mutex};
use embassy_time::Timer;
use panic_halt as _;
use pe437xx::{asynch::spi::PE43701, Address, Attenuation};

bind_interrupts!(struct Irqs {
    DMA1_CHANNEL3 => dma::InterruptHandler<peripherals::DMA1_CH3>;
});

const PE_ADDR: Address = match Address::new(0x00) {
    Ok(addr) => addr,
    Err(_) => panic!("Invalid address"),
};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());

    let mut config = SpiConfig::default();
    config.mode = MODE_0;
    config.frequency = Hertz(10_000_000);

    let le = Output::new(p.PA3, Level::Low, Speed::Medium);
    let spi_bus = Spi::new_txonly(p.SPI1, p.PA5, p.PA7, p.DMA1_CH3, Irqs, config);
    let spi_bus = Mutex::<NoopRawMutex, _>::new(spi_bus);
    let spi = SpiDevice::new(&spi_bus, DummyPin::new_low());

    let mut pe43701 = PE43701::new(spi, le, PE_ADDR).unwrap();

    let mut attenuation = Attenuation::from_db(16.25).unwrap();
    loop {
        pe43701.set_attenuation(attenuation).await.unwrap();
        attenuation = attenuation.wrapping_add(Attenuation::quarter_db());
        Timer::after_millis(500).await;
    }
}
