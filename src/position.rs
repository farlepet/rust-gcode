use crate::GCodeError;

/// Coordinate values are multiplied by this value prior to being stored within
/// GCodePosition/GCodeOffset
const COORDINATE_MULT: i64 = 1 << 16;

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
    /// Creates a new GCodePosition from floating point values
    pub fn from_f64(x: Option<f64>, y: Option<f64>, z: Option<f64>) -> Result<GCodePosition, GCodeError> {
        Ok(Self {
            x: if x.is_some() { Some(Self::f64_to_fixed(x.unwrap())?) } else { None },
            y: if y.is_some() { Some(Self::f64_to_fixed(y.unwrap())?) } else { None },
            z: if z.is_some() { Some(Self::f64_to_fixed(z.unwrap())?) } else { None },
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
        let val = val * (COORDINATE_MULT as f64);
        if (val > (i64::MAX as f64)) || (val < (i64::MIN as f64)) {
            Err(GCodeError::OutOfRangeError)
        } else {
            Ok(val as i64)
        }
    }

    /// Returns X component represented as an f64
    pub fn x_f64(&self) -> Option<f64> {
        self.x.map(|val| (val as f64) / (COORDINATE_MULT as f64))
    }

    /// Returns Y component represented as an f64
    pub fn y_f64(&self) -> Option<f64> {
        self.y.map(|val| (val as f64) / (COORDINATE_MULT as f64))
    }

    /// Returns Z component represented as an f64
    pub fn z_f64(&self) -> Option<f64> {
        self.z.map(|val| (val as f64) / (COORDINATE_MULT as f64))
    }

    /// Returns X,Y,Z components represented as f64's
    pub fn as_f64(&self) -> (Option<f64>, Option<f64>, Option<f64>) {
        (self.x.map(|val| (val as f64) / (COORDINATE_MULT as f64)),
         self.y.map(|val| (val as f64) / (COORDINATE_MULT as f64)),
         self.z.map(|val| (val as f64) / (COORDINATE_MULT as f64)))
    }
}
impl std::ops::Add<Self> for GCodePosition {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: if let (Some(l), Some(r)) = (self.x, rhs.x) { Some(l + r) } else { self.x },
            y: if let (Some(l), Some(r)) = (self.y, rhs.y) { Some(l + r) } else { self.y },
            z: if let (Some(l), Some(r)) = (self.z, rhs.z) { Some(l + r) } else { self.z },
        }
    }
}
impl std::ops::Sub<Self> for GCodePosition {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: if let (Some(l), Some(r)) = (self.x, rhs.x) { Some(l - r) } else { self.x },
            y: if let (Some(l), Some(r)) = (self.y, rhs.y) { Some(l - r) } else { self.y },
            z: if let (Some(l), Some(r)) = (self.z, rhs.z) { Some(l - r) } else { self.z },
        }
    }
}
impl core::fmt::Display for GCodePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_fixed(val: Option<i64>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            /* Not the best, but for now just converting back to floating-point
             * in order to display. */
            if let Some(val) = val {
                write!(f, "{}", (val as f64) / (COORDINATE_MULT as f64))
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
        assert_eq!(pos, GCodePosition::from_raw_full(1 * COORDINATE_MULT, 2  * COORDINATE_MULT, 3 * COORDINATE_MULT));

        let pos = GCodePosition::from_f64(Some(1.0), None, Some(3.0))?;
        assert_eq!(pos, GCodePosition::from_raw(Some(1 * COORDINATE_MULT), None, Some(3 * COORDINATE_MULT)));

        let pos = GCodePosition::from_f64_full(((i64::MAX / COORDINATE_MULT) as f64) + 2.0, 1.0, 1.0);
        assert_eq!(pos, Err(GCodeError::OutOfRangeError));

        let pos = GCodePosition::from_f64_full(((i64::MIN / COORDINATE_MULT) as f64) - 2.0, 1.0, 1.0);
        assert_eq!(pos, Err(GCodeError::OutOfRangeError));

        Ok(())
    }
}
