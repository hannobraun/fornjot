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

impl From<&str> for Color {
    fn from(hex_str: &str) -> Self {
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
                    Self([r, g, b, 255])
                } else {
                    Self::default()
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
                    Self([r, g, b, a])
                } else {
                    Self::default()
                }
            }
            _ => Self::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_color() {
        assert_eq!(Color::from("#FF0000"), Color([255, 0, 0, 255]));
        assert_eq!(Color::from("FF0000"), Color([255, 0, 0, 255])); // Test without #
        assert_eq!(Color::from("#00FF0080"), Color([0, 255, 0, 128]));
        assert_eq!(Color::from("00FF0080"), Color([0, 255, 0, 128])); // Test without #
        assert_eq!(Color::from("#123456"), Color([0x12, 0x34, 0x56, 255]));
        assert_eq!(Color::from("123456"), Color([0x12, 0x34, 0x56, 255]));
        assert_eq!(Color::from("#123456AB"), Color([0x12, 0x34, 0x56, 0xAB]));
        assert_eq!(Color::from("123456AB"), Color([0x12, 0x34, 0x56, 0xAB]));
        assert_eq!(Color::from("#ABC"), Color::default()); // Invalid length
        assert_eq!(Color::from("ABC"), Color::default()); // Invalid length
        assert_eq!(Color::from("#GGHHII"), Color::default()); // Invalid hex characters
        assert_eq!(Color::from("invalid"), Color::default()); // Invalid input
    }
}
