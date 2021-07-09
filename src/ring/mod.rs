//! Complex number module
use std::ops::{Add, Sub, Mul, AddAssign};
use std::fmt;
use crate::traits::{Zero, Abs};

/// Structure representing a complex number
///
/// # Attributes
/// * `real` - Real part of complex number
/// * `imag` - Imaginary part of complex number
#[derive(Copy, Clone)]
pub struct Complex<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + Copy + Abs<T> + fmt::Debug> {
    pub real: T,
    pub imag: T
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + Copy + Abs<T> + fmt::Debug> Add for Complex<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + Copy + Abs<T> + fmt::Debug> AddAssign for Complex<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        };
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + Copy + Abs<T> + fmt::Debug> Sub for Complex<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + Copy + Abs<T> + fmt::Debug> Mul for Complex<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + Copy + Abs<T> + fmt::Debug> Zero<Complex<T>> for Complex<T> {
    fn zero() -> Complex<T> {
        return Complex{real: T::zero(), imag: T::zero()}
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + Copy + Abs<T> + fmt::Debug> Abs<Complex<T>> for Complex<T> {
    fn abs(self) -> f64 {
        return T::abs(self.real * self.real + self.imag * self.imag);
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + Copy + Abs<T> + fmt::Debug> fmt::Debug for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Complex")
         .field("real", &self.real)
         .field("imag", &self.imag)
         .finish()
    }
}

/// Structure representing an integer in mod 2^32
///
/// # Attributes
/// * `value` - Value of the integer
#[derive(Copy, Clone)]
pub struct ModInteger32 {
    pub value: u64
}

impl Add for ModInteger32 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            value: (self.value + other.value) % (2^32)
        }
    }
}

impl AddAssign for ModInteger32 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            value: (self.value + other.value) % (2^32)
        };
    }
}

impl Mul for ModInteger32 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            value: (self.value * other.value) % (2^32)
        }
    }
}

impl Zero<ModInteger32> for ModInteger32 {
    fn zero() -> ModInteger32 {
        return ModInteger32{value: 0};
    }
}

/// Structure representing an integer in mod 2^64
///
/// # Attributes
/// * `value` - Value of the integer
#[derive(Copy, Clone)]
pub struct ModInteger64 {
    pub value: u128
}

impl Add for ModInteger64 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            value: (self.value + other.value) % (2^64)
        }
    }
}

impl AddAssign for ModInteger64 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            value: (self.value + other.value) % (2^64)
        };
    }
}

impl Mul for ModInteger64 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            value: (self.value * other.value) % (2^64)
        }
    }
}

impl Zero<ModInteger64> for ModInteger64 {
    fn zero() -> ModInteger64 {
        return ModInteger64{value: 0};
    }
}

#[cfg(test)]
mod tests;