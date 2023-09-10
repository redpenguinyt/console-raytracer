use gemini_engine::elements::view::Modifier;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Debug, Clone, Copy)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Colour {
    pub const BLACK: Self = Self::new(0, 0, 0);
    pub const WHITE: Self = Self::new(255, 255, 255);
    pub const RED: Self = Self::new(255, 0, 0);
    pub const GREEN: Self = Self::new(0, 255, 0);
    pub const BLUE: Self = Self::new(0, 0, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn as_modifier(&self) -> Modifier {
        Modifier::from_rgb(self.r, self.g, self.b)
    }
}

impl Add for Colour {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self::new(
			self.r + rhs.r,
			self.g + rhs.g,
			self.b + rhs.b,
		)
	}
}

impl AddAssign for Colour {
	fn add_assign(&mut self, rhs: Self) {
		self.r += rhs.r;
		self.g += rhs.g;
		self.b += rhs.b;
	}
}

impl Mul<u8> for Colour {
	type Output = Colour;
	fn mul(self, rhs: u8) -> Self::Output {
		Self::new(
			self.r * rhs,
			self.g * rhs,
			self.b * rhs,
		)
	}
}

impl MulAssign<u8> for Colour {
	fn mul_assign(&mut self, rhs: u8) {
		self.r *= rhs;
		self.g *= rhs;
		self.b *= rhs;
	}
}