#![feature(iter_map_while)]

mod graph;

fn main() {
    graph::Graph::from_file("Map.txt");
}
