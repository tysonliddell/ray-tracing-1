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

impl TryFrom<Vec3> for Color {
    type Error = &'static str;

    /// Convert a vector in the half-open range (0.0, 0.0, 0.0) - (1.0, 1.0, 1.0)
    /// to a color value.
    fn try_from(v: Vec3) -> Result<Self, Self::Error> {
        let (red, green, blue) = (v.x(), v.y(), v.z());

        for v in [red, green, blue] {
            if !(0.0..=1.0).contains(&v) {
                return Err("Value outside of the unit interval.");
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
