use point::Point;
use cluster::Cluster;

pub fn kmeans(points: &[Point], num_clusters: u8) -> Vec<Cluster> {
    let mut clusters = empty_clusters(num_clusters);
    let mut old_centroids = centroids(clusters.as_slice());
    init_assign_points(clusters.as_mut_slice(), points);
    let mut new_centroids = centroids(clusters.as_slice());
    println!("running kmeans...");
    let mut iteration = 0u;
    while old_centroids != new_centroids {
        iteration += 1;
        println!("iteration {}", iteration);
        reassign_points(clusters.as_mut_slice());
        old_centroids = new_centroids;
        new_centroids = centroids(clusters.as_slice());
    }
    clusters
}

fn empty_clusters(num_clusters: u8) -> Vec<Cluster> {
    let mut clusters = Vec::new();
    for _ in range(0, num_clusters) {
        clusters.push(Cluster::empty());
    }
    clusters
}

fn init_assign_points(clusters: &mut [Cluster], points: &[Point]) {
    let num_clusters = clusters.len();
    for (i, &point) in points.iter().enumerate() {
        clusters[i % num_clusters].add(point);
    }
}

fn centroids(clusters: &[Cluster]) -> Vec<Point> {
    clusters.iter().map(|c| c.centroid()).collect()
}

fn reassign_points(clusters: &mut [Cluster]) {
    let old_centroids: Vec<(uint, Point)> = clusters.iter()
        .map(|c| c.centroid())
        .enumerate()
        .collect();

    let points: Vec<Point> = clusters.mut_iter()
        .flat_map(|c| c.dump_points().move_iter())
        .collect();

    for point in points.move_iter() {
        let &(index, _) = old_centroids.iter()
            .min_by(|&&(_, p)| p.distance(point).round() as u64)
            .unwrap();

        clusters[index].add(point);
    }
}
