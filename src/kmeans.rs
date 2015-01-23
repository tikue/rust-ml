use std::f64;
use points::point::Point;
use points::cluster::Cluster;

/// Perform kmeans clustering on a collection of points and return the resulting clusters
pub fn run(points: &[Point], num_clusters: u32) -> Vec<Cluster> {
    let mut clusters = empty_clusters(num_clusters);
    let mut old_centroids = centroids(&*clusters);
    init_assign_points(clusters.as_mut_slice(), points);
    let mut new_centroids = centroids(&*clusters);
    println!("running kmeans...");
    let mut iteration = 0;
    while old_centroids != new_centroids {
        iteration += 1;
        println!("iteration {}", iteration);
        reassign_points(clusters.as_mut_slice());
        old_centroids = new_centroids;
        new_centroids = centroids(&*clusters);
    }
    clusters
}

fn empty_clusters(num_clusters: u32) -> Vec<Cluster> {
    (0..num_clusters)
        .map(|_| Cluster::empty())
        .collect()
}

fn init_assign_points(clusters: &mut [Cluster], points: &[Point]) {
    let num_clusters = clusters.len();
    for (i, &point) in points.iter().enumerate() {
        clusters[i % num_clusters].add(point);
    }
}

fn centroids(clusters: &[Cluster]) -> Vec<Point> {
    clusters.iter()
        .map(|c| c.centroid())
        .collect()
}

fn reassign_points(clusters: &mut [Cluster]) {
    let old_centroids: Vec<(usize, Point)> = clusters.iter()
        .map(|c| c.centroid())
        .enumerate()
        .collect();

    let points: Vec<Point> = clusters.iter_mut()
        .flat_map(|c| c.evict_all().into_iter())
        .collect();

    for point in points.into_iter() {
        let (index, _) = old_centroids.iter()
            .map(|&(index, p)| (index, point.distance(p)))
            .fold((0, f64::MAX_VALUE), 
                  |t1, t2| {
                      let ((_, f1), (_, f2)) = (t1, t2);
                      if f1 < f2 { t1 } else { t2 }
                  });

        clusters[index].add(point);
    }
}
