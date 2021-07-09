pub trait Zero<T> {
    fn zero() -> T;
}

pub trait Abs<T> {
    fn abs(self) -> f64;
}