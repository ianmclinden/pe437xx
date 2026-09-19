# `pe437xx` - RF Digital Step Attenuator (DSA) Driver

[![crates.io](https://img.shields.io/crates/v/pe437xx.svg)](https://crates.io/crates/pe437xx)
[![Documentation](https://docs.rs/pe437xx/badge.svg)](https://docs.rs/pe437xx)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.82+-blue.svg)
[![CI Status](https://github.com/ianmclinden/pe437xx/actions/workflows/ci.yaml/badge.svg)](https://github.com/ianmclinden/pe437xx/actions/workflows/ci.yaml)

A `no_std` platform-agnostic driver for the PE437xx family RF Digital Step Attenuators by Peregrine Semiconductor, built on the [`embedded-hal`] 1.0 traits.

It is compatible with the [PE43701], [PE43702], [PE43703], [PE43704], [PE43705], [PE43711], [PE43712], and [PE43713].

Communication is currently only supported via the chip's SPI-like serial interface. 

## Features

- Blocking (synchronous) API using [`embedded-hal`] 1.0 (`SpiDevice` + `OutputPin`)
- Async API using [`embedded-hal-async`] 1.0 

## Feature Flags

- `async`: Enables async SPI support via [`embedded-hal-async`] 1.0.
- `defmt`: Enables logging via the [`defmt`] library.
- `serde`: Enable serialization/deserialization of the crate types with [`serde`].
- `unchecked`: Enable `_unchecked` creation of crate types.

## Examples

This crate uses [`probe-run`](https://crates.io/crates/probe-run) to run some embedded examples. See the [examples](examples) directory for more complete examples using the driver.

```rust
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
    Err(e) => panic!("{e}"),
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
```
## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.82 and up. It *might* compile with older versions but that may change in any new patch release.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   <http://opensource.org/licenses/MIT>)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
[`embedded-hal-async`]: https://github.com/rust-embedded/embedded-hal/tree/master/embedded-hal-async
[`defmt`]: https://github.com/knurling-rs/defmt
[`serde`]: https://github.com/serde-rs/serde
[PE43701]: https://www.psemi.com/wp-content/uploads/pdf/obs/pe43701ds.pdf
[PE43702]: https://www.psemi.com/wp-content/uploads/pdf/obs/pe43702ds.pdf
[PE43703]: https://www.psemi.com/wp-content/uploads/pdf/obs/pe43703ds.pdf
[PE43704]: https://www.psemi.com/pdf/datasheets/pe43704ds.pdf
[PE43705]: https://www.psemi.com/pdf/datasheets/pe43705ds.pdf
[PE43711]: https://www.psemi.com/pdf/datasheets/pe43711ds.pdf
[PE43712]: https://www.psemi.com/pdf/datasheets/pe43712ds.pdf
[PE43713]: https://www.psemi.com/pdf/datasheets/pe43713ds.pdf