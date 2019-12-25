// Solver function for part 1
pub fn calc_fuel_of_modules(masses: String) -> usize {
    masses.lines().map(|mass| mass.parse::<usize>().unwrap() / 3 - 2).sum()
}

// Solver function for part 2
pub fn calc_fuel_of_modules_with_fuel(masses: String) -> u32 {
    let mut fuel = 0;
    for mass in masses.lines() {
        // The resulting fuel can be negative, so use `isize`
        let mut additional_fuel = mass.parse::<isize>().unwrap() / 3 - 2;
        while additional_fuel > 0 {
            fuel += additional_fuel as usize;
            additional_fuel = additional_fuel / 3 - 2;
        }
    }
    fuel as u32
}
