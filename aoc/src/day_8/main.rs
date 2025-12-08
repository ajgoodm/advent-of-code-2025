use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use coord_3d::Coord3D;
use utils::{pop_set, AocBufReader};

fn main() {
    println!(
        "part 1: {}",
        part_1(
            AocBufReader::from_string("aoc/src/day_8/data/part_1.txt")
                .map(|line| Coord3D::from_str(&line).unwrap())
                .collect(),
            1000,
        )
    );
    println!(
        "part 2: {}",
        part_2(
            AocBufReader::from_string("aoc/src/day_8/data/part_1.txt")
                .map(|line| Coord3D::from_str(&line).unwrap())
                .collect()
        )
    );
}

fn part_1(coords: Vec<Coord3D<usize>>, n_connections: usize) -> usize {
    let (christmas_graph, _) = ChristmasGraph::new(coords, n_connections);
    let clique_sizes = christmas_graph.click_sizes();
    clique_sizes[0] * clique_sizes[1] * clique_sizes[2]
}
fn part_2(coords: Vec<Coord3D<usize>>) -> usize {
    // FIX-ME: This runs in about 2-3 minutes
    // We should be able to fix this so we don't re-calculate the first clique every time.
    // But to be honest, we probably won't
    let mut n_connections: usize = 1;
    loop {
        let (christmas_graph, last_edge) = ChristmasGraph::new(coords.clone(), n_connections);
        if christmas_graph.is_one_clique() {
            let (left, right) = last_edge;
            return left.x * right.x;
        }
        n_connections += 1;
    }
}

type Node = Coord3D<usize>;
type Edge = (Coord3D<usize>, Coord3D<usize>);

struct ChristmasGraph {
    nodes: HashSet<Node>,
    nodes_to_edges: HashMap<Node, Vec<Edge>>,
}

impl ChristmasGraph {
    fn new(junction_boxes: Vec<Node>, n_connections: usize) -> (Self, Edge) {
        let nodes = junction_boxes.clone().into_iter().collect::<HashSet<_>>();
        let mut box_pairs: Vec<Edge> = vec![];
        for idx_1 in 0..(junction_boxes.len() - 1) {
            for idx_2 in (idx_1 + 1)..junction_boxes.len() {
                box_pairs.push((junction_boxes[idx_1].clone(), junction_boxes[idx_2].clone()))
            }
        }
        box_pairs.sort_by_key(|(a, b)| a.squared_euclidean_distance(b));

        let mut edges = box_pairs
            .into_iter()
            .take(n_connections)
            .collect::<Vec<Edge>>();
        let mut nodes_to_edges: HashMap<Node, Vec<Edge>> =
            nodes.iter().cloned().map(|node| (node, vec![])).collect();
        for edge in edges.iter() {
            let (left, right) = edge;
            nodes_to_edges.get_mut(left).unwrap().push(edge.clone());
            nodes_to_edges.get_mut(right).unwrap().push(edge.clone());
        }

        (
            Self {
                nodes,
                nodes_to_edges,
            },
            edges.pop().unwrap(),
        )
    }

    fn neighbors(&self, node: &Node) -> Vec<Node> {
        self.nodes_to_edges
            .get(node)
            .unwrap()
            .iter()
            .cloned()
            .fold(vec![], |mut vec, item| {
                let (left, right) = item;
                vec.push(left);
                vec.push(right);
                vec
            })
    }

    fn click_sizes(&self) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        let mut all_nodes = self.nodes.clone();
        while let Some(node) = pop_set(&mut all_nodes) {
            let mut to_visit: Vec<Node> = vec![node];
            let mut clique_nodes: HashSet<Node> = HashSet::new();
            while let Some(next) = to_visit.pop() {
                let new_neighbors = self
                    .neighbors(&next)
                    .into_iter()
                    .filter(|neighbor| !clique_nodes.contains(neighbor))
                    .collect::<Vec<_>>();
                to_visit.extend(new_neighbors);
                clique_nodes.insert(next);
            }
            all_nodes.retain(|x| !clique_nodes.contains(x));
            result.push(clique_nodes.len());
        }

        result.sort_by(|a, b| b.cmp(a));
        result
    }

    fn is_one_clique(&self) -> bool {
        let mut all_nodes = self.nodes.clone();
        let first = pop_set(&mut all_nodes).unwrap();

        let mut to_visit: Vec<Node> = vec![first];
        let mut clique_nodes: HashSet<Node> = HashSet::new();
        while let Some(next) = to_visit.pop() {
            let new_neighbors = self
                .neighbors(&next)
                .into_iter()
                .filter(|neighbor| !clique_nodes.contains(neighbor))
                .collect::<Vec<_>>();
            to_visit.extend(new_neighbors);
            clique_nodes.insert(next);
        }

        clique_nodes == self.nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                [
                    "162,817,812",
                    "57,618,57",
                    "906,360,560",
                    "592,479,940",
                    "352,342,300",
                    "466,668,158",
                    "542,29,236",
                    "431,825,988",
                    "739,650,466",
                    "52,470,668",
                    "216,146,977",
                    "819,987,18",
                    "117,168,530",
                    "805,96,715",
                    "346,949,466",
                    "970,615,88",
                    "941,993,340",
                    "862,61,35",
                    "984,92,344",
                    "425,690,689",
                ]
                .into_iter()
                .map(|x| Coord3D::from_str(x).unwrap())
                .collect(),
                10
            ),
            40
        )
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                [
                    "162,817,812",
                    "57,618,57",
                    "906,360,560",
                    "592,479,940",
                    "352,342,300",
                    "466,668,158",
                    "542,29,236",
                    "431,825,988",
                    "739,650,466",
                    "52,470,668",
                    "216,146,977",
                    "819,987,18",
                    "117,168,530",
                    "805,96,715",
                    "346,949,466",
                    "970,615,88",
                    "941,993,340",
                    "862,61,35",
                    "984,92,344",
                    "425,690,689",
                ]
                .into_iter()
                .map(|x| Coord3D::from_str(x).unwrap())
                .collect()
            ),
            25272
        )
    }
}
