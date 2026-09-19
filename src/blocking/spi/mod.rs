//! Blocking drivers for `PE437xx` chips in serial configuration mode.
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
//! [examples]: https://github.com/ianmclinden/pe437xx/tree/main/examples
//!
//! ```
//! # struct MockSpi;
//! # impl embedded_hal::spi::ErrorType for MockSpi {
//! #     type Error = std::convert::Infallible;
//! # }
//! # impl embedded_hal::spi::SpiDevice for MockSpi {
//! #     fn transaction(
//! #         &mut self,
//! #         _: &mut [embedded_hal::spi::Operation<'_, u8>],
//! #     ) -> Result<(), Self::Error> {
//! #         Ok(())
//! #     }
//! # }
//! # struct MockPin;
//! # impl embedded_hal::digital::ErrorType for MockPin {
//! #     type Error = std::convert::Infallible;
//! # }
//! # impl embedded_hal::digital::OutputPin for MockPin {
//! #     fn set_low(&mut self) -> Result<(), Self::Error> {
//! #         Ok(())
//! #     }
//! #     fn set_high(&mut self) -> Result<(), Self::Error> {
//! #         Ok(())
//! #     }
//! # }
//! # let spi = MockSpi;
//! # let le = MockPin;
//! use pe437xx::{Address, Attenuation, spi::PE43701};
//!
//! // Address configured by board layout, or other GPIO
//! let addr = Address::new(0x00).unwrap();
//!
//! // PE437xx SPI drivers require:
//! // - SPI mode 0 (CPOL=0, CPHA=0)
//! // - 8-bit words
//! // - LSB first
//! // - Max 10 MHz clock
//! let mut pe43701 = PE43701::new(spi, le, addr).unwrap();
//!
//! let attenuation = Attenuation::from_db(16.25).unwrap();
//! pe43701.set_attenuation(attenuation).unwrap();
//!
//! assert_ne!(pe43701.attenuation(), Attenuation::MIN);
//! assert_ne!(pe43701.attenuation(), Attenuation::MAX);
//! ```

mod pe437xx;
pub use pe437xx::{PE43702, PE43711};
mod pe437xx_addr;
pub use pe437xx_addr::{PE43701, PE43703, PE43704, PE43705, PE43712, PE43713};
