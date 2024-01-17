/// RGBA color
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Color(pub [u8; 4]);

impl Default for Color {
    fn default() -> Self {
        // The default color is red. This is an arbitrary choice.
        Self([255, 0, 0, 255])
    }
}

impl From<[u8; 4]> for Color {
    fn from(rgba: [u8; 4]) -> Self {
        Self(rgba)
    }
}

impl From<[u8; 3]> for Color {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self([r, g, b, 255])
    }
}

impl From<[f64; 4]> for Color {
    fn from(rgba: [f64; 4]) -> Self {
        let rgba = rgba.map(|value| {
            let value = value.clamp(0., 1.);
            let value: u8 = (value * 255.0) as u8;
            value
        });

        Self(rgba)
    }
}

impl From<[f64; 3]> for Color {
    fn from([r, g, b]: [f64; 3]) -> Self {
        Self::from([r, g, b, 1.])
    }
}
