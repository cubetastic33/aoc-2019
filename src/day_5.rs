use std::io;
use std::io::Write;

#[aoc_generator(day5)]
pub fn input_generator(intcode: &str) -> Vec<isize> {
    intcode.split(',').flat_map(|n| n.parse()).collect()
}

fn intcode_computer(mut memory: Vec<isize>) -> usize {
    // A variable to store the current instruction's pointer
    let mut i = 0;
    let mut diagnostic_code = 0;
    while i < memory.len() {
        if memory[i] % 100 == 1 {
            // Addition instruction
            // This parameter is guaranteed to always be in position mode
            let storage_address = memory[i + 3] as usize;
            let parameter_1 = if memory[i] % 1000 - memory[i] % 100 == 0 {
                // Position mode
                memory[memory[i + 1] as usize]
            } else {
                // Immediate mode
                memory[i + 1]
            };
            let parameter_2 = if memory[i] % 10000 - memory[i] % 1000 == 0 {
                // Position mode
                memory[memory[i + 2] as usize]
            } else {
                // Immediate mode
                memory[i + 2]
            };
            memory[storage_address] = parameter_1 + parameter_2;
            if storage_address != i {
                i += 4;
            }
        } else if memory[i] % 100 == 2 {
            // Multiplication instruction
            // This parameter is guaranteed to always be in position mode
            let storage_address = memory[i + 3] as usize;
            let parameter_1 = if memory[i] % 1000 - memory[i] % 100 == 0 {
                // Position mode
                memory[memory[i + 1] as usize]
            } else {
                // Immediate mode
                memory[i + 1]
            };
            let parameter_2 = if memory[i] % 10000 - memory[i] % 1000 == 0 {
                // Position mode
                memory[memory[i + 2] as usize]
            } else {
                // Immediate mode
                memory[i + 2]
            };
            memory[storage_address] = parameter_1 * parameter_2;
            if storage_address != i {
                i += 4;
            }
        } else if memory[i] % 100 == 3 {
            // Input instruction
            let storage_address = memory[i + 1] as usize;
            let mut input = String::new();
            print!("Enter the ID of the system to test: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut input)
                .expect("Error: failed to read from stdin");
            memory[storage_address] = input.trim().parse().unwrap();
            if storage_address != i {
                i += 2;
            }
        } else if memory[i] % 100 == 4 {
            // Output instruction
            println!("{}", memory[memory[i + 1] as usize]);
            diagnostic_code = memory[memory[i + 1] as usize] as usize;
            i += 2;
        } else if memory[i] % 100 == 5 {
            // Jump-if-true instruction
            let parameter_1 = if memory[i] % 1000 - memory[i] % 100 == 0 {
                // Position mode
                memory[memory[i + 1] as usize]
            } else {
                // Immediate mode
                memory[i + 1]
            };
            let parameter_2 = if memory[i] % 10000 - memory[i] % 1000 == 0 {
                // Position mode
                memory[memory[i + 2] as usize]
            } else {
                // Immediate mode
                memory[i + 2]
            } as usize;
            if parameter_1 != 0 {
                i = parameter_2;
            } else {
                i += 3;
            }
        } else if memory[i] % 100 == 6 {
            // Jump-if-false instruction
            let parameter_1 = if memory[i] % 1000 - memory[i] % 100 == 0 {
                // Position mode
                memory[memory[i + 1] as usize]
            } else {
                // Immediate mode
                memory[i + 1]
            };
            let parameter_2 = if memory[i] % 10000 - memory[i] % 1000 == 0 {
                // Position mode
                memory[memory[i + 2] as usize]
            } else {
                // Immediate mode
                memory[i + 2]
            } as usize;
            if parameter_1 == 0 {
                i = parameter_2;
            } else {
                i += 3;
            }
        } else if memory[i] % 100 == 7 {
            // Less than instruction
            let storage_address = memory[i + 3] as usize;
            let parameter_1 = if memory[i] % 1000 - memory[i] % 100 == 0 {
                // Position mode
                memory[memory[i + 1] as usize]
            } else {
                // Immediate mode
                memory[i + 1]
            };
            let parameter_2 = if memory[i] % 10000 - memory[i] % 1000 == 0 {
                // Position mode
                memory[memory[i + 2] as usize]
            } else {
                // Immediate mode
                memory[i + 2]
            };
            memory[storage_address] = if parameter_1 < parameter_2 { 1 } else { 0 };
            if storage_address != i {
                i += 4;
            }
        } else if memory[i] % 100 == 8 {
            // Equals instruction
            let storage_address = memory[i + 3] as usize;
            let parameter_1 = if memory[i] % 1000 - memory[i] % 100 == 0 {
                // Position mode
                memory[memory[i + 1] as usize]
            } else {
                // Immediate mode
                memory[i + 1]
            };
            let parameter_2 = if memory[i] % 10000 - memory[i] % 1000 == 0 {
                // Position mode
                memory[memory[i + 2] as usize]
            } else {
                // Immediate mode
                memory[i + 2]
            };
            memory[storage_address] = if parameter_1 == parameter_2 { 1 } else { 0 };
            if storage_address != i {
                i += 4;
            }
        } else {
            // The opcode must to be 99, so halt
            break;
        }
    }
    diagnostic_code
}

#[aoc(day5, part1)]
pub fn test_air_conditioner(intcode: &Vec<isize>) -> usize {
    intcode_computer(intcode.clone())
}

#[aoc(day5, part2)]
pub fn test_thermal_radiator_controller(intcode: &Vec<isize>) -> usize {
    intcode_computer(intcode.clone())
}
