use std::collections::{HashMap, HashSet};

use utils::{pop_set, shortest_path_length, AocBufReader, DijkstraSearchable};

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

/// For the first button, iterate through its possible values
/// For each value, iterate through the second button's possible values
/// and so on...
fn part_2_inner(
    machine: Machine,
    target: Vec<usize>,
    cache: &mut HashMap<Vec<usize>, Option<usize>>,
) -> Option<usize> {
    if target.iter().all(|x| *x == 0) {
        return Some(0);
    }

    if let Some(val) = cache.get(&target) {
        return *val;
    }

    if machine.n_buttons() == 1 {
        let (_empty_machine, last_button) = machine.pop_button().unwrap();

        let mut values_at_button_idxs: HashSet<usize> = HashSet::new();
        let mut values_at_not_button_idxs: HashSet<usize> = HashSet::new();
        for (joltage_idx, target_value) in target.iter().enumerate() {
            if last_button.contains(&joltage_idx) {
                values_at_button_idxs.insert(*target_value);
            } else {
                values_at_not_button_idxs.insert(*target_value);
            }
        }

        if values_at_not_button_idxs.len() > 1 {
            return None;
        }
        if values_at_not_button_idxs.len() == 1
            && pop_set(&mut values_at_not_button_idxs) != Some(0)
        {
            return None;
        }

        if values_at_button_idxs.len() != 1 {
            return None;
        }
        return pop_set(&mut values_at_button_idxs);
    }

    let (smaller_machine, button) = machine.pop_button().unwrap();
    let max_presses = button
        .iter()
        .map(|joltage_idx| target[*joltage_idx])
        .min()
        .unwrap();

    let result = (0..=max_presses)
        .filter_map(|n_presses| {
            let mut after_presses = target.clone();
            for joltage_idx in button.iter() {
                after_presses[*joltage_idx] -= n_presses;
            }
            part_2_inner(smaller_machine.clone(), after_presses, cache).map(|val| val + n_presses)
        })
        .min();

    cache.insert(target, result);
    result
}

/// Sigh this is a sneaky linear system optimization problem, and most folks
/// on the internet have used solvers in their language of choice. I'll
/// try to find a Rust solver and see if I can use it here. I've re-coded
/// different flavors of this solution like 4 times and they all time-out
/// on the list of data
fn part_2(iter: impl Iterator<Item = String>) -> usize {
    let mut cache = HashMap::new();
    iter.map(|line| {
        println!("{}", line);
        let (_, target, machine) = Machine::from_string(line);
        part_2_inner(machine, target, &mut cache).unwrap()
    })
    .sum()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    fn n_buttons(&self) -> usize {
        self.buttons.len()
    }

    fn pop_button(&self) -> Option<(Self, Vec<usize>)> {
        let mut buttons = self.buttons.clone();
        buttons.pop().map(|last| (Self { buttons }, last))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_bits() {
        let (lights, _, machine) =
            Machine::from_string("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string());
        let next = machine.push_button(&lights, 1);
        assert_eq!(next, "..##".chars().collect::<Vec<char>>())
    }

    #[test]
    fn test_part_1_djikstra() {
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
    fn test_part_2_inner() {
        let mut cache = HashMap::new();
        let (_, target, machine) =
            Machine::from_string("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string());
        assert_eq!(part_2_inner(machine, target, &mut cache), Some(10));
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
