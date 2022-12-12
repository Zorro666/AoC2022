use crate::file_to_vec;

/*

*/

static INPUT_FILE: &str = "data/day13/input.txt";

pub fn run() {
    println!("Day13: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day13: End");
}

struct Day {
    part1: bool,
    strings: Vec<Vec<u8>>,
}

impl Day {
    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            strings: Vec::new(),
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.part1();
            println!("Day13: Result1 {result1}");
            let expected = 7737;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.part2();
            println!("Day13: Result2 {result2}");
            let expected = 2697;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        for line in lines {
            self.strings.push(line.as_bytes().to_vec());
        }
    }

    fn part1(&self) -> usize {
        let count = self.strings.len();
        let mut total = 0;
        for i in 0..count {
            total += i;
        }
        return total;
    }

    fn part2(&self) -> usize {
        let count = self.strings.len();
        let mut total = 0;
        for i in 0..count {
            total += i;
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
        let input = (
            vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ],
            15,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.part1(), input.1);
    }

    #[test]
    fn part2() {
        let input = (
            vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ],
            15,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(false);
        day.parse(&lines);
        assert_eq!(day.part2(), input.1);
    }
}
