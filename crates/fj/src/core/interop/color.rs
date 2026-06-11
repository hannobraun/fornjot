/// RGBA color
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Color(pub [u8; 4]);

/// Error for the color parsing issues
#[derive(Debug, PartialEq, Eq)]
pub struct ParseColorError;

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

impl TryFrom<&str> for Color {
    type Error = ParseColorError;

    fn try_from(hex_str: &str) -> Result<Self, Self::Error> {
        let trimmed_hex = hex_str.trim_start_matches('#');
        let len = trimmed_hex.len();

        let parse_component = |start: usize, end: usize| -> Option<u8> {
            u8::from_str_radix(&trimmed_hex[start..end], 16).ok()
        };

        match len {
            6 => {
                let r_opt = parse_component(0, 2);
                let g_opt = parse_component(2, 4);
                let b_opt = parse_component(4, 6);
                if let (Some(r), Some(g), Some(b)) = (r_opt, g_opt, b_opt) {
                    Ok(Self([r, g, b, 255]))
                } else {
                    Err(ParseColorError)
                }
            }
            8 => {
                let r_opt = parse_component(0, 2);
                let g_opt = parse_component(2, 4);
                let b_opt = parse_component(4, 6);
                let a_opt = parse_component(6, 8);

                // construction of color
                if let (Some(r), Some(g), Some(b), Some(a)) =
                    (r_opt, g_opt, b_opt, a_opt)
                {
                    Ok(Self([r, g, b, a]))
                } else {
                    Err(ParseColorError)
                }
            }
            _ => Err(ParseColorError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_hex_to_color() {
        assert_eq!(Color::try_from("#FF0000"), Ok(Color([255, 0, 0, 255])));
        assert_eq!(Color::try_from("FF0000"), Ok(Color([255, 0, 0, 255]))); // Test without #
        assert_eq!(Color::try_from("#00FF0080"), Ok(Color([0, 255, 0, 128])));
        assert_eq!(Color::try_from("00FF0080"), Ok(Color([0, 255, 0, 128]))); // Test without #
        assert_eq!(
            Color::try_from("#123456"),
            Ok(Color([0x12, 0x34, 0x56, 255]))
        );
        assert_eq!(
            Color::try_from("123456"),
            Ok(Color([0x12, 0x34, 0x56, 255]))
        );
        assert_eq!(
            Color::try_from("#123456AB"),
            Ok(Color([0x12, 0x34, 0x56, 0xAB]))
        );
        assert_eq!(
            Color::try_from("123456AB"),
            Ok(Color([0x12, 0x34, 0x56, 0xAB]))
        );
        assert_eq!(Color::try_from("#ABC"), Err(ParseColorError)); // Invalid length
        assert_eq!(Color::try_from("ABC"), Err(ParseColorError)); // Invalid length
        assert_eq!(Color::try_from("#GGHHII"), Err(ParseColorError)); // Invalid hex characters
        assert_eq!(Color::try_from("invalid"), Err(ParseColorError)); // Invalid input
    }
}
