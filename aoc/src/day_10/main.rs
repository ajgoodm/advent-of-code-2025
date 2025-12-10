use std::collections::{HashMap, HashSet};

use utils::{shortest_path_length, AocBufReader, DijkstraSearchable};

fn main() {
    println!(
        "part 1: {}",
        part_1(AocBufReader::from_string("aoc/src/day_10/data/part_1.txt"))
    );
    println!(
        "part 2: {}",
        part_2(AocBufReader::from_string("aoc/src/day_10/data/part_1.txt"))
    );
}

fn part_1(iter: impl Iterator<Item = String>) -> usize {
    iter.map(|line| {
        let (target, _, machine) = Machine::from_string(line);
        let mut targets: HashSet<Vec<char>> = HashSet::new();
        let target_len = target.len();
        targets.insert(target);

        let start: Vec<char> = (0..target_len).map(|_| '.').collect();
        shortest_path_length(machine, start, targets).unwrap()
    })
    .sum()
}

fn part_2(iter: impl Iterator<Item = String>) -> usize {
    iter.map(|line| {
        println!("starting: {}", line);
        let mut cache = HashMap::new();
        let (_, target, machine) = Machine::from_string(line);
        let target_len = target.len();
        let start: Vec<usize> = (0..target_len).map(|_| 0).collect();
        let joltage_machine = JoltageMachine::from_part_1_machine(machine, target);
        depth_first_search(&joltage_machine, start, 0, &mut cache).unwrap()
    })
    .sum()
}

#[derive(Debug)]
struct Machine {
    buttons: Vec<Vec<usize>>,
}

impl Machine {
    fn from_string(s: String) -> (Vec<char>, Vec<usize>, Self) {
        let mut split = s.split_whitespace();
        let indicator_lights: Vec<char> = split
            .next()
            .map(|x| x[1..(x.len() - 1)].chars().collect())
            .unwrap();

        let mut buttons: Vec<Vec<usize>> = vec![];
        let mut joltage: Vec<usize> = vec![];
        for string in split {
            match string.chars().next().unwrap() {
                '(' => buttons.push(
                    string[1..(string.len() - 1)]
                        .split(',')
                        .map(|x| x.parse().unwrap())
                        .collect(),
                ),
                '{' => {
                    joltage = string[1..(string.len() - 1)]
                        .split(',')
                        .map(|x| x.parse().unwrap())
                        .collect()
                }
                _ => panic!(),
            }
        }

        (indicator_lights, joltage, Self { buttons })
    }

    fn push_button(&self, lights: &[char], button_idx: usize) -> Vec<char> {
        let mut result = lights.to_owned();
        for light_idx in self.buttons[button_idx].iter() {
            let new_char = match lights[*light_idx] {
                '.' => '#',
                '#' => '.',
                _ => panic!(),
            };
            result[*light_idx] = new_char;
        }

        result
    }
}

impl DijkstraSearchable for Machine {
    type Node = Vec<char>;
    type Cost = usize;

    fn neighbors(
        &self,
        previous: &Self::Node,
        previous_cost: Self::Cost,
    ) -> Vec<(Self::Node, Self::Cost)> {
        (0..(self.buttons.len()))
            .map(|button_idx| (self.push_button(previous, button_idx), previous_cost + 1))
            .collect()
    }
}

struct JoltageMachine {
    buttons: Vec<Vec<usize>>,
    target: Vec<usize>,
}

impl JoltageMachine {
    fn n_buttons(&self) -> usize {
        self.buttons.len()
    }

    fn push_button(&self, joltage: &[usize], button_idx: usize) -> Vec<usize> {
        let mut result = joltage.to_owned();
        for joltage_idx in self.buttons[button_idx].iter() {
            result[*joltage_idx] += 1;
        }

        result
    }

    fn target_is_reachable_from(&self, start: &[usize]) -> bool {
        self.target.iter().zip(start.iter()).all(|(t, v)| t >= v)
    }

    fn from_part_1_machine(machine: Machine, target: Vec<usize>) -> Self {
        let mut buttons = machine.buttons;
        buttons.sort_by_key(|x| x.iter().sum::<usize>());
        buttons.reverse(); // sort from longest to shortest
        Self { buttons, target }
    }
}

fn depth_first_search(
    machine: &JoltageMachine,
    start: Vec<usize>,
    cost_to_reach_start: usize,
    cache: &mut HashMap<Vec<usize>, Option<usize>>,
) -> Option<usize> {
    if start == machine.target {
        return Some(cost_to_reach_start);
    }

    if let Some(val) = cache.get(&start) {
        return *val;
    }

    let result = (0..machine.n_buttons())
        .map(|button_idx| machine.push_button(&start, button_idx))
        .filter(|next| machine.target_is_reachable_from(next))
        .filter_map(|next| depth_first_search(machine, next, cost_to_reach_start + 1, cache))
        .min();

    cache.insert(start, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_bits() {
        let (lights, _, machine) =
            Machine::from_string("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string());
        let next = machine.push_button(&lights, 1);
        assert_eq!(next, "..##".chars().collect::<Vec<char>>())
    }

    #[test]
    fn part_1_djikstra() {
        let (target, _, machine) =
            Machine::from_string("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string());
        let mut targets: HashSet<Vec<char>> = HashSet::new();
        targets.insert(target);

        assert_eq!(
            shortest_path_length(machine, Vec::from_iter("....".chars()), targets),
            Some(2)
        )
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                [
                    "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
                    "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
                    "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
                ]
                .into_iter()
                .map(|x| x.to_string())
            ),
            33
        )
    }
}
