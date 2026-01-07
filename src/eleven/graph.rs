use std::collections::{HashMap, HashSet};

use tracing::info;

type VertexLookup = HashMap<String, usize>;

#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub name: String,
    pub outputs: Vec<usize>,
}

#[derive(Debug)]
pub struct Graph {
    vertices: VertexLookup,
    pub nodes: HashMap<usize, Node>,
}

impl Graph {
    pub fn new(vertices: VertexLookup, nodes: HashMap<usize, Node>) -> Self {
        Self { vertices, nodes }
    }

    pub fn find_node(&self, node_name: &str) -> &Node {
        let (_, node) = self
            .nodes
            .iter()
            .find(|i| i.1.name == node_name)
            .expect("expected a you node");

        node
    }

    pub fn travel_nodes(&self, node: &Node, num_paths: &mut usize) {
        info!(name = node.name, id = node.id, "travel");

        if node.name == "out" {
            *num_paths += 1;
            info!("done");
            return;
        }

        for nid in node.outputs.iter() {
            let next = self.nodes.get(nid).unwrap();

            self.travel_nodes(next, num_paths);
        }
    }
}

pub fn create_graph(data: HashMap<String, Vec<String>>) -> Graph {
    let mut vertices: HashSet<&str> = HashSet::new();

    for (key, value) in data.iter() {
        vertices.insert(key);

        value.iter().for_each(|i| {
            vertices.insert(i);
        });
    }

    let vertices: VertexLookup = vertices
        .into_iter()
        .map(|s| s.to_owned())
        .enumerate()
        .map(|(idx, s)| (s, idx))
        .collect();

    let mut nodes = HashMap::new();

    for (s, idx) in vertices.iter() {
        let node = Node {
            id: *idx,
            name: s.to_owned(),
            outputs: Vec::new(),
        };

        nodes.insert(*idx, node);
    }

    for (key, value) in data.iter() {
        let idx = vertices.get(key).unwrap();

        let values: Vec<usize> = value.iter().map(|i| *vertices.get(i).unwrap()).collect();

        nodes.entry(*idx).and_modify(|n| n.outputs = values);
    }

    Graph::new(vertices, nodes)
}
