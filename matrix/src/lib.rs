use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar> Matrix<T> {
    pub fn new() -> Self {
        Matrix(vec![Vec::new()])
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        let mut data = Vec::with_capacity(rows);
        for _ in 0..rows {
            data.push(vec![T::zero(); cols]);
        }
        Matrix(data)
    }

    pub fn identity(n: usize) -> Self {
        let mut data = Vec::with_capacity(n);
        for i in 0..n {
            let mut row = vec![T::zero(); n];
            row[i] = T::one();
            data.push(row);
        }
        Matrix(data)
    }

    pub fn rows(&self) -> usize {
        self.0.len()
    }

    pub fn cols(&self) -> usize {
        self.0.get(0).map_or(0, |r| r.len())
    }
}

pub trait Scalar:
    Copy +
    PartialEq +
    Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
}

macro_rules! impl_scalar {
    ($($t:ty),*) => {
        $(
            impl Scalar for $t {
                fn zero() -> Self { 0 as $t }
                fn one() -> Self { 1 as $t }
            }
        )*
    };
}

impl_scalar!(u32, u64, i32, i64);

impl Scalar for f32 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}

impl Scalar for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}
