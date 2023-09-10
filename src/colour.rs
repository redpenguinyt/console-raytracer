use gemini_engine::elements::view::Modifier;
use std::ops::{Add, AddAssign, Mul, MulAssign};

fn mul_u8_by_f64(value: u8, rhs: f64) -> u8 {
    (value as f64 * rhs).round() as u8
}

#[derive(Debug, Clone, Copy)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Colour {
    pub const BLACK: Self = Self::new(0, 0, 0);
    pub const WHITE: Self = Self::new(255, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn greyscale(v: u8) -> Self {
        Self::new(v, v, v)
    }

    pub fn as_modifier(&self) -> Modifier {
        Modifier::from_rgb(self.r, self.g, self.b)
    }
}

impl Add for Colour {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Mul<f64> for Colour {
    type Output = Colour;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(
            mul_u8_by_f64(self.r, rhs),
            mul_u8_by_f64(self.g, rhs),
            mul_u8_by_f64(self.b, rhs),
        )
    }
}

impl MulAssign<f64> for Colour {
    fn mul_assign(&mut self, rhs: f64) {
        self.r = mul_u8_by_f64(self.r, rhs);
        self.r = mul_u8_by_f64(self.g, rhs);
        self.r = mul_u8_by_f64(self.b, rhs);
    }
}
