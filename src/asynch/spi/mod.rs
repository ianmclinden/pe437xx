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
//! [examples]: https://github.com/ianmclinden/pe437xx/tree/main/examples
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

mod pe437xx;
pub use pe437xx::{PE43702, PE43711};
mod pe437xx_addr;
pub use pe437xx_addr::{PE43701, PE43703, PE43704, PE43705, PE43712, PE43713};
