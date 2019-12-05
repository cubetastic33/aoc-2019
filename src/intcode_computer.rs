pub struct Computer {
    pub memory: Vec<isize>,
    pub inputs: Vec<isize>,
    pub outputs: Vec<isize>,
}

impl Default for Computer {
    fn default() -> Self {
        Computer {
            memory: Vec::new(),
            inputs: Vec::new(),
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

    pub fn run(&mut self) {
        // A variable to store the current instruction's pointer
        let mut i = 0;
        loop {
            match self.memory[i] % 100 {
                1 => {
                    // Addition instruction
                    let storage_address = self.memory[i + 3] as usize;
                    self.memory[storage_address] = self.get_parameter(i, 1) + self.get_parameter(i, 2);
                    i += 4;
                }
                2 => {
                    // Multiplication instruction
                    let storage_address = self.memory[i + 3] as usize;
                    self.memory[storage_address] = self.get_parameter(i, 1) * self.get_parameter(i, 2);
                    i += 4;
                }
                3 => {
                    // Input instruction
                    let storage_address = self.memory[i + 1] as usize;
                    self.memory[storage_address] = self.inputs.pop().unwrap();
                    i += 2;
                }
                4 => {
                    // Output instruction
                    self.outputs.push(self.memory[if self.memory[i] % 1000 - self.memory[i] % 100 == 0 { self.memory[i + 1] as usize } else { i + 1 }]);
                    i += 2;
                }
                5 => {
                    // Jump-if-true instruction
                    if self.get_parameter(i, 1) != 0 {
                        i = self.get_parameter(i, 2) as usize;
                    } else {
                        i += 3;
                    }
                }
                6 => {
                    // Jump-if-false instruction
                    if self.get_parameter(i, 1) == 0 {
                        i = self.get_parameter(i, 2) as usize;
                    } else {
                        i += 3;
                    }
                }
                7 => {
                    // Less than instruction
                    let storage_address = self.memory[i + 3] as usize;
                    self.memory[storage_address] = if self.get_parameter(i, 1) < self.get_parameter(i, 2) { 1 } else { 0 };
                    i += 4;
                }
                8 => {
                    // Equals instruction
                    let storage_address = self.memory[i + 3] as usize;
                    self.memory[storage_address] = if self.get_parameter(i, 1) == self.get_parameter(i, 2) { 1 } else { 0 };
                    i += 4;
                }
                _ => {
                    // The opcode must to be 99, so halt
                    break;
                }
            }
        }
    }
}
