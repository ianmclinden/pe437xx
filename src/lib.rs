//! Simple driver library for the `pe437xx` family of RF Digital Step Attenuators (DSA)
//! by Peregrine Semiconductor, using [`embedded-hal`] traits.
//!
//! It is compatible with the PE43701, PE43702, PE43703, PE43704, PE43705, PE43711, PE43712, and PE43713.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/
//!
//! The driver allows configuring the DSA attenuation, and keeps the last configured value for convenience.
//! As the chips
//!
//! ## Examples:
//! See the [examples] directory.
//!
//! [examples]: https://github.com/pe437xx/pe437xx/tree/main/examples
//!
//! ## Datasheets:
//! [PE43701](https://www.psemi.com/wp-content/uploads/pdf/obs/pe43701ds.pdf)
//! [PE43702](https://www.psemi.com/wp-content/uploads/pdf/obs/pe43702ds.pdf)
//! [PE43703](https://www.psemi.com/wp-content/uploads/pdf/obs/pe43703ds.pdf)
//! [PE43704](https://www.psemi.com/pdf/datasheets/pe43704ds.pdf)
//! [PE43705](https://www.psemi.com/pdf/datasheets/pe43705ds.pdf)
//! [PE43711](https://www.psemi.com/pdf/datasheets/pe43711ds.pdf)
//! [PE43712](https://www.psemi.com/pdf/datasheets/pe43712ds.pdf)
//! [PE43713](https://www.psemi.com/pdf/datasheets/pe43713ds.pdf)

#![cfg_attr(not(feature = "unchecked"), deny(unsafe_code))]
#![deny(missing_docs)]
#![no_std]

mod error;
pub use error::Error;
mod types;
pub use types::{Address, Attenuation, InvalidAddress, InvalidAttenuation};

mod blocking;
#[doc(inline)]
pub use blocking::*;

#[cfg(feature = "async")]
pub mod asynch;
