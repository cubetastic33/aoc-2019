use super::intcode_computer::Computer;
use std::collections::VecDeque;
use itertools::Itertools;

#[aoc_generator(day7)]
pub fn input_generator(intcode: &str) -> Vec<isize> {
    intcode.split(',').flat_map(|n| n.parse()).collect()
}

#[aoc(day7, part1)]
pub fn find_highest_signal(intcode: &[isize]) -> isize {
    let mut highest_signal = 0;
    for permutation in (0..5).permutations(5) {
        let mut next_input = 0;
        for phase_setting in permutation {
            let mut computer = Computer {
                memory: intcode.to_vec(),
                ..Default::default()
            };
            computer.inputs = VecDeque::from(vec![phase_setting, next_input]);
            computer.run();
            next_input = computer.outputs[0];
        }
        if next_input > highest_signal {
            highest_signal = next_input;
        }
    }
    highest_signal
}

#[aoc(day7, part2)]
pub fn find_highest_signal_with_feedback_loop(intcode: &[isize]) -> isize {
    let mut highest_signal = 0;
    for permutation in (5..10).permutations(5) {
        let mut amplifiers = [
            Computer {
                memory: intcode.to_vec(),
                inputs: VecDeque::from(vec![permutation[0]]),
                ..Default::default()
            },
            Computer {
                memory: intcode.to_vec(),
                inputs: VecDeque::from(vec![permutation[1]]),
                ..Default::default()
            },
            Computer {
                memory: intcode.to_vec(),
                inputs: VecDeque::from(vec![permutation[2]]),
                ..Default::default()
            },
            Computer {
                memory: intcode.to_vec(),
                inputs: VecDeque::from(vec![permutation[3]]),
                ..Default::default()
            },
            Computer {
                memory: intcode.to_vec(),
                inputs: VecDeque::from(vec![permutation[4]]),
                ..Default::default()
            },
        ];
        let mut next_input = 0;
        let mut amplifier_no = 0;
        loop {
            amplifiers[amplifier_no % 5].inputs.push_back(next_input);
            if amplifiers[amplifier_no % 5].run() && amplifier_no % 5 == 4 {
                let thruster_input = amplifiers[amplifier_no % 5].outputs[amplifiers[amplifier_no % 5].outputs.len() - 1];
                if thruster_input > highest_signal {
                    highest_signal = thruster_input;
                }
                break;
            }
            next_input = amplifiers[amplifier_no % 5].outputs[amplifiers[amplifier_no % 5].outputs.len() - 1];
            amplifier_no += 1;
        }
    }
    highest_signal
}
