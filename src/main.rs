use std::env;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::time::Instant;

mod day01;
mod day02;
mod day03;

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

pub fn file_to_vec(filename: &str) -> io::Result<Vec<String>> { 
    let file_in = fs::File::open(filename)?; 
    let file_reader = BufReader::new(file_in); 
    Ok(file_reader.lines().filter_map(io::Result::ok).collect()) 
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
}