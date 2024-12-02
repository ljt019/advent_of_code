const PUZZLE_INPUT_LINK: &str = "https://adventofcode.com/2024/day/1/input";

#[derive(Debug)]
struct Pair {
    left: u32,
    right: u32,
}

/// Get the lowest u32 value in a Vec<u32>
fn get_lowest_number_in_vec(column_vec: &mut Vec<u32>) -> u32 {
    // Should never be empty
    assert!(!column_vec.is_empty());

    let mut lowest_number: u32 = u32::MAX;
    let mut lowest_index: usize = 0;

    for (i, &row) in column_vec.iter().enumerate() {
        if row < lowest_number {
            lowest_number = row;
            lowest_index = i;
        }
    }

    column_vec.swap_remove(lowest_index);

    lowest_number
}

/// Pass in two Vec<u32> and return a Pair of lowest values in each Vec
fn get_lowest_pair_in_vecs(column_vec_1: &mut Vec<u32>, column_vec_2: &mut Vec<u32>) -> Pair {
    let lowest_value_1 = get_lowest_number_in_vec(column_vec_1);
    let lowest_value_2 = get_lowest_number_in_vec(column_vec_2);

    let pair = Pair {
        left: lowest_value_1,
        right: lowest_value_2,
    };

    pair
}

/// Pass in a u32 and Vec<u32> and it returns how many times that u32 shows up in the vec
fn frequency_of_value_in_vec(value: u32, vec: &Vec<u32>) -> u32 {
    let mut frequency: u32 = 0;

    for vec_value in vec {
        if &value == vec_value {
            frequency = frequency + 1;
        }
    }

    frequency
}

pub fn day_1() {
    let mut part_1_answer: u32 = 0;
    let mut part_2_answer: u32 = 0;

    // Download Puzzle Input
    let raw_puzzle_data =
        crate::get_puzzle_input(PUZZLE_INPUT_LINK).expect("Couldn't get puzzle input data");

    // Create vecs to store column data
    let mut left_column_raw: Vec<u32> = Vec::new();
    let mut right_column_raw: Vec<u32> = Vec::new();

    // Create vec to store pair data
    let mut paired_data: Vec<Pair> = Vec::new();

    // Get raw data as lines
    for line in raw_puzzle_data.lines() {
        // Split line by white space
        let mut split_columns = line.split_whitespace();

        // Add left side to left column, and vice-versa
        left_column_raw.push(split_columns.next().unwrap().parse::<u32>().unwrap());
        right_column_raw.push(split_columns.next().unwrap().parse::<u32>().unwrap());
    }

    // Part 1
    {
        // Get copy of columns
        let mut left_column = left_column_raw.clone();
        let mut right_column = right_column_raw.clone();

        // Pair up the smallest number in the left list with the smallest number in the right list,
        // then the second-smallest left number with the second-smallest right number, and so on.
        while !left_column.is_empty() && !right_column.is_empty() {
            paired_data.push(get_lowest_pair_in_vecs(&mut left_column, &mut right_column));
        }

        // Calculate the distances, and sum them to get the answer
        for (left, right) in paired_data.iter().map(|pair| (pair.left, pair.right)) {
            part_1_answer += left.abs_diff(right);
        }

        // Print the answer to stdout
        println!("Part 1: {:?}", part_1_answer)
    }

    // Part 2
    {
        // Get copy of columns
        let left_column = left_column_raw.clone();
        let right_column = right_column_raw.clone();

        // Create vec to store frequency
        let mut frequency_data: Vec<(u32, u32)> = Vec::new();

        // Get frequency data
        for value in left_column {
            let frequency = frequency_of_value_in_vec(value, &right_column);

            let frequency_data_pair = (value, frequency);

            frequency_data.push(frequency_data_pair)
        }

        // get similarity score with frequency data
        for frequency_data_pair in frequency_data {
            let similarity_score = frequency_data_pair.0 * frequency_data_pair.1;

            part_2_answer = part_2_answer + similarity_score;
        }

        println!("Part 2: {:?}", part_2_answer)
    }
}
