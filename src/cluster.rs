use point::Point;

use std::iter::AdditiveIterator;
use std::mem::swap;
use std::rand;
use std::rand::distributions::{IndependentSample, Normal};
use std::slice::Items;

/// A cluster of points
#[deriving(Clone)]
pub struct Cluster {
    points: Vec<Point>,
    centroid: Point,
}

impl Cluster {
    /// Constructs an empty cluster with centroid arbitrarily placed at the origin
    pub fn empty() -> Cluster {
        Cluster {
            points: vec![],
            centroid: Point::origin()
        }
    }

    /// Constructs a cluster with a gaussian distribution centered at the given point
    pub fn gaussian(centroid: Point, std_dev: f64, num_points: uint) -> Cluster {
        let normal_x = Normal::new(centroid.x, std_dev);
        let normal_y = Normal::new(centroid.y, std_dev);
        let rng = &mut rand::task_rng();

        Cluster {
            points: Vec::from_fn(num_points, |_| Point {
                        x: normal_x.ind_sample(rng),
                        y: normal_y.ind_sample(rng),
                    }),
            centroid: centroid,
        }
    }

    /// Returns an iterator over the points in the cluster
    pub fn iter(&self) -> Items<Point> {
        return self.points.iter();
    }

    /// Add a point to the cluster
    pub fn add(&mut self, point: Point) {
        self.points.push(point);
        self.centroid = Cluster::compute_centroid(self.points.as_slice());
    }

    /// Returns the centroid of the cluster
    pub fn centroid(&self) -> Point {
        self.centroid
    }

    /// Returns the distance from the centroid to the given point
    pub fn distance_from_centroid(&self, point: Point) -> f64 {
        self.centroid.distance(point)
    }

    /// Removes and returns all the points in the cluster and places the centroid at the origin
    pub fn evict_all(&mut self) -> Vec<Point> {
        let mut old_points = Vec::new();
        swap(&mut old_points, &mut self.points);
        self.centroid = Point::origin();
        old_points
    }

    /// Returns the centroid of all the points in the cluster
    fn compute_centroid(points: &[Point]) -> Point {
        points.iter().map(|&p| p).sum() / points.len()
    }
}

