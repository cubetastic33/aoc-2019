macro_rules! calc_fuel_for_module {
    ($mass:expr) => {
        $mass / 3 - 2
    }
}

// The first problem for this day
pub fn calc_fuel_of_modules(masses: Vec<usize>) -> usize {
    let mut fuel = 0;
    for mass in masses {
        fuel += calc_fuel_for_module!(mass);
    }
    fuel
}

// The second problem for this day
pub fn calc_fuel_of_modules_with_fuel(masses: Vec<usize>) -> usize {
    let mut fuel = 0;
    for mass in masses {
        // The resulting fuel can be negative, so use `isize`
        let mut additional_fuel = calc_fuel_for_module!(mass as isize);
        while additional_fuel > 0 {
            fuel += additional_fuel as usize;
            additional_fuel = calc_fuel_for_module!(additional_fuel);
        }
    }
    fuel
}
