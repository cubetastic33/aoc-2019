use aoc_2019::*;

// Test for the third star
#[test]
fn star_3() {
    let mut intcode: Vec<usize> = read_lines_from_file("inputs/day_2.txt")[0].split(',').map(|x| x.parse().unwrap()).collect();
    intcode[1] = 12;
    intcode[2] = 2;
    assert_eq!(day_2::intcode_computer(intcode), 3267740);
}

// Test for the fourth star
#[test]
fn star_4() {
    let intcode: Vec<usize> = read_lines_from_file("inputs/day_2.txt")[0].split(',').map(|x| x.parse().unwrap()).collect();
    assert_eq!(day_2::gravity_assist(intcode), 7870);
}
