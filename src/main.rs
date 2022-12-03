use std::env;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub fn file_to_vec(filename: &str) -> io::Result<Vec<String>> { 
    let file_in = fs::File::open(filename)?; 
    let file_reader = BufReader::new(file_in); 
    Ok(file_reader.lines().filter_map(io::Result::ok).collect()) 
} 

pub fn str_array_to_string_array(strings: Vec<&str>) -> Vec<String> {
    let mut lines = Vec::new() as Vec<String>;
    for s in strings {
        lines.push(s.to_string());
    }
    return lines;
}

fn run_day(requested_day: i32, this_day: i32, extra_days: bool, function: fn())
{
    if (requested_day == -1) || (requested_day == this_day) || (extra_days && this_day >= requested_day) {
        let now = Instant::now();
        function();
        let milli_seconds = now.elapsed().as_millis() as u64;
        let seconds = (milli_seconds as f32) / 1000.0f32;
        let minutes = seconds / 60.0f32;
        println!("Elapsed {milli_seconds}ms {seconds}s {minutes}mins");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut day: i32 = -1;
    let mut extra_days: bool = false;
    if args.len() == 2 {
        let arg1: &String = &args[1];
            extra_days = arg1.ends_with('+');
            day = arg1.trim_end_matches('+').parse().expect("Not a number");
    }
    run_day(day, 1, extra_days, day01::run);
    run_day(day, 2, extra_days, day02::run);
    run_day(day, 3, extra_days, day03::run);
    run_day(day, 4, extra_days, day04::run);
    run_day(day, 5, extra_days, day05::run);
    run_day(day, 6, extra_days, day06::run);
    run_day(day, 7, extra_days, day07::run);
    run_day(day, 8, extra_days, day08::run);
    run_day(day, 9, extra_days, day09::run);
    run_day(day, 10, extra_days, day10::run);
    run_day(day, 11, extra_days, day11::run);
    run_day(day, 12, extra_days, day12::run);
    run_day(day, 13, extra_days, day13::run);
    run_day(day, 14, extra_days, day14::run);
    run_day(day, 15, extra_days, day15::run);
    run_day(day, 16, extra_days, day16::run);
    run_day(day, 17, extra_days, day17::run);
    run_day(day, 18, extra_days, day18::run);
    run_day(day, 19, extra_days, day19::run);
    run_day(day, 20, extra_days, day20::run);
    run_day(day, 21, extra_days, day21::run);
    run_day(day, 22, extra_days, day22::run);
    run_day(day, 23, extra_days, day23::run);
    run_day(day, 24, extra_days, day24::run);
    run_day(day, 25, extra_days, day25::run);
}