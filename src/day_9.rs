use super::intcode_computer::Computer;
use std::collections::VecDeque;

// Solver function for part 1
pub fn get_boost_keycode(intcode: Vec<i64>) -> i64 {
    let mut computer = Computer {
        memory: intcode.to_vec(),
        inputs: VecDeque::from(vec![1]),
        ..Default::default()
    };
    computer.run();
    computer.outputs[computer.outputs.len() - 1]
}

// Solver function for part 2
pub fn get_coordinates_of_distress_signal(intcode: Vec<i64>) -> i64 {
    let mut computer = Computer {
        memory: intcode.to_vec(),
        inputs: VecDeque::from(vec![2]),
        ..Default::default()
    };
    computer.run();
    computer.outputs[computer.outputs.len() - 1]
}
