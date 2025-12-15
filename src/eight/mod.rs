use kdtree::{KdTree, distance::squared_euclidean};

#[derive(Debug)]
struct Edge {
    distance: f32,
    from: usize,
    to: usize,
}

pub fn parse(s: &str) -> (KdTree<f32, usize, [f32; 3]>, Vec<[f32; 3]>) {
    let mut points: Vec<[f32; 3]> = Vec::new();
    let mut kd = kdtree::KdTree::new(3);

    for (idx, line) in s.lines().enumerate() {
        let point: Vec<f32> = line.split(',').map(|i| i.parse::<f32>().unwrap()).collect();

        let point: [f32; 3] = point.try_into().unwrap();

        let _ = kd.add(point, idx);

        points.push(point);
    }

    (kd, points)
}

pub fn find_nearest(mut kd: KdTree<f32, usize, [f32; 3]>, points: Vec<[f32; 3]>) {
    let mut edges: Vec<(usize, usize)> = Vec::new();

    let mut nearest: Vec<Edge> = Vec::new();

    for (idx, point) in points.iter().enumerate() {
        let r = kd.nearest(point, 2, &squared_euclidean).unwrap();

        let edges: Vec<Edge> = r[1..]
            .into_iter()
            .map(|(d, to)| Edge {
                distance: *d,
                from: idx,
                to: **to,
            })
            .collect();

        nearest.extend(edges);
    }

    nearest.sort_by(|a, b| a.distance.total_cmp(&b.distance));

    dbg!(nearest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_one() {
        let s = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        let (kd, points) = parse(s);

        find_nearest(kd, points);
    }
}
