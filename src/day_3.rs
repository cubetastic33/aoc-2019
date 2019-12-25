enum Line {
    Vertical(isize, [isize; 2]),
    Horizontal([isize; 2], isize),
}

fn manhattan_distance(point_a: [isize; 2], point_b: [isize; 2]) -> usize {
    ((point_a[0] - point_b[0]).abs() + (point_a[1] - point_b[1]).abs()) as usize
}

fn find_intersections(wires: &[String]) -> Vec<[isize; 2]> {
    let mut first_wire_ranges = Vec::new();
    let mut intersections = Vec::new();

    for (i, wire) in wires.iter().enumerate() {
        let mut current_coordinates = [0, 0];
        for instruction in wire.split(',').map(|x| x.to_string()) {
            let length: isize = instruction[1..].parse().unwrap();
            match instruction.as_bytes()[0] as char {
                'U' => {
                    if i == 0 {
                        // we're in the first wire
                        first_wire_ranges.push(Line::Vertical(current_coordinates[0], [current_coordinates[1], current_coordinates[1] + length]));
                    } else {
                        // we're in the second wire
                        for range in &first_wire_ranges {
                            match *range {
                                Line::Vertical(x, y_range) => {
                                    if x == current_coordinates[0] {
                                        // Both are segments of the same line
                                        for y in current_coordinates[1] + 1..current_coordinates[1] + length {
                                            if y >= y_range[0] && y <= y_range[1] {
                                                // This point intersects with the first wire
                                                intersections.push([x, y]);
                                            }
                                        }
                                    }
                                }
                                Line::Horizontal(x_range, y) => {
                                    if current_coordinates[0] >= x_range[0] && current_coordinates[0] <= x_range[1] {
                                        if y > current_coordinates[1] && y <= current_coordinates[1] + length {
                                            intersections.push([current_coordinates[0], y]);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    current_coordinates[1] += length;
                }
                'R' => {
                    if i == 0 {
                        // We're in the first wire
                        first_wire_ranges.push(Line::Horizontal([current_coordinates[0], current_coordinates[0] + length], current_coordinates[1]));
                    } else {
                        // We're in the second wire
                        for range in &first_wire_ranges {
                            match *range {
                                Line::Vertical(x, y_range) => {
                                    if x > current_coordinates[0] && x <= current_coordinates[0] + length {
                                        if current_coordinates[1] >= y_range[0] && current_coordinates[1] <= y_range[1] {
                                            intersections.push([x, current_coordinates[1]]);
                                        }
                                    }
                                }
                                Line::Horizontal(x_range, y) => {
                                    if y == current_coordinates[1] {
                                        // Both are segments of the same line
                                        for x in current_coordinates[0] + 1..current_coordinates[0] + length {
                                            if x >= x_range[0] && x <= x_range[1] {
                                                // This point intersects with the first wire
                                                intersections.push([x, y]);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    current_coordinates[0] += length;
                }
                'D' => {
                    if i == 0 {
                        // We're in the first wire
                        first_wire_ranges.push(Line::Vertical(current_coordinates[0], [current_coordinates[1] - length, current_coordinates[1]]));
                    } else {
                        // We're in the second wire
                        for range in &first_wire_ranges {
                            match *range {
                                Line::Vertical(x, y_range) => {
                                    if x == current_coordinates[0] {
                                        // Both are segments of the same line
                                        for y in current_coordinates[1] - length..current_coordinates[1] {
                                            if y >= y_range[0] && y <= y_range[1] {
                                                // This point intersects with the first wire
                                                intersections.push([x, y]);
                                            }
                                        }
                                    }
                                }
                                Line::Horizontal(x_range, y) => {
                                    if current_coordinates[0] >= x_range[0] && current_coordinates[0] <= x_range[1] {
                                        if y >= current_coordinates[1] - length && y < current_coordinates[1] {
                                            intersections.push([current_coordinates[0], y]);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    current_coordinates[1] -= length;
                }
                'L' => {
                    if i == 0 {
                        // We're in the first wire
                        first_wire_ranges.push(Line::Horizontal([current_coordinates[0] - length, current_coordinates[0] + 1], current_coordinates[1]));
                    } else {
                        // We're in the second wire
                        for range in &first_wire_ranges {
                            match *range {
                                Line::Vertical(x, y_range) => {
                                    if x >= current_coordinates[0] - length && x < current_coordinates[0] {
                                        if current_coordinates[1] >= y_range[0] && current_coordinates[1] <= y_range[1] {
                                            intersections.push([x, current_coordinates[1]]);
                                        }
                                    }
                                }
                                Line::Horizontal(x_range, y) => {
                                    if y == current_coordinates[1] {
                                        // Both are segments of the same line
                                        for x in current_coordinates[0] - length..current_coordinates[0] {
                                            if x >= x_range[0] && x <= x_range[1] {
                                                // This point intersects with the first wire
                                                intersections.push([x, y]);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    current_coordinates[0] -= length;
                }
                _ => {}
            }
        }
    }

    intersections
}

// Serializer function
pub fn input_generator(wires: String) -> Vec<String> {
    wires.lines().map(|wire| wire.to_string()).collect()
}

// Solver function for part 1
pub fn distance_from_closest_intersection(wires: Vec<String>) -> usize {
    let mut least_distance = 0;
    for intersection in find_intersections(&wires) {
        let distance = manhattan_distance([0, 0], intersection);
        if least_distance == 0 || distance < least_distance {
            least_distance = distance;
        }
    }
    least_distance
}

// Solver function for part 2
pub fn best_steps(wires: Vec<String>) -> usize {
    let mut least_steps = 0;
    for intersection in find_intersections(&wires) {
        let mut wire_1_steps = 0;
        let mut wire_2_steps = 0;
        let mut wire_1_reached = false;
        let mut wire_2_reached = false;
        'wires: for (i, wire) in wires.iter().enumerate() {
            let mut current_coordinates = [0, 0];
            for instruction in wire.split(',').map(|x| x.to_string()) {
                let length: isize = instruction[1..].parse().unwrap();
                match instruction.as_bytes()[0] as char {
                    'U' => {
                        if intersection[0] == current_coordinates[0] && intersection[1] > current_coordinates[1] && intersection[1] <= current_coordinates[1] + length {
                            // We're going to reach the intersection now
                            if i == 0 {
                                wire_1_reached = true;
                                wire_1_steps += intersection[1] - current_coordinates[1];
                            } else {
                                wire_2_reached = true;
                                wire_2_steps += intersection[1] - current_coordinates[1];
                            }
                        } else {
                            if i == 0 {
                                wire_1_steps += length;
                            } else {
                                wire_2_steps += length;
                            }
                        }
                        current_coordinates[1] += length;
                    }
                    'R' => {
                        if intersection[0] > current_coordinates[0] && intersection[0] <= current_coordinates[0] + length && intersection[1] == current_coordinates[1] {
                            // We're going to reach the intersection now
                            if i == 0 {
                                wire_1_reached = true;
                                wire_1_steps += intersection[0] - current_coordinates[0];
                            } else {
                                wire_2_reached = true;
                                wire_2_steps += intersection[0] - current_coordinates[0];
                            }
                        } else {
                            if i == 0 {
                                wire_1_steps += length;
                            } else {
                                wire_2_steps += length;
                            }
                        }
                        current_coordinates[0] += length;
                    }
                    'D' => {
                        if intersection[0] == current_coordinates[0] && intersection[1] >= current_coordinates[1] - length && intersection[1] < current_coordinates[1] {
                            // We're going to reach the intersection now
                            if i == 0 {
                                wire_1_reached = true;
                                wire_1_steps += current_coordinates[1] - intersection[1];
                            } else {
                                wire_2_reached = true;
                                wire_2_steps += current_coordinates[1] - intersection[1];
                            }
                        } else {
                            if i == 0 {
                                wire_1_steps += length;
                            } else {
                                wire_2_steps += length;
                            }
                        }
                        current_coordinates[1] -= length;
                    }
                    'L' => {
                        if intersection[0] >= current_coordinates[0] - length && intersection[0] < current_coordinates[0] && intersection[1] == current_coordinates[1] {
                            // We're going to reach the intersection now
                            if i == 0 {
                                wire_1_reached = true;
                                wire_1_steps += current_coordinates[0] - intersection[0];
                            } else {
                                wire_2_reached = true;
                                wire_2_steps += current_coordinates[0] - intersection[0];
                            }
                        } else {
                            if i == 0 {
                                wire_1_steps += length;
                            } else {
                                wire_2_steps += length;
                            }
                        }
                        current_coordinates[0] -= length;
                    }
                    _ => {}
                }
                if i == 0 && wire_1_reached || i == 1 && wire_2_reached {
                    continue 'wires;
                }
            }
        }
        if least_steps == 0 || wire_1_steps + wire_2_steps < least_steps {
            least_steps = wire_1_steps + wire_2_steps;
        }
    }
    least_steps as usize
}
