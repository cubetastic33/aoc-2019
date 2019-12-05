#[aoc_generator(day4)]
pub fn input_generator(range: &str) -> Vec<usize> {
    range.split('-').flat_map(|n| n.parse()).collect()
}

#[aoc(day4, part1)]
pub fn number_of_passwords(range: &[usize]) -> usize {
    let mut n = 0;
    'passwords: for password in range[0]..range[1] {
        let mut previous_digit = 0;
        let mut double_existed = false;
        for digit in password.to_string().bytes() {
            if digit > previous_digit {
                previous_digit = digit;
            } else if digit == previous_digit {
                double_existed = true;
            } else {
                continue 'passwords;
            }
        }
        if double_existed {
            n += 1;
        }
    }
    n
}

#[aoc(day4, part2)]
pub fn more_filtered_number_of_passwords(range: &[usize]) -> usize {
    let mut n = 0;
    'passwords: for password in range[0]..range[1] {
        let mut previous_digit = 0;
        let mut occurrences = [0; 10];
        for digit in password.to_string().bytes() {
            if digit < previous_digit {
                continue 'passwords;
            }
            previous_digit = digit;
            occurrences[digit as usize - 48] += 1;
        }
        if occurrences.contains(&2) {
            n += 1;
        }
    }
    n
}
