use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct S(f64, f64);

impl S {
    pub fn new(mg: f64, eg: f64) -> Self {
        Self(mg, eg)
    }

    pub fn mg(self) -> f64 {
        self.0
    }

    pub fn eg(self) -> f64 {
        self.1
    }

    pub fn square(self) -> Self {
        Self(self.0.powi(2), self.1.powi(2))
    }

    pub fn sqrt(self) -> Self {
        Self(self.0.sqrt(), self.1.sqrt())
    }

    pub fn min(self, m: f64) -> Self {
        Self(self.0.min(m), self.1.min(m))
    }

    pub fn max(self, m: f64) -> Self {
        Self(self.0.max(m), self.1.max(m))
    }

    pub fn activation(self) -> S {
        self.max(0.0)
    }

    pub fn activation_prime(self) -> S {
        S::new(
            if self.mg() > 0.0 {
                1.0
            } else {
                0.0
            },
            if self.eg() > 0.0 {
                1.0
            } else {
                0.0
            },
        )
    }
}

impl Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "s({}, {})", self.0 as i32, self.1 as i32)
    }
}

impl Div for S {
    type Output = S;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl Div<f64> for S {
    type Output = S;
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl Mul for S {
    type Output = S;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Mul<S> for f64 {
    type Output = S;
    fn mul(self, rhs: S) -> Self::Output {
        S(self * rhs.0, self * rhs.1)
    }
}

impl Mul<f64> for S {
    type Output = S;
    fn mul(self, rhs: f64) -> Self::Output {
        S(self.0 * rhs, self.1 * rhs)
    }
}

impl Add for S {
    type Output = S;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<S> for f64 {
    type Output = S;
    fn add(self, rhs: S) -> Self::Output {
        S(self + rhs.0, self + rhs.1)
    }
}

impl AddAssign for S {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1);
    }
}

impl Sub for S {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for S {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0, self.1 - rhs.1);
    }
}

impl Neg for S {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}
