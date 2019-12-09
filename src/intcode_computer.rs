use std::collections::VecDeque;

pub struct Computer {
    pub memory: Vec<i64>,
    pub instruction_pointer: usize,
    pub relative_base: i64,
    pub inputs: VecDeque<i64>,
    pub outputs: Vec<i64>,
}

impl Default for Computer {
    fn default() -> Self {
        Computer {
            memory: Vec::new(),
            instruction_pointer: 0,
            relative_base: 0,
            inputs: VecDeque::new(),
            outputs: Vec::new(),
        }
    }
}

impl Computer {
    fn get_parameter(&mut self, pointer: usize, n: usize) -> i64 {
        match self.memory[pointer] / 10_i64.pow(n as u32 + 1) % 10 {
            // Position mode
            0 => {
                let address = self.read_from_memory(pointer + n) as usize;
                self.read_from_memory(address)
            }
            // Immediate mode
            1 => self.read_from_memory(pointer + n),
            // Relative mode
            2 => {
                let relative_mode_parameter = self.read_from_memory(pointer + n);
                self.read_from_memory((self.relative_base + relative_mode_parameter) as usize)
            }
            _ => unreachable!()
        }
    }

    fn get_writable_parameter(&mut self, pointer: usize, n: usize) -> usize {
        (match self.memory[pointer] / 10_i64.pow(n as u32 + 1) % 10 {
            // Position mode
            0 => {
                self.read_from_memory(pointer + n)
            }
            // Relative mode
            2 => {
                let relative_mode_parameter = self.read_from_memory(pointer + n);
                self.relative_base + relative_mode_parameter
            }
            _ => unreachable!()
        }) as usize
    }

    pub fn write_to_address(&mut self, address: usize, val: i64) {
        if address >= self.memory.len() {
            self.memory.append(&mut vec![0; address - self.memory.len() + 1]);
        }
        self.memory[address] = val;
    }

    pub fn read_from_memory(&mut self, address: usize) -> i64 {
        if address >= self.memory.len() {
            self.memory.append(&mut vec![0; address - self.memory.len() + 1]);
        }
        self.memory[address]
    }

    pub fn run(&mut self) -> bool {
        loop {
            match self.memory[self.instruction_pointer] % 100 {
                1 => {
                    // Addition instruction
                    let storage_address = self.get_writable_parameter(self.instruction_pointer, 3);
                    let val = self.get_parameter(self.instruction_pointer, 1) + self.get_parameter(self.instruction_pointer, 2);
                    self.write_to_address(storage_address, val);
                    self.instruction_pointer += 4;
                }
                2 => {
                    // Multiplication instruction
                    let storage_address = self.get_writable_parameter(self.instruction_pointer, 3);
                    let val = self.get_parameter(self.instruction_pointer, 1) * self.get_parameter(self.instruction_pointer, 2);
                    self.write_to_address(storage_address, val);
                    self.instruction_pointer += 4;
                }
                3 => {
                    // Input instruction
                    let storage_address = self.get_writable_parameter(self.instruction_pointer, 1);
                    match self.inputs.pop_front() {
                        Some(input) => self.write_to_address(storage_address, input),
                        None => return false,
                    }
                    self.instruction_pointer += 2;
                }
                4 => {
                    // Output instruction
                    let val = self.get_parameter(self.instruction_pointer, 1);
                    self.outputs.push(val);
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
                    let storage_address = self.get_writable_parameter(self.instruction_pointer, 3);
                    let val = if self.get_parameter(self.instruction_pointer, 1) < self.get_parameter(self.instruction_pointer, 2) { 1 } else { 0 };
                    self.write_to_address(storage_address, val);
                    self.instruction_pointer += 4;
                }
                8 => {
                    // Equals instruction
                    let storage_address = self.get_writable_parameter(self.instruction_pointer, 3);
                    let val = if self.get_parameter(self.instruction_pointer, 1) == self.get_parameter(self.instruction_pointer, 2) { 1 } else { 0 };
                    self.write_to_address(storage_address, val);
                    self.instruction_pointer += 4;
                }
                9 => {
                    // Relative base offset instruction
                    self.relative_base += self.get_parameter(self.instruction_pointer, 1);
                    self.instruction_pointer += 2;
                }
                _ => {
                    // The opcode must to be 99, so halt
                    return true;
                }
            }
        }
    }
}
