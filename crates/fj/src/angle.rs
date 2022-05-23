use std::f64::consts::PI;

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
        Self::from_rad(deg * PI / 180.)
    }
    /// Retrieve value of angle as radians
    pub fn rad(&self) -> f64 {
        self.rad
    }
    /// Retrieve value of angle as degrees
    pub fn deg(&self) -> f64 {
        self.rad / PI * 180.
    }

    // ensures that the angle is always 0 <= a < 2*pi
    fn wrap(rad: f64) -> f64 {
        let modulo = rad % (2. * PI);
        if modulo < 0. {
            modulo * -1.
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

impl std::ops::Mul for Angle {
    type Output = Angle;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_rad(self.rad * rhs.rad)
    }
}

impl std::ops::MulAssign for Angle {
    fn mul_assign(&mut self, rhs: Self) {
        self.rad *= rhs.rad;
        self.wrap_assign()
    }
}

impl std::ops::Div for Angle {
    type Output = Angle;
    fn div(self, rhs: Self) -> Self::Output {
        Self::from_rad(self.rad / rhs.rad)
    }
}

impl std::ops::DivAssign for Angle {
    fn div_assign(&mut self, rhs: Self) {
        self.rad /= rhs.rad;
        self.wrap_assign()
    }
}
