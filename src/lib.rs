//! Math module
pub mod polynomial;
pub mod ring;

pub use crate::traits::Zero;

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