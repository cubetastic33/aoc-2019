use aoc_2019::*;

fn main() {
    let masses: Vec<_> = read_lines_from_file("inputs/day_1.txt").iter().map(|x| x.parse().unwrap()).collect();
    println!("Answer for the first part: {}", day_1::calc_fuel_of_modules(masses.clone()));

    println!("Answer for the second part: {}", day_1::calc_fuel_of_modules_with_fuel(masses));
}
