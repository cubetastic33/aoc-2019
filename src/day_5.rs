use super::intcode_computer::Computer;

#[aoc_generator(day5)]
pub fn input_generator(intcode: &str) -> Vec<isize> {
    intcode.split(',').flat_map(|n| n.parse()).collect()
}

#[aoc(day5, part1)]
pub fn test_air_conditioner(intcode: &[isize]) -> isize {
    let mut computer = Computer {
        memory: intcode.to_vec(),
        inputs: vec![1],
        ..Default::default()
    };
    computer.run();
    computer.outputs[computer.outputs.len() - 1]
}

#[aoc(day5, part2)]
pub fn test_thermal_radiator_controller(intcode: &[isize]) -> isize {
    let mut computer = Computer {
        memory: intcode.to_vec(),
        inputs: vec![5],
        ..Default::default()
    };
    computer.run();
    computer.outputs[computer.outputs.len() - 1]
}
