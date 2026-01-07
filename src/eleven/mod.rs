use std::collections::HashMap;

use tracing::{info, warn};

mod graph;
use graph::create_graph;

use crate::eleven::graph::{Graph, Node};

fn parse_str(s: &str) -> Graph {
    let mut data: HashMap<String, Vec<String>> = HashMap::new();

    for line in s.lines() {
        let (key, values) = match line.split_once(':') {
            Some(d) => d,
            None => {
                warn!(line, "could not parse line");
                continue;
            }
        };

        let values: Vec<String> = values.split_whitespace().map(|s| s.to_owned()).collect();

        data.insert(key.to_owned(), values);
    }

    create_graph(data)
}

fn main(graph: Graph) -> usize {
    dbg!(&graph);

    let starting_point = graph.find_node("svr");

    let mut num_paths = 0usize;

    graph.travel_nodes(starting_point, &mut num_paths);

    dbg!(num_paths);

    num_paths
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::{
        eleven::{main, parse_str},
        init_tracing,
    };

    static CASE_ONE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    static CASE_TWO: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_eleven_parse_graph() {
        let graph = parse_str(CASE_ONE);

        dbg!(graph);
    }

    #[test]
    fn test_eleven_iterate_graph() {
        init_tracing();

        let g = parse_str(CASE_ONE);

        let result = main(g);

        assert_eq!(5, result);
    }

    #[test]
    fn test_eleven_two() {
        let g = parse_str(CASE_TWO);

        main(g);
    }

    #[test]
    fn test_eleven_with_input() {
        init_tracing();

        let s = read_to_string("src/eleven/input.txt").expect("expected input file");

        let g = parse_str(&s);

        let result = main(g);

        dbg!(result);
    }
}
