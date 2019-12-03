use aoc_2019::*;

// Test for the fifth star
#[test]
fn star_5() {
    let wires = read_lines_from_file("inputs/day_3.txt");
    assert_eq!(day_3::distance_from_closest_intersection(wires), 1211);
}

// Test for the sixth star
#[test]
fn star_6() {
    let wires = read_lines_from_file("inputs/day_3.txt");
    assert_eq!(day_3::best_steps(wires), 101386);
}
