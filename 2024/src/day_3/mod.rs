use regex::Regex;

const PUZZLE_INPUT_LINK: &str = "https://adventofcode.com/2024/day/3/input";

type MulInstruction = (u32, u32);

/// Extracts valid `mul` instructions from a given line.
///
/// A valid `mul` instruction follows the format `mul(X,Y)`, where:
/// - `X` and `Y` are integers consisting of 1 to 3 digits.
///
/// Example of a valid instruction: `mul(44,46)`
///
/// Examples of invalid instructions:
/// - `mul(4*`
/// - `mul(6,9!`
/// - `?(12,34)`
/// - `mul ( 2 , 4 )`
fn scan_line_for_instructions(line: &str) -> Vec<MulInstruction> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let matches: Vec<_> = re.captures_iter(line).collect();

    let mut valid_instructions: Vec<MulInstruction> = Vec::new();

    for instruction in matches {
        // Get X
        let x_string: &str = &instruction[1];
        let x = x_string.parse::<u32>().unwrap();

        // Get Y
        let y_string: &str = &instruction[2];
        let y = y_string.parse::<u32>().unwrap();

        // Make MulInstruction
        let mul_instruction = (x, y);

        // Add it to valid instructions
        valid_instructions.push(mul_instruction)
    }

    return valid_instructions;
}

/// Extracts valid `mul` instructions from a given line, considering enabling and disabling quirks.
///
/// A valid `mul` instruction follows the format `mul(X,Y)`, where:
/// - `X` and `Y` are integers consisting of 1 to 3 digits.
///
/// Additionally, the extraction of `mul` instructions is influenced by the presence of
/// `do()` and `don't()` instructions:
/// - `do()` enables future `mul` instructions.
/// - `don't()` disables future `mul` instructions.
///
/// At the start of the program, `mul` instructions are enabled by default. Only the most recent
/// `do()` or `don't()` instruction applies.
///
/// Example of a valid instruction: `mul(44,46)`
///
/// Examples of invalid instructions:
/// - `mul(4*`
/// - `mul(6,9!`
/// - `?(12,34)`
/// - `mul ( 2 , 4 )`
fn scan_line_for_instructions_with_quirk(line: &str) -> Vec<MulInstruction> {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut valid_instructions: Vec<MulInstruction> = Vec::new();
    let mut is_enabled = true;

    // Track the position of the last enable/disable instruction
    let mut last_do_index = 0;
    let mut last_dont_index = 0;

    // Find all do() and don't() positions
    let do_positions: Vec<_> = do_re.find_iter(line).map(|m| m.start()).collect();
    let dont_positions: Vec<_> = dont_re.find_iter(line).map(|m| m.start()).collect();

    // Determine final state based on last instruction
    if !do_positions.is_empty() && !dont_positions.is_empty() {
        is_enabled = do_positions.last().unwrap() > dont_positions.last().unwrap();
    }

    // Collect mul instructions respecting the final state
    for cap in mul_re.captures_iter(line) {
        let mul_start = cap.get(0).map_or(0, |m| m.start());

        // Only add if in enabled state
        if is_enabled {
            if let (Ok(x), Ok(y)) = (cap[1].parse::<u32>(), cap[2].parse::<u32>()) {
                valid_instructions.push((x, y));
            }
        }
    }

    valid_instructions
}

pub fn day_3() {
    let raw_puzzle_data =
        crate::get_puzzle_input(PUZZLE_INPUT_LINK).expect("Couldn't get puzzle input data");

    let mut part_1_total: u32 = 0;
    let mut part_2_total: u32 = 0;

    for line in raw_puzzle_data.lines() {
        let mut line_total_part_1: u32 = 0;
        let mut line_total_part_2: u32 = 0;

        let instructions_part_1 = scan_line_for_instructions(line);
        let instructions_part_2 = scan_line_for_instructions_with_quirk(line);

        for instruction in instructions_part_1 {
            let product = instruction.0 * instruction.1;
            line_total_part_1 = line_total_part_1 + product;
        }

        for instruction in instructions_part_2 {
            let product = instruction.0 * instruction.1;
            line_total_part_2 = line_total_part_2 + product;
        }

        part_1_total = part_1_total + line_total_part_1;
        part_2_total = part_2_total + line_total_part_2;
    }

    println!("Part 1: {:?}", part_1_total);
    println!("Part 2: {:?}", part_2_total);
}
