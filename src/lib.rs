use std::{
    num::NonZero,
    ops::{Add, Div, Mul, Neg, Sub},
};

macro_rules! c {
    ($x:expr) => {
        Expr::from_const($x)
    };
}

enum Expr {
    Zero,
    Bounded(Bounds),
}

impl Expr {
    fn from_const(c: i32) -> Self {
        match NonZero::new(c) {
            None => Self::Zero,
            Some(nz) => Self::Bounded(Bounds::from_nonzero(nz)),
        }
    }

    fn bit_bound(&self) -> u32 {
        match self {
            Self::Zero => 0,
            Self::Bounded(b) => b.bit_bound(),
        }
    }
}

impl Neg for Expr {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self
    }
}

impl Add for Expr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Zero, expr) | (expr, Self::Zero) => expr,
            (Self::Bounded(l), Self::Bounded(r)) => Self::Bounded(l + r),
        }
    }
}

impl Sub for Expr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl Mul for Expr {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Zero, _) | (_, Self::Zero) => Self::Zero,
            (Self::Bounded(l), Self::Bounded(r)) => Self::Bounded(l * r),
        }
    }
}

impl Div for Expr {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        // BUG: rhs may be zero even for bounded variant
        match (self, rhs) {
            (_, Self::Zero) => panic!("attempt to divide by zero"),
            (Self::Zero, Self::Bounded(_)) => Self::Zero,
            (Self::Bounded(l), Self::Bounded(r)) => Self::Bounded(l / r),
        }
    }
}

impl Expr {
    fn sqrt(&self) -> Self {
        // BUG: self may be less than zero
        match self {
            Self::Zero => Self::Zero,
            Self::Bounded(b) => Self::Bounded(b.sqrt()),
        }
    }
}

struct Bounds {
    u: f64,
    l: f64,
    d: u32,
}

impl Bounds {
    fn from_nonzero(c: NonZero<i32>) -> Self {
        Self {
            u: (c.unsigned_abs().get() as f64).log2(),
            l: 0.0,
            d: 1,
        }
    }

    fn bit_bound(&self) -> u32 {
        ((self.d - 1) as f64 * self.u + self.l).ceil() as u32
    }
}

impl Add for Bounds {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let a = self.u + rhs.l;
        let b = self.l + rhs.u;
        Self {
            u: a.max(b) + (1.0 + 2.0f64.powf(-(a - b).abs())).log2(),
            l: self.l + rhs.l,
            d: self.d * rhs.d,
        }
    }
}

impl Mul for Bounds {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            u: self.u + rhs.u,
            l: self.l + rhs.l,
            d: self.d * rhs.d,
        }
    }
}

impl Div for Bounds {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            u: self.u + rhs.l,
            l: self.l + rhs.u,
            d: self.d * rhs.d,
        }
    }
}

impl Bounds {
    fn sqrt(&self) -> Self {
        let a = (self.u + self.l) / 2.0;
        let (u, l) = if self.u >= self.l {
            (a, self.l)
        } else {
            (self.u, a)
        };
        let d = self.d * 2;
        Self { u, l, d }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_zero() {
        let expr = c!(0);
        assert_eq!(expr.bit_bound(), 0);
    }

    #[test]
    fn one_plus_one() {
        let expr = c!(1) + c!(1);
        assert_eq!(expr.bit_bound(), 0);
    }

    #[test]
    fn one_div_two() {
        let expr = c!(1) / c!(2);
        assert_eq!(expr.bit_bound(), 1);
    }

    #[test]
    fn neg_one_div_two() {
        let expr = c!(-1) / c!(2);
        assert_eq!(expr.bit_bound(), 1);
    }

    #[test]
    fn one_div_four() {
        let expr = c!(1) / c!(4);
        assert_eq!(expr.bit_bound(), 2);
    }

    #[test]
    fn one_div_int_min() {
        let expr = c!(1) / c!(i32::MIN);
        assert_eq!(expr.bit_bound(), 31);
    }

    #[test]
    fn half_minus_third() {
        let expr = c!(1) / c!(2) - c!(1) / c!(3);
        assert_eq!(expr.bit_bound(), 3);
    }

    #[test]
    fn third_minus_fourth() {
        let expr = c!(1) / c!(3) - c!(1) / c!(4);
        assert_eq!(expr.bit_bound(), 4);
    }

    #[test]
    fn half_times_half() {
        let expr = (c!(1) / c!(2)) * (c!(1) / c!(2));
        assert_eq!(expr.bit_bound(), 2);
    }

    #[test]
    fn half_times_third() {
        let expr = (c!(1) / c!(2)) * (c!(1) / c!(3));
        assert_eq!(expr.bit_bound(), 3);
    }

    #[test]
    fn sqrt_half_times_sqrt_third() {
        let expr = (c!(1) / c!(2)).sqrt() * (c!(1) / c!(3)).sqrt();
        assert_eq!(expr.bit_bound(), 2);
    }

    #[test]
    fn sqrt_two() {
        let expr = c!(2).sqrt();
        assert_eq!(expr.bit_bound(), 1);
    }

    #[test]
    fn sqrt_five() {
        let expr = c!(5).sqrt();
        assert_eq!(expr.bit_bound(), 2);
    }

    #[test]
    fn sqrt_half() {
        let expr = (c!(1) / c!(2)).sqrt();
        assert_eq!(expr.bit_bound(), 1);
    }

    #[test]
    fn sqrt_three_halfs() {
        let expr = (c!(3) / c!(2)).sqrt();
        assert_eq!(expr.bit_bound(), 3);
    }

    #[test]
    fn sqrt_sqrt_7_over_33() {
        let expr = (c!(7) / c!(33)).sqrt().sqrt();
        assert_eq!(expr.bit_bound(), 12);
    }

    #[test]
    fn sqrt_half_minus_third() {
        let expr = (c!(1) / c!(2) - c!(1) / c!(3)).sqrt();
        assert_eq!(expr.bit_bound(), 5);
    }

    #[test]
    fn two_sqrt_two() {
        let expr = c!(2) * c!(2).sqrt();
        assert_eq!(expr.bit_bound(), 2);
    }

    #[test]
    fn two_sqrt_two_minus_sqrt_eight() {
        let expr = c!(2) * c!(2).sqrt() - c!(8).sqrt();
        assert_eq!(expr.bit_bound(), 8);
    }

    #[test]
    fn sqrt_half_div_two() {
        let expr = (c!(2) / (c!(1) / c!(2))).sqrt();
        assert_eq!(expr.bit_bound(), 1);
    }

    #[test]
    fn sqrt_two_div_sqrt_three() {
        let expr = c!(2).sqrt() / c!(3).sqrt();
        assert_eq!(expr.bit_bound(), 3);
    }

    #[test]
    fn complex_zero() {
        let expr = (c!(2).sqrt() + c!(3).sqrt()) - (c!(5) + c!(2) * c!(6).sqrt()).sqrt();
        assert_eq!(expr.bit_bound(), 40);
    }

    #[test]
    fn sqrt_zero_plus_half() {
        let expr = c!(0).sqrt() + c!(1) / c!(2);
        assert_eq!(expr.bit_bound(), 1);
    }

    #[test]
    fn zero_plus_zero() {
        let expr = c!(0) + c!(0);
        assert_eq!(expr.bit_bound(), 0);
    }

    #[test]
    fn one_plus_zero() {
        let expr = c!(1) + c!(0);
        assert_eq!(expr.bit_bound(), 0);
    }

    #[test]
    fn zero_minus_zero() {
        let expr = c!(0) - c!(0);
        assert_eq!(expr.bit_bound(), 0);
    }

    #[test]
    fn zero_minus_one() {
        let expr = c!(0) - c!(1);
        assert_eq!(expr.bit_bound(), 0);
    }

    #[test]
    fn one_minus_zero() {
        let expr = c!(1) - c!(0);
        assert_eq!(expr.bit_bound(), 0);
    }

    #[test]
    fn zero_times_zero() {
        let expr = c!(0) * c!(0);
        assert_eq!(expr.bit_bound(), 0);
    }

    #[test]
    fn zero_times_one() {
        let expr = c!(0) * c!(1);
        assert_eq!(expr.bit_bound(), 0);
    }

    #[test]
    fn one_times_zero() {
        let expr = c!(1) * c!(0);
        assert_eq!(expr.bit_bound(), 0);
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn zero_div_zero() {
        let _ = c!(0) / c!(0);
    }

    #[test]
    fn zero_div_one() {
        let expr = c!(0) / c!(1);
        assert_eq!(expr.bit_bound(), 0);
    }

    #[test]
    #[should_panic]
    fn one_div_zero() {
        let expr = c!(1) / c!(0);
        assert_eq!(expr.bit_bound(), 0);
    }

    #[test]
    fn neg_zero() {
        let expr = -c!(0);
        assert_eq!(expr.bit_bound(), 0);
    }

    #[test]
    fn neg_expr() {
        let expr = -(c!(1) / c!(2)).sqrt();
        assert_eq!(expr.bit_bound(), 1);
    }
}
