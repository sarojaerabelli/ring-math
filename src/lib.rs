//! polynomial module
pub mod polynomial;

pub use crate::traits::Zero;

pub mod traits;

impl Zero<f64> for f64 {
    fn zero() -> f64 {
        return 0.0
    }
}