use std::collections::{HashMap, HashSet};

use kdtree::{KdTree, distance::squared_euclidean};

// mod part_one;

#[derive(Debug, Clone)]
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

fn find_nearest(mut kd: KdTree<f32, usize, [f32; 3]>, points: Vec<[f32; 3]>) -> Vec<Edge> {
    let mut nearest: Vec<Edge> = Vec::new();

    for (idx, point) in points.iter().enumerate() {
        // kd.remove(point, &idx)
        //     .expect("could not remove data from kdtree");

        let r = kd
            .nearest(point, 1000, &squared_euclidean)
            .expect("could not calculate nearest points in kd tree");

        let edges: Vec<Edge> = r
            .iter()
            .map(|(d, to)| Edge {
                distance: *d,
                from: idx,
                to: **to,
            })
            .collect();

        nearest.extend(edges);
    }

    nearest.sort_by(|a, b| a.distance.total_cmp(&b.distance));

    nearest
}

fn count_islands(islands: &HashMap<usize, usize>) -> usize {
    let mut unique = HashSet::new();

    for island in islands.values() {
        unique.insert(*island);
    }

    unique.len()
}

fn connect_all(nearest: Vec<Edge>, points: Vec<[f32; 3]>) {
    let mut islands: HashMap<usize, usize> = HashMap::new();
    let mut current_island = 0usize;

    for (n, edge) in nearest.iter().enumerate() {
        if let Some(from) = islands.get(&edge.from)
            && let Some(to) = islands.get(&edge.to)
        {
            let from = *from;
            let to = *to;

            if from == to {
                continue;
            }

            // connect
            println!("connecting: {} to {} edge: {:?}", to, from, edge);
            for island in islands.values_mut() {
                if *island == to {
                    *island = from;
                }
            }

            dbg!(&islands);

            let count = count_islands(&islands);

            if count == 1 {
                let from_point = points.get(edge.from);
                let to_point = points.get(edge.to);

                dbg!(edge, from_point, to_point);

                break;
            }
        } else if let Some(island_idx) = islands.get(&edge.from) {
            islands.insert(edge.to, *island_idx);
        } else if let Some(island_idx) = islands.get(&edge.to) {
            islands.insert(edge.from, *island_idx);
        } else {
            islands.insert(edge.to, current_island);
            islands.insert(edge.from, current_island);

            current_island += 1;
        }
    }

    dbg!(islands);
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

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

        let edges = find_nearest(kd, points.clone());

        connect_all(edges, points);
    }

    #[test]
    fn with_input() {
        let s = read_to_string("src/eight/input.txt").expect("could not read eight/input.txt");

        let (kd, points) = parse(&s);

        let edges = find_nearest(kd, points.clone());

        connect_all(edges, points);
    }
}
