use point::Point;

use std::iter::{AdditiveIterator, iterate};
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
    pub fn empty() -> Cluster {
        Cluster {
            points: vec![],
            centroid: Point::origin()
        }
    }

    pub fn gaussian(centroid: Point, std_dev: f64, num_points: uint) -> Cluster {
        let normal_x = &Normal::new(centroid.x, std_dev);
        let normal_y = &Normal::new(centroid.y, std_dev);
        let rand_point = |_| Point {
                        x: normal_x.ind_sample(&mut rand::task_rng()),
                        y: normal_y.ind_sample(&mut rand::task_rng()),
                    };
        let seed = rand_point(Point::origin());
        Cluster {
            points: iterate(rand_point, seed)
                .take(num_points)
                .collect(),
            centroid: centroid,
        }
    }

    pub fn iter(&self) -> Items<Point> {
        return self.points.iter();
    }

    pub fn add(&mut self, point: Point) {
        self.points.push(point);
        self.centroid = self.compute_centroid();
    }

    pub fn centroid(&self) -> Point {
        self.centroid
    }

    pub fn distance_from_centroid(&self, point: Point) -> f64 {
        self.centroid.distance(point)
    }

    pub fn dump_points(&mut self) -> Vec<Point> {
        let mut old_points = Vec::new();
        swap(&mut old_points, &mut self.points);
        self.centroid = Point::origin();
        old_points
    }

    fn compute_centroid(&self) -> Point {
        self.points.iter().map(|&p| p).sum() / self.points.len()
    }
}

