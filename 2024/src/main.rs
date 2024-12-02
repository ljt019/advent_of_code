mod day_1;
mod day_2;
mod day_3;

/// Download the Puzzle Input data from AoC
pub fn get_puzzle_input(puzzle_input_link: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    // Retrieve the SESSION_COOKIE environment variable
    let session_cookie = std::env::var("SESSION_COOKIE")
        .map_err(|_| "SESSION_COOKIE environment variable not set")?;

    let response = client
        .get(puzzle_input_link)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?;

    if response.status().is_success() {
        let text = response.text()?;
        println!("Puzzle data fetch - Ok");
        Ok(text)
    } else {
        Err(format!("Puzzle data fetch - {}", response.status()).into())
    }
}

fn main() {
    dotenv::dotenv().ok();

    println!("\nDay 1");
    day_1::day_1();

    println!("\nDay 2");
    day_2::day_2();

    // println!("\nDay 3");
    // day_3::day_3();
}
