use crate::{InvalidAddress, types::InvalidAttenuation};

/// Errors for the Pe437xx driver
#[derive(Debug, thiserror::Error)]
pub enum Error<SpiError, LeError> {
    /// Error communicating over SPI
    #[error(transparent)]
    Spi(SpiError),
    /// An error controlling LE Pin
    #[error(transparent)]
    Le(LeError),

    /// Invalid address
    #[error(transparent)]
    InvalidAddress(#[from] InvalidAddress),

    /// Invalid attenuation value
    #[error(transparent)]
    InvalidAttenuation(#[from] InvalidAttenuation),
}
