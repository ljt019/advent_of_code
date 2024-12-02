const PUZZLE_INPUT_LINK: &str = "https://adventofcode.com/2024/day/2/input";

#[derive(Debug, PartialEq)]
enum SafetyStatus {
    Safe,
    Unsafe,
}

type Report = Vec<u32>;

/// Determines if a report is safe based on level criteria.
///
/// A report is considered safe if:
/// - Levels are strictly increasing or strictly decreasing.
/// - Any two adjacent levels differ by at least one and at most three.
fn determine_safety_status(report: &Report) -> SafetyStatus {
    // Check to make sure all adjacent levels differ by at least one and at most 3
    for window in report.windows(2) {
        let difference = window[0].abs_diff(window[1]);

        if difference > 3 || difference < 1 {
            return SafetyStatus::Unsafe;
        }
    }

    // Check to make sure levels are strictly increasing, or strictly decreasing
    let is_increasing = report.windows(2).all(|window| window[0] < window[1]);
    let is_decreasing = report.windows(2).all(|window| window[0] > window[1]);

    if !(is_increasing || is_decreasing) {
        return SafetyStatus::Unsafe;
    }

    SafetyStatus::Safe
}

/// Determines if a report is safe based on level criteria.
///
/// A report is considered safe if:
/// - Levels are strictly increasing or strictly decreasing.
/// - Any two adjacent levels differ by at least one and at most three.
/// - Removing one level unviolates one of the first two rules.
fn determine_safety_status_with_quirk(report: &Report) -> SafetyStatus {
    if determine_safety_status(report) == SafetyStatus::Safe {
        return SafetyStatus::Safe;
    }

    for index in 0..report.len() {
        // Create a new levels vector without the i-th level.
        let mut modified_levels = report.clone();
        modified_levels.remove(index);

        if determine_safety_status(&modified_levels) == SafetyStatus::Safe {
            return SafetyStatus::Safe;
        }
    }

    return SafetyStatus::Unsafe;
}

pub fn day_2() {
    let raw_puzzle_data =
        crate::get_puzzle_input(PUZZLE_INPUT_LINK).expect("Couldn't get puzzle input data");

    let mut report_data: Vec<Report> = Vec::new();

    for line in raw_puzzle_data.lines() {
        let whitespace_iter = line.split_whitespace();

        let mut report: Report = Vec::new();

        for value in whitespace_iter {
            report.push(value.parse::<u32>().unwrap())
        }

        report_data.push(report);
    }

    let mut safe_count: u32 = 0;
    let mut safe_count_quirk: u32 = 0;

    for report in report_data.iter() {
        let safety_status = determine_safety_status(report);

        if safety_status == SafetyStatus::Safe {
            safe_count = safe_count + 1;
        }
    }

    for report in report_data.iter() {
        let safety_status = determine_safety_status_with_quirk(report);

        if safety_status == SafetyStatus::Safe {
            safe_count_quirk = safe_count_quirk + 1;
        }
    }

    println!("Part 1: {:?}", safe_count);
    println!("Part 2: {:?}", safe_count_quirk);
}
