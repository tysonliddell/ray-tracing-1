use crate::geometry::vec3::Vec3;

pub const RED: Color = Color {
    red: 1.0,
    green: 0.0,
    blue: 0.0,
};
pub const GREEN: Color = Color {
    red: 0.0,
    green: 1.0,
    blue: 0.0,
};
pub const BLUE: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 1.0,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new<T, U, V>(red: T, green: U, blue: V) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
        V: Into<f64>,
    {
        Self {
            red: red.into(),
            green: green.into(),
            blue: blue.into(),
        }
    }
}

impl From<Color> for Vec3 {
    fn from(color: Color) -> Self {
        Vec3::new(color.red, color.green, color.blue)
    }
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        Color::new(v.x(), v.y(), v.z())
    }
}

#[cfg(test)]
mod tests {
    use super::Color;
    use crate::geometry::vec3::Vec3;

    #[test]
    fn into_vec3() {
        let color_v: Vec3 = Color::new(1.0, 2.0, 3.0).into();
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), color_v);
    }

    #[test]
    fn from_vec3() {
        let color: Color = Vec3::new(1.0, 2.0, 3.0).into();
        assert_eq!(Color::new(1.0, 2.0, 3.0), color);
    }
}
