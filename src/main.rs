extern crate ml;

use ml::points::Cluster;
use ml::points::Point;
use ml::plots::Plot;
use ml::kmeans;

fn rand_cluster(x: f64, y: f64) -> Cluster {
    Cluster::gaussian(Point::new(x, y), 3.0, 20)
}

// Creates four normally distributed clusters, gathers up the points, and runs kmeans on them
// TODO: currently fails if any points are outside the graph coordinates
// TODO: points are currently lost if they have the same rounded-int coordinates
pub fn main() {
    let clusters = [
        rand_cluster(12.0, 12.0), 
        rand_cluster(12.0, 38.0), 
        rand_cluster(38.0, 38.0), 
        rand_cluster(38.0, 12.0)];
    let points: Vec<Point> = clusters.iter().flat_map(|c| c.iter().map(|&p| p)).collect();
    let graph = Plot::from_points(&*points);
    print!("{}", graph);
    
    let clusters = kmeans::run(&*points, 4);
    println!("num clusters = {}", clusters.len());

    let graph = Plot::from_clusters(&*clusters);
    print!("{}", graph);
}
