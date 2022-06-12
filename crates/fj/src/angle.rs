#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use std::f64::consts::{PI, TAU};

// One gon in radians
const GON_RAD: f64 = PI / 200.;

/// An angle
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Angle {
    // The value of the angle in radians
    rad: f64,
}

impl Angle {
    /// Create a new angle specified in radians
    pub fn from_rad(rad: f64) -> Self {
        Self {
            rad: Self::wrap(rad),
        }
    }
    /// Create a new angle specified in degrees
    pub fn from_deg(deg: f64) -> Self {
        Self::from_rad(deg.to_radians())
    }
    /// Create a new angle specified in [revolutions](https://en.wikipedia.org/wiki/Turn_(angle))
    pub fn from_rev(rev: f64) -> Self {
        Self::from_rad(rev * TAU)
    }
    /// Create a new angle specified in [gon](https://en.wikipedia.org/wiki/Gradian)
    pub fn from_gon(gon: f64) -> Self {
        Self::from_rad(gon * GON_RAD)
    }
    /// Retrieve value of angle as radians
    pub fn rad(&self) -> f64 {
        self.rad
    }
    /// Retrieve value of angle as degrees
    pub fn deg(&self) -> f64 {
        self.rad.to_degrees()
    }
    /// Retrieve value of angle as [revolutions](https://en.wikipedia.org/wiki/Turn_(angle))
    pub fn rev(&self) -> f64 {
        self.rad / TAU
    }
    /// Retrieve value of angle as [gon](https://en.wikipedia.org/wiki/Gradian)
    pub fn gon(&self) -> f64 {
        self.rad / GON_RAD
    }

    // ensures that the angle is always 0 <= a < 2*pi
    fn wrap(rad: f64) -> f64 {
        let modulo = rad % TAU;
        if modulo < 0. {
            TAU + modulo
        } else {
            modulo
        }
    }

    // ensures that the angle is always 0 <= a < 2*pi
    fn wrap_assign(&mut self) {
        self.rad = Self::wrap(self.rad);
    }
}

impl std::ops::Add for Angle {
    type Output = Angle;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_rad(self.rad + rhs.rad)
    }
}

impl std::ops::AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        self.rad += rhs.rad;
        self.wrap_assign()
    }
}

impl std::ops::Sub for Angle {
    type Output = Angle;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_rad(self.rad - rhs.rad)
    }
}

impl std::ops::SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        self.rad -= rhs.rad;
        self.wrap_assign()
    }
}

impl std::ops::Mul<f64> for Angle {
    type Output = Angle;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_rad(self.rad * rhs)
    }
}

impl std::ops::Mul<Angle> for f64 {
    type Output = Angle;
    fn mul(self, rhs: Angle) -> Self::Output {
        rhs * self
    }
}

impl std::ops::MulAssign<f64> for Angle {
    fn mul_assign(&mut self, rhs: f64) {
        self.rad *= rhs;
        self.wrap_assign()
    }
}

impl std::ops::Div<f64> for Angle {
    type Output = Angle;
    fn div(self, rhs: f64) -> Self::Output {
        Self::from_rad(self.rad / rhs)
    }
}

impl std::ops::DivAssign<f64> for Angle {
    fn div_assign(&mut self, rhs: f64) {
        self.rad /= rhs;
        self.wrap_assign()
    }
}

impl std::ops::Div for Angle {
    type Output = f64;
    fn div(self, rhs: Angle) -> Self::Output {
        self.rad / rhs.rad
    }
}
