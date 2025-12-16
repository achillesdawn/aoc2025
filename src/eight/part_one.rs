use super::Edge;
use std::collections::{BTreeMap, HashMap, HashSet};

#[allow(dead_code)]
fn connect_edges(nearest: Vec<Edge>, num_connections: usize) {
    let mut edges: Vec<Edge> = Vec::new();
    let mut edge_set: HashSet<(usize, usize)> = HashSet::new();
    let mut island_count = 0usize;
    let mut islands: BTreeMap<usize, usize> = BTreeMap::new();
    let mut connection_count = 0usize;

    for edge in nearest.iter() {
        if edge_set.contains(&(edge.to, edge.from)) || edge_set.contains(&(edge.from, edge.to)) {
            continue;
        }

        edge_set.insert((edge.to, edge.from));
        edge_set.insert((edge.from, edge.to));

        connection_count += 1;

        if connection_count - 1 == num_connections {
            break;
        }

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
    }

    dbg!(edges);
    dbg!(&islands);

    let mut counts: HashMap<usize, usize> = HashMap::new();

    for value in islands.values() {
        counts.entry(*value).and_modify(|e| *e += 1).or_insert(1);
    }

    dbg!(&counts);

    let mut counts: Vec<_> = counts.values().collect();

    counts.sort();

    dbg!(counts);
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::eight::{find_nearest, parse};

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

        let edges = find_nearest(kd, points);

        connect_edges(edges, 10);
    }

    #[test]
    fn with_input() {
        let s = read_to_string("src/eight/input.txt").expect("could not read eight/input.txt");

        let (kd, points) = parse(&s);

        let edges = find_nearest(kd, points);

        connect_edges(edges, 1000);
    }
}
