use aoc_helper::{AocDay, Puzzle};

mod intcode_computer;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;

fn main() {
    // day 1
    let mut day_1 = AocDay::new(2019, 1);
    day_1.run(&Puzzle::new(1, day_1::calc_fuel_of_modules)).unwrap();
    day_1.run(&Puzzle::new(2, day_1::calc_fuel_of_modules_with_fuel)).unwrap();
    // day 2
    let mut day_2 = AocDay::new_with_serializer(2019, 2, intcode_computer::intcode_serializer);
    day_2.run(&Puzzle::new(1, day_2::restore_program_state)).unwrap();
    day_2.run(&Puzzle::new(2, day_2::gravity_assist)).unwrap();
    // day 3
    let mut day_3 = AocDay::new_with_serializer(2019, 3, day_3::input_generator);
    day_3.run(&Puzzle::new(1, day_3::distance_from_closest_intersection)).unwrap();
    day_3.run(&Puzzle::new(2, day_3::best_steps)).unwrap();
    // day 4
    let mut day_4 = AocDay::new_with_serializer(2019, 4, day_4::input_generator);
    day_4.run(&Puzzle::new(1, day_4::number_of_passwords)).unwrap();
    day_4.run(&Puzzle::new(2, day_4::more_filtered_number_of_passwords)).unwrap();
    // day 5
    let mut day_5 = AocDay::new_with_serializer(2019, 5, intcode_computer::intcode_serializer);
    day_5.run(&Puzzle::new(1, day_5::test_air_conditioner)).unwrap();
    day_5.run(&Puzzle::new(2, day_5::test_thermal_radiator_controller)).unwrap();
    // day 6
    let mut day_6 = AocDay::new_with_serializer(2019, 6, day_6::input_generator);
    day_6.run(&Puzzle::new(1, day_6::orbit_count_checksum)).unwrap();
    day_6.run(&Puzzle::new(2, day_6::count_orbital_transfers)).unwrap();
    // day 7
    let mut day_7 = AocDay::new_with_serializer(2019, 7, intcode_computer::intcode_serializer);
    day_7.run(&Puzzle::new(1, day_7::find_highest_signal)).unwrap();
    day_7.run(&Puzzle::new(2, day_7::find_highest_signal_with_feedback_loop)).unwrap();
    // day 8
    let mut day_8 = AocDay::new_with_serializer(2019, 8, day_8::input_generator);
    day_8.run(&Puzzle::new(1, day_8::find_checksum)).unwrap();
    day_8.run(&Puzzle::new(2, day_8::decode_image)).unwrap();
    // day 9
    let mut day_9 = AocDay::new_with_serializer(2019, 9, intcode_computer::intcode_serializer);
    day_9.run(&Puzzle::new(1, day_9::get_boost_keycode)).unwrap();
    day_9.run(&Puzzle::new(2, day_9::get_coordinates_of_distress_signal)).unwrap();
    // day 10
    let mut day_10 = AocDay::new_with_serializer(2019, 10, day_10::input_generator);
    day_10.run(&Puzzle::new(1, day_10::asteroids_from_best_location)).unwrap();
    day_10.run(&Puzzle::new(2, day_10::two_hundredth_asteroid)).unwrap();
    // day 11
    let mut day_11 = AocDay::new_with_serializer(2019, 11, intcode_computer::intcode_serializer);
    day_11.run(&Puzzle::new(1, day_11::panels_painted_at_least_once)).unwrap();
    day_11.run(&Puzzle::new(2, day_11::registration_identifier)).unwrap();
    // day 12
    let mut day_12 = AocDay::new_with_serializer(2019, 12, day_12::input_generator);
    day_12.run(&Puzzle::new(1, day_12::total_energy)).unwrap();
    day_12.run(&Puzzle::new(2, day_12::steps_before_repeating)).unwrap();
    // day 13
    let mut day_13 = AocDay::new_with_serializer(2019, 13, intcode_computer::intcode_serializer);
    day_13.run(&Puzzle::new(1, day_13::number_of_block_tiles)).unwrap();
    day_13.run(&Puzzle::new(2, day_13::highest_score)).unwrap();
}
