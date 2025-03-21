//! Utilities related to displaying things.

use std::fmt;

/// Utility struct for displays percentages. The first item in the struct is the
/// numerator and the second item in the struct is the denominator.
pub struct PercentageFormat(pub u64, pub u64);

impl fmt::Display for PercentageFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.1 == 0 {
            f.write_str("N/A")
        } else {
            let (a, b) = (self.0 as f64, self.1 as f64);
            write!(f, "{:.2}%", a / b * 100.0)
        }
    }
}
