use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::{once, successors};
use std::ops::{Index, IndexMut};

use regex::Regex;

use std::collections::BinaryHeap;

use itertools::izip;

#[path = "./state.rs"]
mod state;
use state::State;

#[path = "./utils.rs"]
mod utils;
use utils::*;

///////////////////////////////////////////////////////////////////////////////
//                                   Graph                                   //
///////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug)]
pub struct Graph {
    coordinates: Vec<(f64, f64)>,
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
            let dist = Graph::euclidean_distance(coordinates[i], coordinates[j]);
            nodes[i].push(Edge {
                cost: dist,
                node: j,
            });
            nodes[j].push(Edge {
                cost: dist,
                node: i,
            });
        }
        Ok(Graph { coordinates, nodes })
    }

    pub fn dijkstra(&self, start: usize, goal: usize) -> Path {
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
        let mut path: Vec<_> = successors(Some(goal), |&current| match previous[current] {
            Prev::Node(node) => Some(node),
            Prev::Start => None,
            _ => unreachable!(), // this would mean a node in the path constructed by the Dijkstra algorithm doesn't have a predecessor
        })
        .collect();
        path.reverse();
        // TODO this walks along the entire path again, to get the angles at each turn
        // is there a better way to do it?
        let angles = once(Angle::Straight) // special case for start
            .chain(
                izip!(path.iter(), path.iter().skip(1), path.iter().skip(2))
                    .map(|(&from, &at, &to)| self.angle(from, at, to)),
            )
            .chain(once(Angle::Straight)) // special case for end
            .collect();
        Path { path, angles }
    }

    fn euclidean_distance((a, b): (f64, f64), (x, y): (f64, f64)) -> f64 {
        ((x - a) * (x - a) + (y - b) * (y - b)).sqrt()
    }

    fn angle(&self, from: usize, at: usize, to: usize) -> Angle {
        let from = self.coordinates[from];
        let at = self.coordinates[at];
        let to = self.coordinates[to];
        let from_at = (at.0 - from.0, at.1 - from.1);
        let at_to = (to.0 - at.0, to.1 - at.1);

        let angle = ((from_at.0 * at_to.0 + from_at.1 * at_to.1)
            / Graph::euclidean_distance(from, at)
            / Graph::euclidean_distance(at, to))
        .acos()
            * 180.0
            / std::f64::consts::PI;
        if angle.abs() < 10.0 {
            Angle::Straight
        } else {
            // https://math.stackexchange.com/questions/555198/find-direction-of-angle-between-2-vectors
            // delta > 0 means turning right
            let delta = -from_at.0 * at_to.1 + from_at.1 * at_to.0;
            Angle::Turn(angle, Direction::from(delta > 0.0))
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Path {
    path: Vec<usize>,
    angles: Vec<Angle>,
}

impl Index<usize> for Path {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.path[index]
    }
}

impl IndexMut<usize> for Path {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.path[index]
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("{:^5}|{:^14}\n", "Node", "Go");
        output.push_str(&"=".repeat(5 + 1 + 14));
        output.push_str("\n");
        for (node, angle) in izip!(self.path.iter(), self.angles.iter()) {
            output.push_str(&format!("{:^5}|{:<14}\n", node, angle));
        }
        write!(f, "{}", output)
    }
}
