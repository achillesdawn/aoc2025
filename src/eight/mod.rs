use std::collections::{BTreeMap, HashMap, HashSet};

use kdtree::{KdTree, distance::squared_euclidean};

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

pub fn find_nearest(kd: KdTree<f32, usize, [f32; 3]>, points: Vec<[f32; 3]>) {
    let mut nearest: Vec<Edge> = Vec::new();

    println!("finding nearest");

    for (idx, point) in points.iter().enumerate() {
        let r = kd
            .nearest(point, 1000, &squared_euclidean)
            .expect("could not calculate nearest points in kd tree");

        let edges: Vec<Edge> = r[1..]
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

    connect_edges(nearest);
}

fn connect_edges(nearest: Vec<Edge>) {
    let mut edges: Vec<Edge> = Vec::new();
    let mut edge_set: HashSet<(usize, usize)> = HashSet::new();
    let mut island_count = 0usize;
    let mut islands: BTreeMap<usize, usize> = BTreeMap::new();
    let mut connection_count = 0u32;

    for edge in nearest.iter() {
        if edge_set.contains(&(edge.to, edge.from)) || edge_set.contains(&(edge.from, edge.to)) {
            continue;
        }

        edge_set.insert((edge.to, edge.from));
        edge_set.insert((edge.from, edge.to));

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
                println!("joining {} to {}", second, first);

                let mut n = 0usize;
                // join groups
                for value in islands.values_mut() {
                    if *value == second {
                        n += 1;
                        *value = first;
                    }
                }

                println!("changed {}", n);
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

        if connection_count == 1000 {
            break;
        } else if connection_count > 950 {
            println!("hello")
        }
    }

    dbg!(edges);
    dbg!(&islands);

    let mut counts: HashMap<usize, usize> = HashMap::new();

    for value in islands.values() {
        counts.entry(*value).and_modify(|e| *e += 1).or_insert(1);
    }

    dbg!(counts);
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

        find_nearest(kd, points);
    }

    #[test]
    fn with_input() {
        let s = read_to_string("src/eight/input.txt").expect("could not read eight/input.txt");

        let (kd, points) = parse(&s);

        find_nearest(kd, points);
    }
}
