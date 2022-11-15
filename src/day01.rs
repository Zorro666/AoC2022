use crate::file_to_vec;

static INPUT_FILE: &str = "data/day01/input.txt";

pub fn run() {
    println!("Day01: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day01: End");
}

struct Day {
    part1: bool,
    values: Vec<i32>
}

impl Day {

const TARGET_SUM: i32 = 2020;

fn instance(part1: bool) -> Day {
    Day { part1: part1, values: Vec::new() }
}

fn execute(&mut self) {
    let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
    self.parse(&lines);

    if self.part1 {
        let result1 = Day::sum_two(Day::TARGET_SUM, &mut self.values);
        println!("Day01 : Result1 {result1}");
        let expected = 800139;
        if result1 != expected {
            panic!("Part1 is broken {result1} != {expected}");
        }
    }
    else {
        let result2 = Day::sum_three(Day::TARGET_SUM, &mut self.values);
        println!("Day01 : Result2 {result2}");
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

fn sum_two(target_sum: i32, values: &mut Vec<i32>) -> i64 {
    values.sort();
    let count = values.len();
    for i in 0..count - 1 {
        let ivalue = values[i];
        let mut iremainder = target_sum;
        iremainder -= ivalue;
        for j in i+1..count {
            let jvalue = values[j];
            let remainder = iremainder - jvalue;
            if remainder < 0 {
                break;
            }
            if remainder == 0 {
                let sum = ivalue + jvalue;
                if sum != target_sum {
                    panic!("Expecting {target_sum} got {sum} = {ivalue} + {jvalue}");
                }
                let mult: i64 = ivalue as i64 * jvalue as i64;
                return mult;
            }
        }
    }
    panic!("Sum of {target_sum} not found");
}

fn sum_three(target_sum: i32, values: &mut Vec<i32>) -> i64 {
    values.sort();
    let count = values.len();
    for i in 0..count - 2 {
        let ivalue = values[i];
        let mut iremainder = target_sum;
        iremainder -= ivalue;
        for j in i+1..count-1 {
            let jvalue = values[j];
            let jremainder = iremainder - jvalue;
            if jremainder < 0 {
                break;
            }
            for k in j+1..count {
                let kvalue = values[k];
                let remainder = jremainder - kvalue;
                if remainder < 0 {
                    break;
                }
                if remainder == 0 {
                    let sum = ivalue + jvalue + kvalue;
                    if sum != target_sum {
                        panic!("Expecting {target_sum} got {sum} = {ivalue} + {jvalue} + {kvalue}");
                    }
                    let mult: i64 = ivalue as i64 * jvalue as i64 * kvalue as i64;
                    return mult;
                }
            }
        }
    }
    panic!("Sum of {target_sum} not found");
}

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_two() {
        let mut values = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(Day::sum_two(2020, &mut values), 514579);
    }

    #[test]
    fn sum_three() {
        let mut values = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(Day::sum_three(2020, &mut values), 241861950);
    }
}
