use aoc_2019::*;

// Test for the first star
#[test]
fn star_1() {
    let masses = read_lines_from_file("inputs/day_1.txt");
    assert_eq!(3427947, day_1::calc_fuel_of_modules(masses));
}

// Test for the second star
#[test]
fn star_2() {
    let masses = read_lines_from_file("inputs/day_1.txt");
    assert_eq!(5139037, day_1::calc_fuel_of_modules_with_fuel(masses));
}
