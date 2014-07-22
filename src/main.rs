extern crate kmeans;
use kmeans::cluster::Cluster;
use kmeans::graph::Graph;
use kmeans::point::Point;
use kmeans::show::Show;
use kmeans::run::kmeans;

fn rand_cluster(x: f64, y: f64) -> Cluster {
    Cluster::gaussian(Point::new(x, y), 3.0, 20)
}

// Creates four normally distributed clusters, gathers up the points, and runs kmeans on them
// TODO: currently fails if any points are outside the graph coordinates
// TODO: points are currently lost if they have the same rounded-int coordinates
pub fn main() {
    let clusters = [
        rand_cluster(12.0, 12.0), 
        rand_cluster(20.0, 20.0), 
        rand_cluster(30.0, 30.0), 
        rand_cluster(30.0, 12.0)];
    let graph = Graph::from_clusters(clusters.as_slice());
    graph.show();
    
    let points: Vec<Point> = clusters.iter().flat_map(|c| c.iter().map(|&p| p)).collect();
    let clusters = kmeans(points.as_slice(), 4);
    println!("num clusters = {}", clusters.len());

    let graph = Graph::from_clusters(clusters.as_slice());
    graph.show();
}
