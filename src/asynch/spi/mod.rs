//! Async drivers for `PE437xx` chips in serial configuration mode.
//!
//! Requires that the chip has P/S pin set to high.
//!
//! SPI device must be configured with:
//! - [`MODE_0`] (CPOL=0, CPHA=0)
//! - 8-bit words
//! - LSB-first byte order
//! - Maximum clock frequency as specified by the chip's datasheet (typically 10 MHz)
//!
//! [`MODE_0`]: embedded_hal::spi::MODE_0
//!
//! ## Examples
//! See the [examples] directory for more complete examples.
//!
//! [examples]: https://github.com/pe437xx/pe437xx/tree/main/examples
//!
//! ```ignore
//! use pe437xx::{Address, Attenuation, async::spi::PE43701};
//!
//! let le = todo!("GPIO init, digital output");
//! let spi = todo!("SPI Device init, CPOL=0, CPHA=0, 8-bit, LSB, 10 MHz clock");
//!
//! // Address configured by board layout, or other GPIO
//! let mut pe43701 = PE43701::new(spi, le, Address::new(0x00).unwrap()).unwrap();
//!
//! // Set 16.25 dB of attenuation (0.25 dB steps)
//! let attenuation = Attenuation::from_db(16.25).unwrap();
//! pe43701.set_attenuation(attenuation).await.unwrap();
//! ```

mod pe437xx;
pub use pe437xx::{PE43702, PE43711};
mod pe437xx_addr;
pub use pe437xx_addr::{PE43701, PE43703, PE43704, PE43705, PE43712, PE43713};
