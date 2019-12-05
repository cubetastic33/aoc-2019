use super::intcode_computer::Computer;

#[aoc_generator(day2)]
pub fn input_generator(intcode: &str) -> Vec<isize> {
    intcode.split(',').flat_map(|n| n.parse()).collect()
}

#[aoc(day2, part1)]
pub fn restore_program_state(intcode: &[isize]) -> usize {
    let mut memory = intcode.to_vec();
    memory[1] = 12;
    memory[2] = 2;
    let mut computer = Computer {
        memory,
        ..Default::default()
    };
    computer.run();
    computer.memory[0] as usize
}

#[aoc(day2, part2)]
pub fn gravity_assist(initial_intcode: &[isize]) -> usize {
    let mut computer = Computer {
        memory: initial_intcode.to_vec(),
        ..Default::default()
    };
    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode = initial_intcode.to_vec();
            intcode[1] = noun;
            intcode[2] = verb;
            computer.memory = intcode;
            computer.run();
            if computer.memory[0] == 19690720 {
                return (100 * noun + verb) as usize;
            }
        }
    }
    unreachable!()
}
