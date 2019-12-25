type Position = (usize, usize);

// Serializer function
pub fn input_generator(input: String) -> Vec<Position> {
    let mut map = Vec::new();
    for (y, row) in input.lines().enumerate() {
        for (x, position) in row.chars().enumerate() {
            if position == '#' {
                map.push((x, y));
            }
        }
    }
    map
}

fn find_steps(x_offset: isize, y_offset: isize) -> (isize, isize) {
    if x_offset == 0 {
        (0, y_offset / y_offset.abs())
    } else if y_offset == 0 {
        (x_offset / x_offset.abs(), 0)
    } else if x_offset.abs() <= y_offset.abs() {
        for factor in 1..=x_offset.abs() {
            if x_offset % factor == 0 && y_offset % (x_offset / factor) == 0 {
                return (factor * x_offset / x_offset.abs(), y_offset * factor / x_offset);
            }
        }
        unreachable!()
    } else {
        for factor in 1..=y_offset.abs() {
            if y_offset % factor == 0 && x_offset % (y_offset / factor) == 0 {
                return (x_offset * factor / y_offset, factor * y_offset / y_offset.abs());
            }
        }
        unreachable!()
    }
}

fn in_line_of_sight(map: &[Position], monitoring_station: &Position, asteroid: &Position) -> bool {
    let asteroid_x_offset = monitoring_station.0 as isize - asteroid.0 as isize;
    let asteroid_y_offset = monitoring_station.1 as isize - asteroid.1 as isize;
    let asteroid_step_rate = find_steps(asteroid_x_offset, asteroid_y_offset);
    for position in map {
        if position != asteroid && position != monitoring_station {
            let x_offset = monitoring_station.0 as isize - position.0 as isize;
            let y_offset = monitoring_station.1 as isize - position.1 as isize;
            let step_rate = find_steps(x_offset, y_offset);
            if step_rate == asteroid_step_rate && (x_offset.abs() < asteroid_x_offset.abs() || y_offset.abs() < asteroid_y_offset.abs()) {
                return false;
            }
        }
    }
    true
}

fn count_detectable_asteroids(map: &[Position], monitoring_station: &Position) -> usize {
        let mut detectable_asteroids = 0;
        for asteroid in map {
            if asteroid != monitoring_station {
                if in_line_of_sight(map, monitoring_station, asteroid) {
                    detectable_asteroids += 1;
                }
            }
        }
        detectable_asteroids
}

// Solver function for part 1
pub fn asteroids_from_best_location(map: Vec<Position>) -> usize {
    let mut highest_detectable_asteroids = 0;
    for monitoring_station in &map {
        let detectable_asteroids = count_detectable_asteroids(&map, monitoring_station);
        if detectable_asteroids > highest_detectable_asteroids {
            highest_detectable_asteroids = detectable_asteroids;
        }
    }
    highest_detectable_asteroids
}

// Solver function for part 2
pub fn two_hundredth_asteroid(map: Vec<Position>) -> usize {
    let mut map_with_angles = Vec::new();
    let mut highest_detectable_asteroids = (0, (0, 0));
    for monitoring_station in &map {
        let detectable_asteroids = count_detectable_asteroids(&map, monitoring_station);
        if detectable_asteroids > highest_detectable_asteroids.0 {
            highest_detectable_asteroids = (detectable_asteroids, *monitoring_station);
        }
    }

    let monitoring_station = highest_detectable_asteroids.1;

    for asteroid in &map {
        if *asteroid != monitoring_station {
            if in_line_of_sight(&map, &monitoring_station, asteroid) {
                let mut angle = if asteroid.1 == monitoring_station.1 && asteroid.0 as isize - monitoring_station.0 as isize > 0 {
                    std::f32::consts::FRAC_PI_2
                } else if asteroid.1 == monitoring_station.1 {
                    std::f32::consts::FRAC_PI_2 * 3.
                } else if asteroid.0 == monitoring_station.0 && asteroid.1 > monitoring_station.1 {
                    std::f32::consts::PI
                } else if asteroid.0 < monitoring_station.0 && asteroid.1 > monitoring_station.1 {
                    ((asteroid.0 as f32 - monitoring_station.0 as f32) / (monitoring_station.1 as f32 - asteroid.1 as f32)).atan() + std::f32::consts::PI
                } else if asteroid.0 > monitoring_station.0 && asteroid.1 > monitoring_station.1 {
                    ((asteroid.0 as f32 - monitoring_station.0 as f32) / (asteroid.1 as f32 - monitoring_station.1 as f32)).atan() + std::f32::consts::FRAC_PI_2
                } else {
                    ((asteroid.0 as f32 - monitoring_station.0 as f32) / (monitoring_station.1 as f32 - asteroid.1 as f32)).atan()
                };
                if angle < 0. {
                    angle = angle + std::f32::consts::PI * 2.;
                }
                map_with_angles.push((angle, asteroid));
            }
        }
    }

    map_with_angles.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    (map_with_angles[199].1).0 * 100 + (map_with_angles[199].1).1
}
