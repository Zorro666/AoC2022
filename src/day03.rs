use crate::file_to_vec;

static INPUT_FILE: &str = "data/day03/input.txt";

pub fn run() {
    println!("Day03: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day03: End");
}

struct Day {
    part1: bool,
    values: Vec<i32>
}

impl Day {

fn instance(part1: bool) -> Day {
    Day { part1: part1, values: Vec::new() }
}

fn execute(&mut self) {
    let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
    self.parse(&lines);
    self.values.sort();

    if self.part1 {
        let result1 = 101;
        println!("Day03 : Result1 {result1}");
        let expected = 800139;
        if result1 != expected {
            panic!("Part1 is broken {result1} != {expected}");
        }
    }
    else {
        let result2 = 20202;
        println!("Day03 : Result2 {result2}");
        let expected = 59885340;
        if result2 != expected {
            panic!("Part2 is broken {result2} != {expected}");
        }
    }
}

fn parse(&mut self, lines: &Vec<String>) {
    let count = lines.len();
    self.values.resize(count, 0);
    let mut i = 0;
    for line in lines {
        self.values[i] = line.parse().expect("Not a number");
        i += 1;
    }
}

}