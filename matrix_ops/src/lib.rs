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
            let mut row = Vec::with_capacity(cols);
            for _ in 0..cols {
                row.push(T::zero());
            }
            data.push(row);
        }
        Matrix(data)
    }

    pub fn identity(n: usize) -> Self {
        let mut data = Vec::with_capacity(n);
        for i in 0..n {
            let mut row = Vec::with_capacity(n);
            for j in 0..n {
                row.push(if i == j { T::one() } else { T::zero() });
            }
            data.push(row);
        }
        Matrix(data)
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
        $(impl Scalar for $t {
            fn zero() -> Self { 0 as $t }
            fn one() -> Self { 1 as $t }
        })*
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

impl<T: Scalar> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }
        for i in 0..self.0.len() {
            if self.0[i].len() != rhs.0[i].len() {
                return None;
            }
        }
        let mut result = Vec::with_capacity(self.0.len());
        for i in 0..self.0.len() {
            let mut row = Vec::with_capacity(self.0[i].len());
            for j in 0..self.0[i].len() {
                row.push(self.0[i][j] + rhs.0[i][j]);
            }
            result.push(row);
        }
        Some(Matrix(result))
    }
}

impl<T: Scalar> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }
        for i in 0..self.0.len() {
            if self.0[i].len() != rhs.0[i].len() {
                return None;
            }
        }
        let mut result = Vec::with_capacity(self.0.len());
        for i in 0..self.0.len() {
            let mut row = Vec::with_capacity(self.0[i].len());
            for j in 0..self.0[i].len() {
                row.push(self.0[i][j] - rhs.0[i][j]);
            }
            result.push(row);
        }
        Some(Matrix(result))
    }
}
