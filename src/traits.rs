pub trait Zero<T> {
    fn zero() -> T;
}

pub trait One<T> {
    fn one() -> T;
}

pub trait Abs<T> {
    fn abs(self) -> f64;
}