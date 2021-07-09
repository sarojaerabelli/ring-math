//! Complex number module
use std::ops::{Add, Sub, Mul, AddAssign};
use crate::traits::Zero;

/// Structure representing a complex number
///
/// # Attributes
/// * `real` - Real part of complex number
/// * `imag` - Imaginary part of complex number
#[derive(Copy, Clone)]
pub struct Complex<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + Copy> {
    pub real: T,
    pub imag: T
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + Copy> Add for Complex<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + Copy> AddAssign for Complex<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        };
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + Copy> Mul for Complex<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + Copy> Zero<Complex<T>> for Complex<T> {
    fn zero() -> Complex<T> {
        return Complex{real: T::zero(), imag: T::zero()}
    }
}



#[cfg(test)]
mod tests;