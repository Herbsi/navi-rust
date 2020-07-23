use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::successors;

use regex::Regex;

use std::collections::BinaryHeap;

#[path = "./state.rs"]
mod state;
use state::State;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Edge {
    node: usize,
    cost: f64,
}

#[derive(Copy, Clone, Debug)]
enum Prev {
    Start,
    Undefined,
    Node(usize),
}

#[derive(Default, Debug)]
pub struct Graph {
    nodes: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn from_file(filename: &str) -> std::io::Result<Graph> {
        let coordinate_re = Regex::new(r"^\s*\d+\s*([0-9.]*)\s*([0-9.]*)\s*$").unwrap();
        let connection_re = Regex::new(r"^\s*(\d+)\s*(\d*)\s*$").unwrap();

        // skip the NODES: line
        let mut buf_reader = BufReader::new(File::open(filename)?).lines().skip(1);

        let coordinates: Vec<_> = buf_reader
            .by_ref()
            .map_while(|line| {
                let line = line.expect("Unable to read line");
                coordinate_re.captures(&line).map(|caps| {
                    // caps[1,2] are safe to unwrap because inside here
                    // the regex has already been matched
                    (
                        caps[1].parse::<f64>().unwrap(),
                        caps[2].parse::<f64>().unwrap(),
                    )
                })
            })
            .collect();

        fn euclidean_distance((a, b): (f64, f64), (x, y): (f64, f64)) -> f64 {
            ((x - a) * (x - a) + (y - b) * (y - b)).sqrt()
        }

        let point_count = coordinates.len();
        let mut nodes = vec![vec![]; point_count];

        // the EDGES line has already been iterated over
        for line in buf_reader {
            let line = line.expect("Unable to read line");
            let caps = connection_re.captures(&line).unwrap();
            let (i, j) = (
                caps[1].parse::<usize>().unwrap(),
                caps[2].parse::<usize>().unwrap(),
            );
            let dist = euclidean_distance(coordinates[i], coordinates[j]);
            nodes[i].push(Edge {
                cost: dist,
                node: j,
            });
            nodes[j].push(Edge {
                cost: dist,
                node: i,
            });
        }
        Ok(Graph { nodes })
    }

    pub fn dijkstra(&self, start: usize, goal: usize) -> Vec<usize> {
        let point_count = self.nodes.len();
        // Stores the current distance value for each node
        let mut dist = vec![f64::MAX; point_count];

        // Stores whether a particular node has been visited
        let mut visited = vec![false; point_count];

        // Stores the predecessor of each node, in case it has been visited
        let mut previous = vec![Prev::Undefined; point_count];

        let mut heap = BinaryHeap::new();

        // Initialization
        dist[start] = 0.0;
        heap.push(State {
            cost: 0.0,
            position: start,
        });
        visited[start] = true;
        previous[start] = Prev::Start;

        // Main Dijkstra
        while let Some(State { cost, position }) = heap.pop() {
            if position == goal {
                break;
            }

            for edge in &self.nodes[position] {
                let next = State {
                    cost: cost + edge.cost,
                    position: edge.node,
                };

                if next.cost < dist[next.position] {
                    dist[next.position] = next.cost;
                    previous[next.position] = Prev::Node(position);

                    if !visited[next.position] {
                        heap.push(next);
                        visited[next.position] = true;
                    };
                }
            }
        }

        // Collect the path
        let mut path : Vec<_> = successors(Some(goal), |&current| match previous[current] {
            Prev::Node(node) => Some(node),
            Prev::Start => None,
            _ => unreachable!(), // this would mean a node in the path constructed by the Dijkstra algorithm doesn't have a predecessor
        })
        .collect();
        path.reverse();
        path
    }
}
