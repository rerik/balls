use std::fmt;
use std::ops;

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { 
            x, 
            y 
        }
    }
    pub fn len_sqr(&self) -> f64{
        self.x*self.x + self.y*self.y
    }
    pub fn len(&self) -> f64{
        self.len_sqr().sqrt()
    }
    pub fn projection(&self, other: Vec2) -> f64 {
        ((*self)* other) / other.len()
    }
    pub fn set_len(&mut self, len: f64) {
        *self *= len / self.len();
    }
    pub fn reflect(&mut self, line: Vec2, coeff: f64) {
        let mut proj: Vec2 = line;
        proj.set_len(self.projection(line));
        let normal: Vec2 = *self - proj;
        *self -= normal * 2. * coeff;
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, multiplicator: f64) -> Self {
        Self {
            x: self.x * multiplicator,
            y: self.y * multiplicator,
        }
    }
}

impl ops::MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, multiplicator: f64) {
        *self = *self * multiplicator;
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, divider: f64) -> Self {
        Self {
            x: self.x / divider,
            y: self.y / divider,
        }
    }
}

impl ops::Mul for Vec2 {
    type Output = f64;
    // scalar product, not vector!
    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
}
