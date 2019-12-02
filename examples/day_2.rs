use aoc_2019::*;

fn main() {
    let mut intcode: Vec<usize> = read_lines_from_file("inputs/day_2.txt")[0].split(',').map(|x| x.parse().unwrap()).collect();
    intcode[1] = 12;
    intcode[2] = 2;
    println!("Answer for the first part: {}", day_2::intcode_computer(intcode));

    let intcode: Vec<usize> = read_lines_from_file("inputs/day_2.txt")[0].split(',').map(|x| x.parse().unwrap()).collect();
    println!("Answer for the second part: {}", day_2::gravity_assist(intcode));
}