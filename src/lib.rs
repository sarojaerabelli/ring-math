//! Math module
pub mod polynomial;
pub mod ring;
pub mod utilities;

pub use crate::traits::{Zero, Abs};

pub mod traits;

impl Zero<f32> for f32 {
    fn zero() -> f32 {
        return 0.0;
    }
}

impl Zero<f64> for f64 {
    fn zero() -> f64 {
        return 0.0;
    }
}

impl Abs<f32> for f32 {
    fn abs(self) -> f64 {
        if self > 0.0 {
            return self as f64;
        } else {
            return -self as f64;
        }
    }
}

impl Abs<f64> for f64 {
    fn abs(self) -> f64 {
        if self > 0.0 {
            return self;
        } else {
            return -self;
        }
    }
}