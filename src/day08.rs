use crate::file_to_vec;

/*

*/

static INPUT_FILE: &str = "data/day08/input.txt";

pub fn run() {
    println!("Day08: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day08: End");
}

struct Day {
    part1: bool,
    left_strings: Vec<Vec<u8>>,
    right_strings: Vec<Vec<u8>>,
    strings: Vec<Vec<u8>>,
}

impl Day {
    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            left_strings: Vec::new(),
            right_strings: Vec::new(),
            strings: Vec::new(),
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.sum_priorities();
            println!("Day08: Result1 {result1}");
            let expected = 7737;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.sum_priorities_three();
            println!("Day08: Result2 {result2}");
            let expected = 2697;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        for line in lines {
            let rucksack_size = line.len() / 2;
            let left = line[0..rucksack_size].as_bytes().to_vec();
            let right = line[rucksack_size..].as_bytes().to_vec();
            self.left_strings.push(left);
            self.right_strings.push(right);
            self.strings.push(line.as_bytes().to_vec());
        }
    }

    fn sum_priorities(&self) -> i64 {
        let count = self.left_strings.len();
        let mut total = 0_i64;
        for i in 0..count {
            let l = &self.left_strings[i];
            let r = &self.right_strings[i];
            for b in l {
                if r.contains(b) {
                    let c = *b;
                    if c >= 'a' as u8 && c <= 'z' as u8 {
                        let priority = c - 'a' as u8;
                        total += priority as i64;
                        total += 1;
                    } else {
                        let priority = c - 'A' as u8;
                        total += priority as i64;
                        total += 27;
                    }
                    break;
                }
            }
        }
        return total;
    }

    fn sum_priorities_three(&self) -> i64 {
        let count = self.left_strings.len();
        let mut total = 0_i64;
        for i in (0..count).step_by(3) {
            let base = &self.strings[i];
            for b in base {
                let mut found = 0;
                for j in 1..3 {
                    let other = &self.strings[i + j];
                    if other.contains(b) {
                        found += 1;
                    }
                }
                if found == 2 {
                    let c = *b;
                    if c >= 'a' as u8 && c <= 'z' as u8 {
                        let priority = c - 'a' as u8;
                        total += priority as i64;
                        total += 1;
                    } else {
                        let priority = c - 'A' as u8;
                        total += priority as i64;
                        total += 27;
                    }
                    break;
                }
            }
        }
        return total;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::str_array_to_string_array;

    #[test]
    fn part1() {
        let input: Vec<&str> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        let lines = str_array_to_string_array(input);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.sum_priorities(), 157);
    }

    #[test]
    fn part2() {
        let input: Vec<&str> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        let lines = str_array_to_string_array(input);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.sum_priorities_three(), 70);
    }
}
