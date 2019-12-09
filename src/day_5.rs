use super::intcode_computer::Computer;
use std::collections::VecDeque;

#[aoc_generator(day5)]
pub fn input_generator(intcode: &str) -> Vec<i64> {
    intcode.split(',').flat_map(|n| n.parse()).collect()
}

#[aoc(day5, part1)]
pub fn test_air_conditioner(intcode: &[i64]) -> i64 {
    let mut computer = Computer {
        memory: intcode.to_vec(),
        inputs: VecDeque::from(vec![1]),
        ..Default::default()
    };
    computer.run();
    computer.outputs[computer.outputs.len() - 1]
}

#[aoc(day5, part2)]
pub fn test_thermal_radiator_controller(intcode: &[i64]) -> i64 {
    let mut computer = Computer {
        memory: intcode.to_vec(),
        inputs: VecDeque::from(vec![5]),
        ..Default::default()
    };
    computer.run();
    computer.outputs[computer.outputs.len() - 1]
}
