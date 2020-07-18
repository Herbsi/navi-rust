#![feature(iter_map_while)]

mod graph;

fn main() {
    let g = graph::Graph::from_file("Map.txt").unwrap();
    println!("{:?}", g.dijkstra(54, 56));
    println!("{:?}", g.dijkstra(32, 9));
    println!("{:?}", g.dijkstra(23, 8));
}
