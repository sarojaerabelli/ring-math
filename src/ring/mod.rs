//! Complex number module
use std::ops::{Add, Sub, Mul, AddAssign};
use std::cmp::{PartialEq, Eq};
use std::fmt;
use crate::traits::{Zero, One, Abs};

/// Structure representing a complex number
///
/// # Attributes
/// * `real` - Real part of complex number
/// * `imag` - Imaginary part of complex number
#[derive(Copy, Clone)]
pub struct Complex<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T>
                   + Zero<T> + One<T>+ Copy + Abs<T> + fmt::Debug + PartialEq> {
    pub real: T,
    pub imag: T
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + One<T>
     + Copy + Abs<T> + fmt::Debug + PartialEq> Add for Complex<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + One<T>
     + Copy + Abs<T> + fmt::Debug + PartialEq> AddAssign for Complex<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        };
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + One<T>
     + Copy + Abs<T> + fmt::Debug + PartialEq> Sub for Complex<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + One<T>
     + Copy + Abs<T> + fmt::Debug + PartialEq> Mul for Complex<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + One<T>
     + Copy + Abs<T> + fmt::Debug + PartialEq> Zero<Complex<T>> for Complex<T> {
    fn zero() -> Complex<T> {
        return Complex{real: T::zero(), imag: T::zero()}
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + One<T>
      + Copy + Abs<T> + fmt::Debug + PartialEq> One<Complex<T>> for Complex<T> {
    fn one() -> Complex<T> {
        return Complex{real: T::one(), imag: T::zero()}
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + One<T>
     + Copy + Abs<T> + fmt::Debug + PartialEq> Abs<Complex<T>> for Complex<T> {
    fn abs(self) -> f64 {
        return T::abs(self.real * self.real + self.imag * self.imag);
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + One<T>
     + Copy + Abs<T> + fmt::Debug + PartialEq> fmt::Debug for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Complex")
         .field("real", &self.real)
         .field("imag", &self.imag)
         .finish()
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + One<T>
     + Copy + Abs<T> + fmt::Debug + PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imag == other.imag
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero<T> + One<T>
     + Copy + Abs<T> + fmt::Debug + PartialEq> Eq for Complex<T> {}

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

impl fmt::Debug for ModInteger32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModInteger32")
         .field("value", &self.value)
         .finish()
    }
}

impl PartialEq for ModInteger32 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for ModInteger32 {}

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

impl fmt::Debug for ModInteger64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModInteger64")
         .field("value", &self.value)
         .finish()
    }
}

impl PartialEq for ModInteger64 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for ModInteger64 {}

#[cfg(test)]
mod tests;