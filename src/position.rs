use crate::GCodeError;

/// Represents a position
///
/// Uses fixed-point rather than floating-point to preserve accuracy over
/// repeated manipulation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GCodePosition {
    x: Option<i64>,
    y: Option<i64>,
    z: Option<i64>,
}
impl GCodePosition {
    /// Coordinate values are multiplied by this value prior to being stored within
    /// GCodePosition/GCodeOffset
    const FIXED_SCALE: i64 = 1 << 16;

    /// Creates a new GCodePosition from floating point values
    pub fn from_f64(
        x: Option<f64>,
        y: Option<f64>,
        z: Option<f64>,
    ) -> Result<GCodePosition, GCodeError> {
        Ok(Self {
            x: if x.is_some() {
                Some(Self::f64_to_fixed(x.unwrap())?)
            } else {
                None
            },
            y: if y.is_some() {
                Some(Self::f64_to_fixed(y.unwrap())?)
            } else {
                None
            },
            z: if z.is_some() {
                Some(Self::f64_to_fixed(z.unwrap())?)
            } else {
                None
            },
        })
    }

    /// Convenience method - same as from_f64, but all values are present
    pub fn from_f64_full(x: f64, y: f64, z: f64) -> Result<GCodePosition, GCodeError> {
        Self::from_f64(Some(x), Some(y), Some(z))
    }

    /// Creates a new GCodePosition from raw values, no conversion is applied.
    pub fn from_raw(x: Option<i64>, y: Option<i64>, z: Option<i64>) -> Self {
        Self { x, y, z }
    }

    /// Convenience method - same as from_raw, but all values are present
    pub fn from_raw_full(x: i64, y: i64, z: i64) -> Self {
        Self::from_raw(Some(x), Some(y), Some(z))
    }

    /// Converts a floating-point value to the fixed-point representation used
    /// bt GCodePosition
    pub fn f64_to_fixed(val: f64) -> Result<i64, GCodeError> {
        let val = val * (Self::FIXED_SCALE as f64);
        if (val > (i64::MAX as f64)) || (val < (i64::MIN as f64)) {
            Err(GCodeError::OutOfRangeError)
        } else {
            Ok(val as i64)
        }
    }

    /// Returns X component represented as an f64
    pub fn x_f64(&self) -> Option<f64> {
        self.x.map(|val| (val as f64) / (Self::FIXED_SCALE as f64))
    }

    /// Returns Y component represented as an f64
    pub fn y_f64(&self) -> Option<f64> {
        self.y.map(|val| (val as f64) / (Self::FIXED_SCALE as f64))
    }

    /// Returns Z component represented as an f64
    pub fn z_f64(&self) -> Option<f64> {
        self.z.map(|val| (val as f64) / (Self::FIXED_SCALE as f64))
    }

    /// Returns X,Y,Z components represented as f64's
    pub fn as_f64(&self) -> (Option<f64>, Option<f64>, Option<f64>) {
        (
            self.x.map(|val| (val as f64) / (Self::FIXED_SCALE as f64)),
            self.y.map(|val| (val as f64) / (Self::FIXED_SCALE as f64)),
            self.z.map(|val| (val as f64) / (Self::FIXED_SCALE as f64)),
        )
    }
}
impl std::ops::Add<Self> for GCodePosition {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: if let (Some(l), Some(r)) = (self.x, rhs.x) {
                Some(l + r)
            } else {
                self.x
            },
            y: if let (Some(l), Some(r)) = (self.y, rhs.y) {
                Some(l + r)
            } else {
                self.y
            },
            z: if let (Some(l), Some(r)) = (self.z, rhs.z) {
                Some(l + r)
            } else {
                self.z
            },
        }
    }
}
impl std::ops::AddAssign<Self> for GCodePosition {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl std::ops::Sub<Self> for GCodePosition {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: if let (Some(l), Some(r)) = (self.x, rhs.x) {
                Some(l - r)
            } else {
                self.x
            },
            y: if let (Some(l), Some(r)) = (self.y, rhs.y) {
                Some(l - r)
            } else {
                self.y
            },
            z: if let (Some(l), Some(r)) = (self.z, rhs.z) {
                Some(l - r)
            } else {
                self.z
            },
        }
    }
}
impl std::ops::SubAssign<Self> for GCodePosition {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl std::ops::Mul<f64> for GCodePosition {
    type Output = GCodePosition;
    fn mul(self, rhs: f64) -> Self::Output {
        let fixed = match Self::f64_to_fixed(rhs) {
            Ok(fixed) => fixed,
            Err(_) => panic!("Over/underflow during GCodePosition multiplication"),
        };

        let mul_fixed = |val: i64| {
            let res = ((val as i128) * (fixed as i128)) / (Self::FIXED_SCALE as i128);
            if (res > (i64::MAX as i128)) || (res < (i64::MIN as i128)) {
                panic!("Over/underflow during GCodePosition multiplication");
            }
            res as i64
        };

        Self {
            x: self.x.map(mul_fixed),
            y: self.y.map(mul_fixed),
            z: self.z.map(mul_fixed),
        }
    }
}
impl std::ops::MulAssign<f64> for GCodePosition {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}
impl std::ops::Div<f64> for GCodePosition {
    type Output = GCodePosition;
    fn div(self, rhs: f64) -> Self::Output {
        let fixed = match Self::f64_to_fixed(rhs) {
            Ok(fixed) => fixed,
            Err(_) => panic!("Over/underflow during GCodePosition division"),
        };

        let div_fixed = |val: i64| {
            let res = ((val as i128) * (Self::FIXED_SCALE as i128)) / (fixed as i128);
            if (res > (i64::MAX as i128)) || (res < (i64::MIN as i128)) {
                panic!("Over/underflow during GCodePosition division");
            }
            res as i64
        };

        Self {
            x: self.x.map(div_fixed),
            y: self.y.map(div_fixed),
            z: self.z.map(div_fixed),
        }
    }
}
impl std::ops::DivAssign<f64> for GCodePosition {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}
impl core::fmt::Display for GCodePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_fixed(val: Option<i64>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            /* Not the best, but for now just converting back to floating-point
             * in order to display. */
            if let Some(val) = val {
                write!(f, "{}", (val as f64) / (GCodePosition::FIXED_SCALE as f64))
            } else {
                write!(f, "_")
            }
        }

        write!(f, "(")?;
        fmt_fixed(self.x, f)?;
        write!(f, ",")?;
        fmt_fixed(self.y, f)?;
        write!(f, ",")?;
        fmt_fixed(self.z, f)?;
        write!(f, ")")
    }
}

pub type GCodeOffset = GCodePosition;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_conv() -> Result<(), GCodeError> {
        /* from_f64 */
        let pos = GCodePosition::from_f64_full(1.0, 2.0, 3.0)?;
        assert_eq!(
            pos,
            GCodePosition::from_raw_full(
                1 * GCodePosition::FIXED_SCALE,
                2 * GCodePosition::FIXED_SCALE,
                3 * GCodePosition::FIXED_SCALE
            )
        );

        let pos = GCodePosition::from_f64(Some(1.0), None, Some(3.0))?;
        assert_eq!(
            pos,
            GCodePosition::from_raw(
                Some(1 * GCodePosition::FIXED_SCALE),
                None,
                Some(3 * GCodePosition::FIXED_SCALE)
            )
        );

        let pos = GCodePosition::from_f64_full(
            ((i64::MAX / GCodePosition::FIXED_SCALE) as f64) + 2.0,
            1.0,
            1.0,
        );
        assert_eq!(pos, Err(GCodeError::OutOfRangeError));

        let pos = GCodePosition::from_f64_full(
            ((i64::MIN / GCodePosition::FIXED_SCALE) as f64) - 2.0,
            1.0,
            1.0,
        );
        assert_eq!(pos, Err(GCodeError::OutOfRangeError));

        Ok(())
    }

    #[test]
    fn position_add() -> Result<(), GCodeError> {
        let pos = GCodePosition::from_f64_full(1.0, 2.0, 3.0)?;

        /* Add full to full */
        let mut new_pos = pos + GCodePosition::from_f64_full(2.0, 3.0, 4.0)?;
        assert_eq!(new_pos, GCodePosition::from_f64_full(3.0, 5.0, 7.0)?);

        /* Add assign partial to full, with an absent value and a negative */
        new_pos += GCodePosition::from_f64(Some(4.0), Some(-3.0), None)?;
        assert_eq!(new_pos, GCodePosition::from_f64_full(7.0, 2.0, 7.0)?);

        /* Add partial to partial */
        let pos = GCodePosition::from_f64(Some(-1.0), None, Some(4.5))?;
        let new_pos = pos + GCodePosition::from_f64(None, Some(6.0), Some(3.5))?;
        assert_eq!(
            new_pos,
            GCodePosition::from_f64(Some(-1.0), None, Some(8.0))?
        );

        Ok(())
    }

    #[test]
    fn position_sub() -> Result<(), GCodeError> {
        let pos = GCodePosition::from_f64_full(1.0, 2.0, 3.0)?;

        /* Sub full from full */
        let mut new_pos = pos - GCodePosition::from_f64_full(2.0, 3.0, 5.0)?;
        assert_eq!(new_pos, GCodePosition::from_f64_full(-1.0, -1.0, -2.0)?);

        /* Sub assign partial from full, with an absent value and a negative */
        new_pos -= GCodePosition::from_f64(Some(4.0), Some(-3.0), None)?;
        assert_eq!(new_pos, GCodePosition::from_f64_full(-5.0, 2.0, -2.0)?);

        /* Sub partial from partial */
        let pos = GCodePosition::from_f64(Some(-1.0), None, Some(4.5))?;
        let new_pos = pos - GCodePosition::from_f64(None, Some(6.0), Some(3.5))?;
        assert_eq!(
            new_pos,
            GCodePosition::from_f64(Some(-1.0), None, Some(1.0))?
        );

        Ok(())
    }

    #[test]
    fn position_mul() -> Result<(), GCodeError> {
        let pos = GCodePosition::from_f64_full(1.0, 2.0, 3.0)?;

        /* Multiply on full */
        let new_pos = pos * 2.0;
        assert_eq!(new_pos, GCodePosition::from_f64_full(2.0, 4.0, 6.0)?);

        /* Multiply assign negative on partial */
        let mut pos = GCodePosition::from_f64(None, Some(-6.0), Some(7.0))?;
        pos *= -2.0;
        assert_eq!(pos, GCodePosition::from_f64(None, Some(12.0), Some(-14.0))?);
        Ok(())
    }

    #[test]
    fn position_div() -> Result<(), GCodeError> {
        let pos = GCodePosition::from_f64_full(1.0, 2.0, 3.0)?;

        /* Divide on full */
        let new_pos = pos / 2.0;
        assert_eq!(new_pos, GCodePosition::from_f64_full(0.5, 1.0, 1.5)?);

        /* Divide assign negative on partial */
        let mut pos = GCodePosition::from_f64(None, Some(-6.0), Some(7.0))?;
        pos /= -0.5;
        assert_eq!(pos, GCodePosition::from_f64(None, Some(12.0), Some(-14.0))?);
        Ok(())
    }
}
