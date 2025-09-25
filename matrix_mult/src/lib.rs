use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar + Clone> Matrix<T> {
    pub fn zero(rows: usize, cols: usize) -> Self {
        Matrix(vec![vec![T::zero(); cols]; rows])
    }

    pub fn identity(n: usize) -> Self {
        let mut m = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            m[i][i] = T::one();
        }
        Matrix(m)
    }
}

impl<T: Scalar + Clone> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.0[0].len() != rhs.0.len() { return None; }

        let rows = self.0.len();
        let cols = rhs.0[0].len();
        let mut result = vec![vec![T::zero(); cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                for k in 0..self.0[0].len() {
                    result[i][j] = result[i][j] + (self.0[i][k] * rhs.0[k][j].clone());
                }
            }
        }

        Some(Matrix(result))
    }
}

pub trait Scalar:
    Copy + PartialEq + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
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
