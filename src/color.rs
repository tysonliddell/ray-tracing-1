use crate::geometry::vec3::Vec3;

pub const RED: Color = Color {
    red: 255,
    green: 0,
    blue: 0,
};
pub const GREEN: Color = Color {
    red: 0,
    green: 255,
    blue: 0,
};
pub const BLUE: Color = Color {
    red: 0,
    green: 0,
    blue: 255,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    /// Scale a color.
    ///
    /// # Panics
    /// Panics if the scale factor is not in the range [0,1].
    pub fn scaled(&self, scale: f64) -> Color {
        if !(0.0..=1.0).contains(&scale) {
            panic!("Illegal color scale value");
        }

        Color {
            red: (self.red as f64 * scale) as u8,
            green: (self.green as f64 * scale) as u8,
            blue: (self.blue as f64 * scale) as u8,
        }
    }
}

impl TryFrom<Vec3> for Color {
    type Error = &'static str;

    /// Convert a vector located within the unit cube into a color value. A
    /// vector (x,y,z) lies in the unit cube if and only if 0 <= x,y,z <= 1.
    ///
    /// # Errors
    /// Returns an error if the input vector is not located in the unit cube.
    fn try_from(v: Vec3) -> Result<Self, Self::Error> {
        let (red, green, blue) = (v.x(), v.y(), v.z());

        for v in [red, green, blue] {
            if !(0.0..=1.0).contains(&v) {
                return Err("Cannot convert value outside of the unit cube into a Color.");
            }
        }

        Ok(Color {
            red: ((red * 255.0) as u8),
            green: ((green * 255.0) as u8),
            blue: ((blue * 255.0) as u8),
        })
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Color {
            red: value.0,
            green: value.1,
            blue: value.2,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::Color;

    #[test]
    fn scale_color() {
        let color = Color::from((10, 20, 30));
        let half_color = Color::from((5, 10, 15));
        assert_eq!(color.scaled(0.5), half_color);
    }

    #[rstest]
    #[should_panic(expected = "Illegal color scale value")]
    #[case(-0.01)]
    #[should_panic(expected = "Illegal color scale value")]
    #[case(1.01)]
    #[should_panic(expected = "Illegal color scale value")]
    #[case(10.01)]
    #[case(0.0)]
    #[case(1.0)]
    fn scale_color_panics(#[case] scale_factor: f64) {
        let color = Color::from((10, 20, 30));
        color.scaled(scale_factor);
    }
}
