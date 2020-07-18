use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use regex::Regex;

#[derive(Default, Debug)]
pub struct Graph {
    nodes: Vec<Vec<Option<f64>>>,
}

impl Graph {
    pub fn from_file(filename: &str) -> std::io::Result<Graph> {
        let coordinate_re = Regex::new(r"^\s*\d+\s*([0-9.]*)\s*([0-9.]*)\s*$").unwrap();
        let connection_re = Regex::new(r"^\s*(\d+)\s*(\d*)\s*$").unwrap();

        // skip the NODES: line
        let mut buf_reader = BufReader::new(File::open(filename)?).lines().skip(1);

        let coordinates: Vec<_> = buf_reader.by_ref()
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
        let mut nodes = vec![vec![None; point_count]; point_count];

        // the EDGES line has already been iterated over
        for line in buf_reader {
            let line = line.expect("Unable to read line");
            let caps = connection_re.captures(&line).unwrap();
            let (i, j) = (
                caps[1].parse::<usize>().unwrap(),
                caps[2].parse::<usize>().unwrap(),
            );
            let dist = euclidean_distance(coordinates[i], coordinates[j]);
            nodes[i][j] = Some(dist);
            nodes[j][i] = Some(dist);
        }
        Ok(Graph { nodes })
    }
}

fn euclidean_distance((a, b): (f64, f64), (x, y): (f64, f64)) -> f64 {
    ((x - a) * (x - a) + (y - b) * (y - b)).sqrt()
}
