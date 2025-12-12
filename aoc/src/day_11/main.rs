use std::collections::HashMap;

use utils::AocBufReader;

fn main() {
    let server_rack =
        ServerRack::from_lines(AocBufReader::from_string("aoc/src/day_11/data/part_1.txt"));
    println!("part 1: {}", part_1(&server_rack));
    println!("part 2: {}", part_2(&server_rack));
}

fn part_1(server_rack: &ServerRack) -> usize {
    server_rack.n_paths("you", "end", &mut HashMap::new())
}

fn part_2(server_rack: &ServerRack) -> usize {
    let mut cache = HashMap::new();
    server_rack.n_paths("svr", "fft", &mut cache)
        * server_rack.n_paths("fft", "dac", &mut cache)
        * server_rack.n_paths("dac", "out", &mut cache)
}

#[derive(Debug)]
struct ServerRack {
    graph: HashMap<String, Vec<String>>,
}

impl ServerRack {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut graph = HashMap::new();
        for line in lines {
            let mut key_vals = line.split(": ");
            let key = key_vals.next().unwrap().to_owned();
            let vals: Vec<String> = key_vals
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.to_owned())
                .collect();
            graph.insert(key, vals);
        }

        Self { graph }
    }

    fn neighbors(&self, node: &str) -> &Vec<String> {
        self.graph.get(node).unwrap()
    }

    fn n_paths(
        &self,
        start: &str,
        end: &str,
        cache: &mut HashMap<(String, String), usize>,
    ) -> usize {
        if let Some(cached) = cache.get(&(start.to_owned(), end.to_owned())) {
            return *cached;
        }

        let neighbors = self.neighbors(start);
        let result = neighbors
            .iter()
            .map(|next| {
                if next == end {
                    1
                } else if next == "out" {
                    0
                } else {
                    self.n_paths(next, end, cache)
                }
            })
            .sum();

        cache.insert((start.to_owned(), end.to_owned()), result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_lines() {
        let server_rack = ServerRack::from_lines(
            [
                "aaa: you hhh",
                "you: bbb ccc",
                "bbb: ddd eee",
                "ccc: ddd eee fff",
                "ddd: ggg",
                "eee: out",
                "fff: out",
                "ggg: out",
                "hhh: ccc fff iii",
                "iii: out",
            ]
            .into_iter()
            .map(|x| x.to_string()),
        );
        assert_eq!(server_rack.n_paths("you", "out"), 5);
    }
}
