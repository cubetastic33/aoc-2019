pub fn intcode_computer(mut intcode: Vec<usize>) -> usize {
    let mut i = 0;
    while i < intcode.len() - 1 {
        if intcode[i] == 1 {
            // Add the integers specified by the next two integers and store
            // them in the location specified by the integer after them
            let storage_location = intcode[i + 3];
            intcode[storage_location] = intcode[intcode[i + 1]] + intcode[intcode[i + 2]];
        } else if intcode[i] == 2 {
            // Multiply the integers specified by the next two integers and
            // store them in the location specified by the integer after them
            let storage_location = intcode[i + 3];
            intcode[storage_location] = intcode[intcode[i + 1]] * intcode[intcode[i + 2]];
        } else {
            // Make sure it's 99. If it's not, something's wrong
            assert_eq!(intcode[i], 99);
            return intcode[0];
        }
        i += 4;
    }
    intcode[0]
}

pub fn gravity_assist(initial_intcode: Vec<usize>) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode = initial_intcode.clone();
            intcode[1] = noun;
            intcode[2] = verb;
            if intcode_computer(intcode) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    // In reality, we should never get here
    0
}