const PUZZLE_INPUT_LINK: &str = "https://adventofcode.com/2024/day/3/input";

pub fn day_3() {
    let raw_puzzle_data =
        crate::get_puzzle_input(PUZZLE_INPUT_LINK).expect("Couldn't get puzzle input data");

    println!("{:?}", raw_puzzle_data);
}
