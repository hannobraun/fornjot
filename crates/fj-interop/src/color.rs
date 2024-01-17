/// RGBA color
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Color(pub [u8; 4]);

impl Default for Color {
    fn default() -> Self {
        // The default color is red. This is an arbitrary choice.
        Self([255, 0, 0, 255])
    }
}
