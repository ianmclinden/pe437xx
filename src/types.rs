/// Invalid address
#[derive(Debug, Copy, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid device address")]
pub struct InvalidAddress;

/// A Pe437xx device address
///
/// Configurable on PE43701, PE43703, PE43704, PE43705, PE43712, and PE43713
/// by tying address pins `A0`, `A1`, and `A2` high for 1, or low for 0.
///
/// The address pins represent the LSB of a `u8` address, so the valid addresses are in the range `0..=7`.
///
/// ## Address Table
/// | A2 | A1 | A0 | Address |
/// |:-:|:-:|:-:|------:|
/// | L | L | L | 0b000 |
/// | L | L | H | 0b001 |
/// | L | H | L | 0b010 |
/// | L | H | H | 0b011 |
/// | H | L | L | 0b100 |
/// | H | L | H | 0b101 |
/// | H | H | L | 0b110 |
/// | H | H | H | 0b111 |
///
/// ## Example
/// ```
/// use pe437xx::Address;
///
/// assert!(Address::new(0).is_ok());
/// assert!(Address::new(7).is_ok());
/// assert!(Address::new(8).is_err());
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(u8);

impl Address {
    /// The minimum configurable address
    const MIN_ADDR: u8 = 0;
    /// The maximum configurable address
    const MAX_ADDR: u8 = 7;

    /// Create a new [`Address`]
    ///
    /// # Errors
    /// If the address is outside the supported range `0..=7`
    #[inline]
    pub const fn new(address: u8) -> Result<Self, InvalidAddress> {
        match address {
            Self::MIN_ADDR..=Self::MAX_ADDR => Ok(Self(address)),
            _ => Err(InvalidAddress),
        }
    }

    /// Creates a new [`Address`] without checking if the address is in-range.
    /// This results in undefined behaviour if the address is outside the acceptible range.
    ///
    /// # Safety
    /// The caller must guarantee that `address` is in the range `0..=7`.
    #[cfg(feature = "unchecked")]
    #[must_use]
    #[inline]
    pub unsafe fn new_unchecked(address: u8) -> Self {
        Self(address)
    }
}

impl TryFrom<u8> for Address {
    type Error = InvalidAddress;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Address> for u8 {
    fn from(value: Address) -> Self {
        value.0
    }
}

impl core::fmt::Display for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let a = u8::deserialize(deserializer)?;
        Address::new(a).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Address {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{}", self.0);
    }
}

/// Invalid attenuation
#[derive(Debug, Copy, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid attenuation")]
pub struct InvalidAttenuation;

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
pub const fn round_f32(x: f32) -> i32 {
    let t = x as i32;
    if x >= 0.0 {
        if x - t as f32 >= 0.5 {
            t + 1
        } else {
            t
        }
    } else {
        if t as f32 - x >= 0.5 {
            t - 1
        } else {
            t
        }
    }
}

/// Attenuation level in decibels
///
/// The attenuation level is specified in quarter-decibel steps, from `0..=127` (`0.0..=31.75` dB)
///
/// ## Example
/// ```
/// use pe437xx::Attenuation;
///
/// assert!(Attenuation::from_db(1.0).is_ok()); // 1 dB
/// assert!(Attenuation::from_steps(4).is_ok()); // 1 dB
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Attenuation {
    steps: u8,
}

impl Attenuation {
    /// The maximum number of quarter-decibel steps that can be configured (31.75 dB)
    pub const STEPS_MAX: u8 = 127;

    /// The minimum [`Attenuation`] value (0 dB)
    pub const MIN: Self = Self { steps: 0 };

    /// The maximum  [`Attenuation`] value (31.75 dB)
    pub const MAX: Self = Self {
        steps: Self::STEPS_MAX,
    };

    /// Creates a new [`Attenuation`] from the supplied steps in quarter decibels.
    ///
    /// # Errors
    /// If the number of steps is not in the range `0..=127`
    ///
    /// # Example
    /// ```
    /// use pe437xx::Attenuation;
    ///
    /// assert!(Attenuation::from_steps(0).is_ok());
    /// assert!(Attenuation::from_steps(127).is_ok());
    /// assert!(Attenuation::from_steps(128).is_err());
    /// ```
    #[inline]
    pub const fn from_steps(steps: u8) -> Result<Self, InvalidAttenuation> {
        match steps {
            0..=Self::STEPS_MAX => Ok(Self { steps }),
            _ => Err(InvalidAttenuation),
        }
    }

    /// Creates a new [`Attenuation`], rounding to the nearest quarter decibel.
    ///
    /// # Errors
    /// If the supplied value is not in the range `0.0..=31.75`
    ///
    /// # Example
    /// ```
    /// use pe437xx::Attenuation;
    ///
    /// assert!(Attenuation::from_db(0.0).is_ok());
    /// assert!(Attenuation::from_db(31.75).is_ok());
    /// assert!(Attenuation::from_db(32.0).is_err());
    /// ```
    #[inline]
    pub const fn from_db(db: f32) -> Result<Self, InvalidAttenuation> {
        let db = match db {
            0.0..=31.75 => db,
            _ => return Err(InvalidAttenuation),
        };

        let steps = round_f32(db * 4.0);

        #[allow(
            clippy::cast_precision_loss,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss
        )]
        match db {
            0.0..=127.0 => Attenuation::from_steps(steps as u8),
            _ => Err(InvalidAttenuation),
        }
    }

    /// Creates a new [`Attenuation`] without checking if the number of steps is in-range.
    /// This results in undefined behaviour if the number of steps is outside the acceptible range.
    ///
    ///
    /// # Safety
    /// The caller must guarantee that `address` is in the range `0..=Attenuation::STEPS_MAX`.
    #[cfg(feature = "unchecked")]
    #[must_use]
    #[inline]
    pub unsafe fn from_steps_unchecked(steps: u8) -> Self {
        Self { steps }
    }

    /// Create 1 dB of [`Attenuation`]
    ///
    /// # Example
    /// ```
    /// use pe437xx::Attenuation;
    ///
    /// assert_eq!(Attenuation::one_db(), Attenuation::from_db(1.0).unwrap());
    /// ```
    #[must_use]
    #[inline]
    pub const fn one_db() -> Self {
        Self { steps: 4 }
    }

    /// Create 0.5 dB of [`Attenuation`]
    ///
    /// # Example
    /// ```
    /// use pe437xx::Attenuation;
    ///
    /// assert_eq!(Attenuation::half_db(), Attenuation::from_db(0.5).unwrap());
    /// ```
    #[must_use]
    #[inline]
    pub const fn half_db() -> Self {
        Self { steps: 2 }
    }

    /// Create 0.25 dB of [`Attenuation`]
    ///
    /// # Example
    /// ```
    /// use pe437xx::Attenuation;
    ///
    /// assert_eq!(Attenuation::quarter_db(), Attenuation::from_db(0.25).unwrap());
    /// ```
    #[must_use]
    #[inline]
    pub const fn quarter_db() -> Self {
        Self { steps: 1 }
    }

    /// Get this [`Attenuation`] value in decibels
    #[must_use]
    #[inline]
    pub const fn db(self) -> f32 {
        self.steps as f32 * 0.25
    }

    /// Get this [`Attenuation`] value in quarter-decibel steps
    #[must_use]
    #[inline]
    pub const fn steps(self) -> u8 {
        self.steps
    }

    /// Saturating addition. Computes self + rhs, saturating at the numeric bounds instead of overflowing.
    #[must_use]
    pub const fn saturating_add(&self, rhs: Self) -> Self {
        let steps = self.steps.saturating_add(rhs.steps);
        if steps > Self::STEPS_MAX {
            Self {
                steps: Self::STEPS_MAX,
            }
        } else {
            Self { steps }
        }
    }

    /// Wrapping addition. Computes self + rhs, wrapping around at the numeric bounds instead of overflowing.
    #[must_use]
    pub const fn wrapping_add(&self, rhs: Self) -> Self {
        let steps = self.steps.wrapping_add(rhs.steps) % (Self::STEPS_MAX + 1);
        Self { steps }
    }

    /// Saturating subtraction. Computes self - rhs, saturating at the numeric bounds instead of overflowing.
    #[must_use]
    pub const fn saturating_sub(&self, rhs: Self) -> Self {
        Self {
            steps: self.steps.saturating_sub(rhs.steps),
        }
    }

    /// Wrapping subtraction. Computes self - rhs, wrapping around at the numeric bounds instead of overflowing.
    #[must_use]
    pub const fn wrapping_sub(&self, rhs: Self) -> Self {
        let steps = self.steps.wrapping_sub(rhs.steps);
        if steps > Self::STEPS_MAX {
            Self {
                steps: steps - (Self::STEPS_MAX + 1),
            }
        } else {
            Self { steps }
        }
    }
}

impl Default for Attenuation {
    fn default() -> Self {
        Self::MAX
    }
}

impl core::fmt::Display for Attenuation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.2} dB", self.db())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Attenuation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.steps)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Attenuation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let a = u8::deserialize(deserializer)?;
        Attenuation::from_steps(a).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Attenuation {
    fn format(&self, fmt: defmt::Formatter) {
        let db = self.db();
        #[allow(clippy::cast_possible_truncation)]
        let whole = db as i32;
        #[allow(clippy::cast_precision_loss)]
        let deci = round_f32((db - whole as f32) * 100.0).abs();

        defmt::write!(fmt, "{}.{:02} dB", whole, deci);
    }
}

#[cfg(test)]
pub mod tests {
    extern crate std;

    use crate::{Address, Attenuation, InvalidAddress, InvalidAttenuation};

    #[test]
    fn test_new_addr() {
        for a in 0u8..=7 {
            let addr = Address::new(a);
            assert!(addr.is_ok_and(|addr| addr.0 == a));
        }

        assert_eq!(Address::new(8), Err(InvalidAddress));
    }

    #[test]
    fn test_addr_from_u8() {
        for a in 0u8..=7 {
            let addr = Address::try_from(a);
            assert!(addr.is_ok_and(|addr| addr.0 == a));
        }

        assert_eq!(Address::try_from(8), Err(InvalidAddress));
    }

    #[test]
    fn test_new_attenuation() {
        for s in 0..=127 {
            let att = Attenuation::from_steps(s).unwrap();
            assert_eq!(att.steps(), s);
            assert!((att.db() - (f32::from(s) / 4.0)).abs() < f32::EPSILON);
        }

        assert_eq!(Attenuation::from_steps(128), Err(InvalidAttenuation));

        for s in 0..=127 {
            let db = f32::from(s) / 4.0;
            let att = Attenuation::from_db(db).unwrap();
            assert_eq!(att.steps(), s);
            assert!((att.db() - db).abs() < f32::EPSILON);
        }

        assert_eq!(Attenuation::from_db(32.0), Err(InvalidAttenuation));
        assert_eq!(Attenuation::from_db(f32::INFINITY), Err(InvalidAttenuation));
        assert_eq!(
            Attenuation::from_db(f32::NEG_INFINITY),
            Err(InvalidAttenuation)
        );
        assert_eq!(Attenuation::from_db(f32::NAN), Err(InvalidAttenuation));
    }

    #[test]
    fn test_attenuation_helpers() {
        assert_eq!(Attenuation::quarter_db().steps(), 1);
        assert!((Attenuation::quarter_db().db() - 0.25).abs() < f32::EPSILON);
        assert_eq!(Attenuation::half_db().steps(), 2);
        assert!((Attenuation::half_db().db() - 0.5).abs() < f32::EPSILON);
        assert_eq!(Attenuation::one_db().steps(), 4);
        assert!((Attenuation::one_db().db() - 1.0).abs() < f32::EPSILON);
        assert_eq!(Attenuation::MAX.steps(), 127);
        assert!((Attenuation::MAX.db() - 31.75).abs() < f32::EPSILON);
        assert_eq!(Attenuation::MIN.steps(), 0);
        assert!((Attenuation::MIN.db() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_attenuation_add() {
        let att = Attenuation::MIN;
        assert_eq!(att.wrapping_add(Attenuation::MIN).steps(), 0);
        assert_eq!(att.wrapping_add(Attenuation::quarter_db()).steps(), 1);
        assert_eq!(att.wrapping_add(Attenuation::half_db()).steps(), 2);
        assert_eq!(att.wrapping_add(Attenuation::one_db()).steps(), 4);
        assert_eq!(att.wrapping_add(Attenuation::MAX).steps(), 127);

        assert_eq!(att.saturating_add(Attenuation::MIN).steps(), 0);
        assert_eq!(att.saturating_add(Attenuation::quarter_db()).steps(), 1);
        assert_eq!(att.saturating_add(Attenuation::half_db()).steps(), 2);
        assert_eq!(att.saturating_add(Attenuation::one_db()).steps(), 4);
        assert_eq!(att.saturating_add(Attenuation::MAX).steps(), 127);

        let att = Attenuation::MAX;
        assert_eq!(att.wrapping_add(Attenuation::MIN).steps(), 127);
        assert_eq!(att.wrapping_add(Attenuation::quarter_db()).steps(), 0);
        assert_eq!(att.wrapping_add(Attenuation::half_db()).steps(), 1);
        assert_eq!(att.wrapping_add(Attenuation::one_db()).steps(), 3);
        assert_eq!(att.wrapping_add(Attenuation::MAX).steps(), 126);

        assert_eq!(att.saturating_add(Attenuation::MIN).steps(), 127);
        assert_eq!(att.saturating_add(Attenuation::quarter_db()).steps(), 127);
        assert_eq!(att.saturating_add(Attenuation::half_db()).steps(), 127);
        assert_eq!(att.saturating_add(Attenuation::one_db()).steps(), 127);
        assert_eq!(att.saturating_add(Attenuation::MAX).steps(), 127);
    }

    #[test]
    fn test_attenuation_sub() {
        let att = Attenuation::MAX;
        assert_eq!(att.wrapping_sub(Attenuation::MIN).steps(), 127);
        assert_eq!(att.wrapping_sub(Attenuation::quarter_db()).steps(), 126);
        assert_eq!(att.wrapping_sub(Attenuation::half_db()).steps(), 125);
        assert_eq!(att.wrapping_sub(Attenuation::one_db()).steps(), 123);
        assert_eq!(att.wrapping_sub(Attenuation::MAX).steps(), 0);

        assert_eq!(att.saturating_sub(Attenuation::MIN).steps(), 127);
        assert_eq!(att.saturating_sub(Attenuation::quarter_db()).steps(), 126);
        assert_eq!(att.saturating_sub(Attenuation::half_db()).steps(), 125);
        assert_eq!(att.saturating_sub(Attenuation::one_db()).steps(), 123);
        assert_eq!(att.saturating_sub(Attenuation::MAX).steps(), 0);

        let att = Attenuation::MIN;
        assert_eq!(att.wrapping_sub(Attenuation::MIN).steps(), 0);
        assert_eq!(att.wrapping_sub(Attenuation::quarter_db()).steps(), 127);
        assert_eq!(att.wrapping_sub(Attenuation::half_db()).steps(), 126);
        assert_eq!(att.wrapping_sub(Attenuation::one_db()).steps(), 124);
        assert_eq!(att.wrapping_sub(Attenuation::MAX).steps(), 1);

        assert_eq!(att.saturating_sub(Attenuation::MIN).steps(), 0);
        assert_eq!(att.saturating_sub(Attenuation::quarter_db()).steps(), 0);
        assert_eq!(att.saturating_sub(Attenuation::half_db()).steps(), 0);
        assert_eq!(att.saturating_sub(Attenuation::one_db()).steps(), 0);
        assert_eq!(att.saturating_sub(Attenuation::MAX).steps(), 0);
    }
}
