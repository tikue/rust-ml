use std::rand::{Rng, task_rng};

/// A point on a 2-dimensional grid
#[deriving(Clone, PartialEq, Show, Zero)]
pub struct Point {
    pub x: f64,
    pub y: f64,
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
            x: task_rng().gen_range(0.0, row_range),
            y: task_rng().gen_range(0.0, column_range)
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

impl Add<Point, Point> for Point {
    fn add(&self, rhs: &Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: ToPrimitive> Div<T, Point> for Point {
    fn div(&self, rhs: &T) -> Point {
        let rhs = rhs.to_f64().unwrap();
        Point::new(self.x / rhs, self.y / rhs)
    }
}
