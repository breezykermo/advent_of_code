mod today;
use std::env;
use std::fs;

fn main() {
    let year = env::var("YEAR").unwrap_or("2020".to_string());
    let day = env::var("DAY").unwrap_or("1".to_string());

    let day_f = format!("./data/{}/{}.txt", year, day);
    let contents =
        fs::read_to_string(day_f).expect(&format!("File for day {} doesn't exist- copy it.", day));
    println!("Solving for day {}", day);
    today::solve(contents);
}
