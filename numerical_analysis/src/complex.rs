// first version : Voghera 2002

use common::common_data::{Couple, Domain, Crash, PI};
use crate::real::{fabs, ln, exp, sin_f64, cos_f64, arctan, nat_pow, pow_f64};

pub struct Complex {
    _re: f64,
    _im: f64,
    _ro: f64,
    _theta: f64,
}

impl Complex {
    pub fn new(real: f64, imaginary: f64) -> Self {
        let _ro = {
            let re_sq = nat_pow(real, 2);
            let im_sq = nat_pow(imaginary, 2);
            pow_f64(re_sq + im_sq, 0.5).unwrap_or(0.0)
        };
        let _theta = if fabs(real) < 1e-09 {
            0.0
        } else {
            let mut res = 0.0;
            if fabs(real) < 1e-50 {
                if imaginary > 0.0 {
                    res = std::f64::consts::FRAC_PI_2;
                } else if imaginary < 0.0 {
                    res = -std::f64::consts::FRAC_PI_2;
                }
            } else if real > 0.0 {
                res = arctan(imaginary / real);
            } else if real < 0.0 {
                if fabs(imaginary) < 1e-50 {
                    res = -std::f64::consts::PI;
                } else if imaginary > 0.0 {
                    res = arctan(imaginary / real) + std::f64::consts::PI;
                } else if imaginary < 0.0 {
                    res = arctan(imaginary / real) - std::f64::consts::PI;
                }
            }
            res
        };

        Complex {
            _re: real,
            _im: imaginary,
            _ro,
            _theta,
        }
    }

    pub fn from_polar(ro: f64, theta: f64) -> Self {
        let _re = ro * cos_f64(theta);
        let _im = ro * sin_f64(theta);
        Complex {
            _re,
            _im,
            _ro: ro,
            _theta: theta,
        }
    }

    pub fn re(&self) -> f64 { self._re }
    pub fn im(&self) -> f64 { self._im }
    pub fn set_re(&mut self, par: f64) { self._re = par; }
    pub fn set_im(&mut self, par: f64) { self._im = par; }

    pub fn length(&self) -> f64 {
        let re_sq = nat_pow(self._re, 2);
        let im_sq = nat_pow(self._im, 2);
        pow_f64(re_sq + im_sq, 0.5).unwrap_or(0.0)
    }

    pub fn arg(&self) -> Result<f64, Domain> {
        if fabs(self._re) < 1e-50 {
            if fabs(self._im) < 1e-50 {
                return Err(1);
            }
            if self._im > 0.0 {
                return Ok(std::f64::consts::FRAC_PI_2);
            } else if self._im < 0.0 {
                return Ok(-std::f64::consts::FRAC_PI_2);
            }
        }
        if self._re > 0.0 {
            Ok(arctan(self._im / self._re))
        } else if self._re < 0.0 {
            if fabs(self._im) < 1e-50 {
                Ok(-std::f64::consts::PI)
            } else if self._im > 0.0 {
                Ok(arctan(self._im / self._re) + std::f64::consts::PI)
            } else {
                Ok(arctan(self._im / self._re) - std::f64::consts::PI)
            }
        } else {
            Ok(0.0)
        }
    }

    pub fn conjugated(&self) -> Self {
        Complex::new(self._re, -self._im)
    }

    pub fn equals(&self, other: &Complex) -> bool {
        const EPSILON: f64 = 1.0e-12;
        (self._re - other._re).abs() < EPSILON && (self._im - other._im).abs() < EPSILON
    }

    pub fn real_factor(&self, factor: f64) -> Self {
        Complex::new(self._re * factor, self._im * factor)
    }

    pub fn to_string_val(&self) -> String {
        format!("{} +I* {}", self._re, self._im)
    }

    pub fn to_string_polar(&self) -> String {
        format!("{} *Exp[+I* {}]", self._ro, self._theta)
    }

    pub fn ln_c(&self) -> Result<Self, Domain> {
        let modulus = self.length();
        if modulus < 1e-60 {
            return Err(1);
        }
        let real = ln(modulus)?;
        let imaginary = self.arg()?;
        Ok(Complex::new(real, imaginary))
    }

    pub fn nat_pow_c(&self, n: usize) -> Self {
        let mut res = Complex::new(1.0, 0.0);
        for _ in 0..n {
            res = res * *self;
        }
        res
    }

    pub fn exp_c(&self) -> Self {
        let exp_re = exp(self._re);
        let cos_im = cos_f64(self._im);
        let sin_im = sin_f64(self._im);
        let mut res = Complex::new(cos_im, sin_im);
        res = res * exp_re;
        res
    }

    pub fn sin_c(&self) -> Self {
        let i_val = Complex::new(0.0, 1.0);
        let iz = i_val * *self;
        let minus_iz = i_val.neg() * *self;
        let e_iz = iz.exp_c();
        let e_minus_iz = minus_iz.exp_c();
        let factor = Complex::new(0.0, -0.5);
        (e_iz - e_minus_iz) * factor
    }

    pub fn cos_c(&self) -> Self {
        let i_val = Complex::new(0.0, 1.0);
        let iz = i_val * *self;
        let minus_iz = i_val.neg() * *self;
        let e_iz = iz.exp_c();
        let e_minus_iz = minus_iz.exp_c();
        let factor = Complex::new(0.5, 0.0);
        (e_iz + e_minus_iz) * factor
    }

    pub fn tan_c(&self) -> Result<Self, Domain> {
        let s = self.sin_c();
        let c = self.cos_c();
        if c.length() < 1e-16 {
            return Err(1);
        }
        Ok(s / c)
    }

    pub fn sinh_c(&self) -> Self {
        let minus_z = -*self;
        let e_z = self.exp_c();
        let e_minus_z = minus_z.exp_c();
        let factor = Complex::new(0.5, 0.0);
        (e_z - e_minus_z) * factor
    }

    pub fn cosh_c(&self) -> Self {
        let minus_z = -*self;
        let e_z = self.exp_c();
        let e_minus_z = minus_z.exp_c();
        let factor = Complex::new(0.5, 0.0);
        (e_z + e_minus_z) * factor
    }

    pub fn tanh_c(&self) -> Result<Self, Domain> {
        let sh = self.sinh_c();
        let ch = self.cosh_c();
        if ch.length() < 1e-16 {
            return Err(1);
        }
        Ok(sh / ch)
    }
}

fn this_clone(c: &Complex) -> Complex {
    Complex::new(c._re, c._im)
}

impl Clone for Complex {
    fn clone(&self) -> Self {
        Complex::new(self._re, self._im)
    }
}

impl Copy for Complex {}

use std::ops::{Add, Sub, Mul, Div, Neg};

impl Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Complex::new(self._re + other._re, self._im + other._im)
    }
}

impl Add<f64> for Complex {
    type Output = Self;
    fn add(self, other: f64) -> Self {
        Complex::new(self._re + other, self._im)
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Complex::new(self._re - other._re, self._im - other._im)
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Complex::new(
            self._re * other._re - self._im * other._im,
            self._re * other._im + self._im * other._re,
        )
    }
}

impl Mul<f64> for Complex {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Complex::new(self._re * other, self._im * other)
    }
}

impl Mul<i32> for Complex {
    type Output = Self;
    fn mul(self, other: i32) -> Self {
        Complex::new(self._re * (other as f64), self._im * (other as f64))
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let numerator = self * other.conjugated();
        let normalizer = nat_pow(other.length(), 2);
        if normalizer > 1e-50 {
            numerator * Complex::new(1.0 / normalizer, 0.0)
        } else {
            panic!("Division by zero vector");
        }
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self {
        Complex::new(-self._re, -self._im)
    }
}

pub const I_COMPLEX: Complex = Complex {
    _re: 0.0,
    _im: 1.0,
    _ro: 1.0,
    _theta: std::f64::consts::FRAC_PI_2,
};