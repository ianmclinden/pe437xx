use crate::{types::Attenuation, Address, Error};

macro_rules! impl_pe437xx_addr {
    ($ic:ident) => {
        #[doc = concat!(stringify!($ic)," RF Digital Step Attenuator")]
        #[derive(Debug)]
        pub struct $ic<SPI, LE> {
            spi: SPI,
            le: LE,
            address: Address,
            attenuation: Attenuation,
        }

        impl<SPI, LE, LeError> $ic<SPI, LE>
        where
            SPI: embedded_hal_async::spi::SpiDevice,
            LE: embedded_hal::digital::OutputPin<Error = LeError>,
        {
            #[doc = concat!("Returns a new [`", stringify!($ic), "`] at the specified [`Address`]")]
            ///
            /// `spi` must be configured in [`MODE_0`] (CPOL=0, CPHA=0), 8-bit words, LSB-First, and with a maximum
            /// clock frequency as specified by the datasheet (typ. 10 MHz).
            ///
            /// # Errors
            /// If the device cannot be configured due to underlying SPI or LE error.
            pub fn new(
                spi: SPI,
                mut le: LE,
                address: Address,
            ) -> Result<Self, Error<SPI::Error, LeError>> {
                // LE must be low while SPI data is shifted, take ownsership and drive low until needed
                le.set_low().map_err(Error::Le)?;

                Ok(Self {
                    spi,
                    le,
                    address,
                    attenuation: Attenuation::MAX,
                })
            }

            /// Set the attenuation in dB
            ///
            /// It is recommended in high-EMI environments to periodically set this value.
            ///
            /// # Errors
            /// If the device cannot be configured due to underlying SPI or LE error.
            pub async fn set_attenuation(
                &mut self,
                attenuation: Attenuation,
            ) -> Result<(), Error<SPI::Error, LeError>> {
                self.spi
                    .write(&[attenuation.steps(), self.address.into()])
                    .await
                    .map_err(Error::Spi)?;

                self.le.set_high().map_err(Error::Le)?;
                self.le.set_low().map_err(Error::Le)?;

                self.attenuation = attenuation;

                Ok(())
            }

            /// Get the configured device address
            pub const fn address(&self) -> Address {
                self.address
            }

            /// Get the previosly configured attenuation, or [`Attenuation::MAX`] if the device has not yet been configured
            pub fn attenuation(&self) -> Attenuation {
                self.attenuation
            }

            /// Destroy driver instance, returning the SPI and LE
            pub fn destroy(self) -> (SPI, LE) {
                (self.spi, self.le)
            }
        }
    };
}

impl_pe437xx_addr!(PE43701);
impl_pe437xx_addr!(PE43703);
impl_pe437xx_addr!(PE43704);
impl_pe437xx_addr!(PE43705);
impl_pe437xx_addr!(PE43712);
impl_pe437xx_addr!(PE43713);
