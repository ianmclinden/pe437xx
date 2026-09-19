//! Simple driver library for the `pe437xx` family of RF Digital Step Attenuators (DSA)
//! by Peregrine Semiconductor, using [`embedded-hal`] traits.
//!
//! It is compatible with the PE43701, PE43702, PE43703, PE43704, PE43705, PE43711, PE43712, and PE43713.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/
//!
//! The driver allows configuring the DSA attenuation, and keeps the last configured value for convenience.
//!
//! ## Example - Serial "SPI" Mode (blocking):
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
//!
//! ## Example - Serial "SPI" Mode (async):
//!
//! ```
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # struct MockSpi;
//! # impl embedded_hal_async::spi::ErrorType for MockSpi {
//! #     type Error = std::convert::Infallible;
//! # }
//! # impl embedded_hal_async::spi::SpiDevice for MockSpi {
//! #     async fn transaction(
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
//! use pe437xx::{asynch::spi::PE43702, Attenuation};
//!
//! // Some chipsets do not support addressing
//! let mut pe43702 = PE43702::new(spi, le).unwrap();
//!
//! let attenuation = Attenuation::from_steps(127).unwrap();
//! pe43702.set_attenuation(attenuation).await.unwrap();
//!
//! assert_eq!(pe43702.attenuation(), attenuation);
//! # Ok(())
//! # }
//! ```
//!
//! See the [examples] directory for more complete examples.
//!
//! [examples]: https://github.com/ianmclinden/pe437xx/tree/main/examples
//!
//! ## Datasheets:
//! [PE43701](https://www.psemi.com/wp-content/uploads/pdf/obs/pe43701ds.pdf),
//! [PE43702](https://www.psemi.com/wp-content/uploads/pdf/obs/pe43702ds.pdf),
//! [PE43703](https://www.psemi.com/wp-content/uploads/pdf/obs/pe43703ds.pdf),
//! [PE43704](https://www.psemi.com/pdf/datasheets/pe43704ds.pdf),
//! [PE43705](https://www.psemi.com/pdf/datasheets/pe43705ds.pdf),
//! [PE43711](https://www.psemi.com/pdf/datasheets/pe43711ds.pdf),
//! [PE43712](https://www.psemi.com/pdf/datasheets/pe43712ds.pdf),
//! [PE43713](https://www.psemi.com/pdf/datasheets/pe43713ds.pdf)

#![cfg_attr(not(feature = "unchecked"), deny(unsafe_code))]
#![deny(missing_docs)]
#![no_std]

mod error;
pub use error::Error;
mod types;
pub use types::{Address, Attenuation, InvalidAddress, InvalidAttenuation};

pub mod blocking;
#[doc(hidden)]
pub use blocking::*;

#[cfg(feature = "async")]
pub mod asynch;
