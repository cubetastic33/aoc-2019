use aoc_2019::*;

fn main() {
    let wires = read_lines_from_file("inputs/day_3.txt");
    println!("Answer for the first part: {}", day_3::distance_from_closest_intersection(wires.clone()));

    println!("Answer for the second part: {}", day_3::best_steps(wires));
}
