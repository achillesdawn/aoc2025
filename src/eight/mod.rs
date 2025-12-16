use std::collections::{BTreeMap, HashMap};

use kdtree::{KdTree, distance::squared_euclidean};

#[derive(Debug, Clone)]
struct Edge<'a> {
    distance: f32,
    from: usize,
    to: usize,

    from_coords: &'a [f32; 3],
    to_coords: &'a [f32; 3],
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
    let mut nearest: Vec<Edge> = Vec::new();

    for (idx, point) in points.iter().enumerate() {
        let from_coords = points.get(idx).unwrap();

        kd.remove(point, &idx).unwrap();

        let r = kd
            .nearest(point, 10, &squared_euclidean)
            .expect("could not calculate nearest points in kd tree");

        let edges: Vec<Edge> = r
            .iter()
            .map(|(d, to)| {
                let to_coords = points.get(**to).unwrap();

                Edge {
                    distance: *d,
                    from: idx,
                    to: **to,

                    from_coords,
                    to_coords,
                }
            })
            .collect();

        nearest.extend(edges);
    }

    nearest.sort_by(|a, b| a.distance.total_cmp(&b.distance));

    connect_edges(nearest);
}

fn connect_edges(nearest: Vec<Edge>) {
    let mut edges: Vec<Edge> = Vec::new();

    let mut island_count = 1usize;
    let mut islands: BTreeMap<usize, usize> = BTreeMap::new();
    let mut connection_count = 0u8;

    for edge in nearest.iter() {
        connection_count += 1;

        if let Some(first) = islands.get(&edge.to)
            && let Some(second) = islands.get(&edge.from)
        {
            // copy to drop borrowed islands
            let first = *first;
            let second = *second;

            dbg!(edge, first, second);

            if first == second {
                println!("already in same group");
                continue;
            } else {
                dbg!(&islands);

                for value in islands.values_mut() {
                    if *value == first {
                        *value = second;
                    }
                }

                dbg!(&islands);
            }
        } else if let Some(island_idx) = islands.get(&edge.to) {
            islands.insert(edge.from, *island_idx);
        } else if let Some(island_idx) = islands.get(&edge.from) {
            islands.insert(edge.to, *island_idx);
        } else {
            islands.insert(edge.to, island_count);
            islands.insert(edge.from, island_count);
            island_count += 1;
        }

        edges.push(edge.clone());

        if connection_count == 10 {
            break;
        }
    }

    dbg!(edges);
    dbg!(&islands);

    let mut counts: HashMap<usize, usize> = HashMap::new();

    for (key, value) in islands {
        counts.entry(value).and_modify(|e| *e += 1).or_insert(1);
    }

    dbg!(counts);
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
