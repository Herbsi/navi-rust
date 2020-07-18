use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use itertools::{izip, Itertools};
use regex::Regex;

use std::cmp::{Eq, Ordering};
use std::collections::BinaryHeap;

#[derive(Copy, Clone, PartialEq)]
struct State {
    cost: f64,
    position: usize,
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // flip the ordering of other and self to turn the heap into a min-heap
        // the then_with part is necessary to make implementation consistent with PartialEq
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Edge {
    node: usize,
    cost: f64,
}

enum Prev {
    Start,
    Undefined,
    Node(Edge),
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
                    (
                        caps[1].parse::<f64>().unwrap(),
                        caps[2].parse::<f64>().unwrap(),
                    )
                })
            })
            .collect();
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
            nodes[i].push(Edge { cost: dist, node: j });
            nodes[j].push(Edge { cost: dist, node: i });
        }
        Ok(Graph { nodes })
    }

    pub fn dijkstra<I>(&self, start: usize, goal: usize) -> Vec<usize> {
        let mut dist: Vec<_> = (0..self.nodes.len()).map(|_| f64::MAX).collect();
        let mut visited: Vec<_> = (0..self.nodes.len()).map(|_| false).collect();
        let mut previous: Vec<Prev> = (0..self.nodes.len()).map(|_| Prev::Undefined).collect();

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
            // Already found a better path
            if cost > dist[position] {
                continue;
            }

            for edge in &self.nodes[position] {
                let next = State {
                    cost: cost + edge.cost,
                    position: edge.node,
                };

                if next.cost < dist[next.position] {
                    dist[next.position] = next.cost;
                    previous[next.position] = Prev::Node(*edge);

                    if !visited[next.position] {
                        heap.push(next);
                        visited[next.position] = true;
                    };
                }

                if next.position == goal {
                    break;
                }
            }
        }

        // Collect the path
        let mut path = vec![];
        let mut current = goal;
        loop {
            match previous[current] {
                Prev::Node(Edge { cost: _, node }) => {
                    path.push(node);
                    current = node;
                }
                Prev::Start => {
                    path.push(start);
                    break;
                }
                _ => {}
            };
        }
        path.into_iter().rev().collect()
    }
}

fn euclidean_distance((a, b): (f64, f64), (x, y): (f64, f64)) -> f64 {
    ((x - a) * (x - a) + (y - b) * (y - b)).sqrt()
}
