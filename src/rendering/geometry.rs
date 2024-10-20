use std::ops::Mul;

/// A structure that contains a point in a two-dimensional coordinate system.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Creates a new point with the given x and y coordinates.
    pub fn new<T, U>(x: T, y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl<N> Mul<N> for Point
where
    N: Into<f64> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: N) -> Self::Output {
        Self {
            x: self.x * rhs.into(),
            y: self.y * rhs.into(),
        }
    }
}

/// A structure that contains width and height values.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    /// Creates a new size with the given width and height.
    pub fn new<T, U>(width: T, height: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        Self {
            width: width.into(),
            height: height.into(),
        }
    }
}

/// A structure that contains the location and dimensions of a rectangle.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}
