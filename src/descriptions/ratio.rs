use std::cmp::Ordering;
use std::fmt;
use std::mem::swap;

use winapi::shared::dxgitype::DXGI_RATIONAL;

#[repr(C)]
#[derive(Copy, Clone, Eq)]
pub struct Ratio {
    pub numerator: u32,
    pub denominator: u32,
}

impl Ratio {
    #[inline]
    pub fn new(num: u32, den: u32) -> Self {
        Ratio {
            numerator: num,
            denominator: den,
        }
    }

    #[inline]
    pub fn to_f32(&self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }

    #[inline]
    pub fn to_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    #[inline]
    pub fn approximate(f: f32) -> Self {
        Ratio::new((f * 14400.0) as u32, 14400).simplify()
    }

    #[inline]
    pub fn approximate_64(f: f64) -> Self {
        Ratio::new((f * 14400.0) as u32, 14400).simplify()
    }

    #[inline]
    pub fn common_denominator(&self, other: &Ratio) -> (Ratio, Ratio) {
        if self.denominator == 0 || other.denominator == 0 {
            return (*self, *other);
        }

        let gcd = gcd(self.denominator, other.denominator);
        let den = self.denominator * (other.denominator / gcd);

        let lhs = Ratio::new(self.numerator * other.denominator / gcd, den);
        let rhs = Ratio::new(other.numerator * self.denominator / gcd, den);

        (lhs, rhs)
    }

    #[inline]
    pub fn simplify(&self) -> Self {
        if self.numerator == 0 && self.denominator == 0 {
            return *self;
        }

        let gcd = gcd(self.numerator, self.denominator);
        Ratio {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }
}

#[cfg(test)]
member_compat_test! {
    ratio_compat:
    Ratio <=> DXGI_RATIONAL {
        numerator <=> Numerator,
        denominator <=> Denominator,
    }
}

// This is safe because of the test above
impl From<DXGI_RATIONAL> for Ratio {
    #[inline]
    fn from(ratio: DXGI_RATIONAL) -> Ratio {
        unsafe { std::mem::transmute(ratio) }
    }
}

// This is safe because of the test above
impl From<Ratio> for DXGI_RATIONAL {
    #[inline]
    fn from(ratio: Ratio) -> DXGI_RATIONAL {
        unsafe { std::mem::transmute(ratio) }
    }
}

impl From<u32> for Ratio {
    #[inline]
    fn from(n: u32) -> Ratio {
        Ratio::new(n, 1)
    }
}

impl From<(u32, u32)> for Ratio {
    #[inline]
    fn from((n, d): (u32, u32)) -> Ratio {
        Ratio::new(n, d)
    }
}

impl fmt::Debug for Ratio {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Ratio({} / {})", self.numerator, self.denominator)
    }
}

impl fmt::Display for Ratio {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}/{}", self.numerator, self.denominator)
    }
}

impl PartialEq for Ratio {
    #[inline]
    fn eq(&self, other: &Ratio) -> bool {
        let a = self.simplify();
        let b = other.simplify();
        a.numerator == b.numerator && a.denominator == b.denominator
    }
}

impl PartialOrd for Ratio {
    #[inline]
    fn partial_cmp(&self, other: &Ratio) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ratio {
    #[inline]
    fn cmp(&self, other: &Ratio) -> Ordering {
        let (lhs, rhs) = self.common_denominator(other);
        lhs.numerator.cmp(&rhs.numerator)
    }
}

#[inline]
fn gcd(mut a: u32, mut b: u32) -> u32 {
    if a == 0 || b == 0 {
        return a | b;
    }

    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();

    while b != 0 {
        b >>= b.trailing_zeros();
        if a > b {
            swap(&mut a, &mut b);
        }
        b -= a;
    }

    a << shift
}

#[cfg(test)]
#[test]
fn test_gcd() {
    assert_eq!(gcd(1, 1), 1);
    assert_eq!(gcd(1, 2), 1);
    assert_eq!(gcd(2, 2), 2);
    assert_eq!(gcd(5, 3), 1);
    assert_eq!(gcd(5, 15), 5);
    assert_eq!(gcd(10, 15), 5);
    assert_eq!(gcd(30, 100), 10);
    assert_eq!(gcd(7, 49), 7);
}

#[cfg(test)]
#[test]
fn test_approximate() {
    assert_eq!(Ratio::approximate(1.0 / 60.0), Ratio::new(1, 60));
}

#[cfg(test)]
#[test]
fn test_ord() {
    assert_eq!(
        Ratio::new(1, 3).common_denominator(&Ratio::new(1, 5)),
        (Ratio::new(5, 15), Ratio::new(3, 15))
    );

    assert!(Ratio::new(1, 30) > Ratio::new(1, 60));
    assert!(Ratio::new(2, 72) > Ratio::new(1, 72));
    assert!(Ratio::new(1, 2) < Ratio::new(2, 3));
}
