use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn input_generator(orbits: &str) -> HashMap<String, Vec<String>> {
    let mut direct_orbits = HashMap::new();
    let orbits: Vec<Vec<_>> = orbits.lines().map(|orbit| orbit.split(')').map(str::to_string).collect()).collect();
    for orbit in orbits {
        direct_orbits.entry(orbit[0].clone()).or_insert(Vec::new()).push(orbit[1].clone());
    }
    direct_orbits
}

// Function that returns the entities orbiting around an entity
fn get_orbiting_entities(orbits: &HashMap<String, Vec<String>>, entity: String) -> Vec<String> {
    let mut entities = Vec::new();
    for orbiting_entity in orbits.get(&entity).unwrap() {
        entities.push(orbiting_entity.clone());
        if orbits.contains_key(orbiting_entity.as_str()) {
            entities.append(&mut get_orbiting_entities(orbits, orbiting_entity.clone()));
        }
    }
    entities
}

fn get_key_for_value(map: &HashMap<String, Vec<String>>, value: String) -> String {
    for (key, val) in map {
        if val.contains(&value) {
            return key.clone();
        }
    }
    unreachable!()
}

fn build_path(orbit_map: &HashMap<String, Vec<String>>, from: &str) -> Vec<String> {
    let mut path = Vec::new();
    if from != "COM" {
        let next = get_key_for_value(orbit_map, from.to_string());
        path.push(next.clone());
        path.append(&mut build_path(orbit_map, &next));
    }
    path
}

#[aoc(day6, part1)]
pub fn orbit_count_checksum(orbits: &HashMap<String, Vec<String>>) -> usize {
    let mut checksum = 0;
    for entity in orbits.keys() {
        checksum += get_orbiting_entities(&orbits, entity.clone()).len();
    }
    checksum
}

#[aoc(day6, part2)]
pub fn count_orbital_transfers(orbits: &HashMap<String, Vec<String>>) -> usize {
    let my_path = build_path(orbits, "YOU");
    let santas_path = build_path(orbits, "SAN");
    for (i, entity) in my_path.iter().enumerate() {
        if santas_path.contains(entity) {
            return i + santas_path.iter().position(|x| x == entity).unwrap();
        }
    }
    unreachable!()
}
