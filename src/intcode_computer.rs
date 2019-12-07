use std::collections::VecDeque;

pub struct Computer {
    pub memory: Vec<isize>,
    pub instruction_pointer: usize,
    pub inputs: VecDeque<isize>,
    pub outputs: Vec<isize>,
}

impl Default for Computer {
    fn default() -> Self {
        Computer {
            memory: Vec::new(),
            instruction_pointer: 0,
            inputs: VecDeque::new(),
            outputs: Vec::new(),
        }
    }
}

impl Computer {
    fn get_parameter(&self, pointer: usize, n: usize) -> isize {
        match self.memory[pointer] / 10_isize.pow(n as u32 + 1) % 10 {
            // Position mode
            0 => self.memory[self.memory[pointer + n] as usize],
            // Immediate mode
            _ => self.memory[pointer + n],
        }
    }

    pub fn run(&mut self) -> bool {
        loop {
            match self.memory[self.instruction_pointer] % 100 {
                1 => {
                    // Addition instruction
                    let storage_address = self.memory[self.instruction_pointer + 3] as usize;
                    self.memory[storage_address] = self.get_parameter(self.instruction_pointer, 1) + self.get_parameter(self.instruction_pointer, 2);
                    self.instruction_pointer += 4;
                }
                2 => {
                    // Multiplication instruction
                    let storage_address = self.memory[self.instruction_pointer + 3] as usize;
                    self.memory[storage_address] = self.get_parameter(self.instruction_pointer, 1) * self.get_parameter(self.instruction_pointer, 2);
                    self.instruction_pointer += 4;
                }
                3 => {
                    // Input instruction
                    let storage_address = self.memory[self.instruction_pointer + 1] as usize;
                    match self.inputs.pop_front() {
                        Some(input) => self.memory[storage_address] = input,
                        None => return false,
                    }
                    self.instruction_pointer += 2;
                }
                4 => {
                    // Output instruction
                    self.outputs.push(self.get_parameter(self.instruction_pointer, 1));
                    self.instruction_pointer += 2;
                }
                5 => {
                    // Jump-if-true instruction
                    if self.get_parameter(self.instruction_pointer, 1) != 0 {
                        self.instruction_pointer = self.get_parameter(self.instruction_pointer, 2) as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                6 => {
                    // Jump-if-false instruction
                    if self.get_parameter(self.instruction_pointer, 1) == 0 {
                        self.instruction_pointer = self.get_parameter(self.instruction_pointer, 2) as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                7 => {
                    // Less than instruction
                    let storage_address = self.memory[self.instruction_pointer + 3] as usize;
                    self.memory[storage_address] = if self.get_parameter(self.instruction_pointer, 1) < self.get_parameter(self.instruction_pointer, 2) { 1 } else { 0 };
                    self.instruction_pointer += 4;
                }
                8 => {
                    // Equals instruction
                    let storage_address = self.memory[self.instruction_pointer + 3] as usize;
                    self.memory[storage_address] = if self.get_parameter(self.instruction_pointer, 1) == self.get_parameter(self.instruction_pointer, 2) { 1 } else { 0 };
                    self.instruction_pointer += 4;
                }
                _ => {
                    // The opcode must to be 99, so halt
                    return true;
                }
            }
        }
    }
}
