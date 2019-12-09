use super::intcode_computer::Computer;
use std::collections::VecDeque;

#[aoc_generator(day9)]
pub fn input_generator(intcode: &str) -> Vec<i64> {
    intcode.split(',').flat_map(|n| n.parse()).collect()
}

#[aoc(day9, part1)]
pub fn get_boost_keycode(intcode: &[i64]) -> i64 {
    let mut computer = Computer {
        memory: intcode.to_vec(),
        inputs: VecDeque::from(vec![1]),
        ..Default::default()
    };
    computer.run();
    computer.outputs[computer.outputs.len() - 1]
}

#[aoc(day9, part2)]
pub fn get_coordinates_of_distress_signal(intcode: &[i64]) -> i64 {
    let mut computer = Computer {
        memory: intcode.to_vec(),
        inputs: VecDeque::from(vec![2]),
        ..Default::default()
    };
    computer.run();
    computer.outputs[computer.outputs.len() - 1]
}
