use std::num::Float;
use std::ops::Add;
use std::ops::Div;
use rand::{Rng, thread_rng};

/// A point on a 2-dimensional grid
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Copy for Point{}
impl PartialEq for Point{
    fn eq(&self, rhs: &Point) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

impl Point {
    /// Returns a new point with the given x- and y-coordinates
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x: x,
            y: y,
        }
    }

    /// Returns a new point with randomly generated coordinates in the given ranges
    pub fn random(row_range: f64, column_range: f64) -> Point {
        Point {
            x: thread_rng().gen_range(0.0, row_range),
            y: thread_rng().gen_range(0.0, column_range)
        }
    }

    /// Returns a new Point with x- and y-coordinates at 0
    pub fn origin() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
        }
    }

    /// Returns the Euclidean distance to the given point
    pub fn distance(&self, point: Point) -> f64 {
        ((self.x - point.x).powf(2.0) + (self.y - point.y).powf(2.0)).sqrt()
    }
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<'a> Add<&'a Point> for Point {
    type Output = Point;
    fn add(self, rhs: &Point) -> Point {
        self + *rhs
    }
}

impl Div<f64> for Point {
    type Output = Point;
    fn div(self, rhs: f64) -> Point {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

impl Div<i64> for Point {
    type Output = Point;
    fn div(self, rhs: i64) -> Point {
        Point::new(self.x / rhs as f64, self.y / rhs as f64)
    }
}

impl Div<usize> for Point {
    type Output = Point;
    fn div(self, rhs: usize) -> Point {
        Point::new(self.x / rhs as f64, self.y / rhs as f64)
    }
}
