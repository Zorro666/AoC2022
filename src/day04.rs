use crate::file_to_vec;

/*

--- Day 4: Camp Cleanup ---

Space needs to be cleared before the last supplies can be unloaded from the ships, and so several Elves have been assigned the job of cleaning up sections of the camp.
Every section has a unique ID number, and each Elf is assigned a range of section IDs.

However, as some of the Elves compare their section assignments with each other, they've noticed that many of the assignments overlap.
To try to quickly find overlaps and reduce duplicated effort, the Elves pair up and make a big list of the section assignments for each pair (your puzzle input).

For example, consider the following list of section assignment pairs:

2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
For the first few pairs, this list means:

Within the first pair of Elves, the first Elf was assigned sections 2-4 (sections 2, 3, and 4), while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
The Elves in the second pair were each assigned two sections.
The Elves in the third pair were each assigned three sections: one got sections 5, 6, and 7, while the other also got 7, plus 8 and 9.
This example list uses single-digit section IDs to make it easier to draw; your actual list might contain larger numbers.
Visually, these pairs of section assignments look like this:

.234.....  2-4
.....678.  6-8

.23......  2-3
...45....  4-5

....567..  5-7
......789  7-9

.2345678.  2-8
..34567..  3-7

.....6...  6-6
...456...  4-6

.23456...  2-6
...45678.  4-8

Some of the pairs have noticed that one of their assignments fully contains the other.
For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6.
In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration.
In this example, there are 2 such pairs.

In how many assignment pairs does one range fully contain the other?

Your puzzle answer was 602.

--- Part Two ---

It seems like there is still quite a bit of duplicate work planned.
Instead, the Elves would like to know the number of pairs that overlap at all.

In the above example, the first two pairs (2-4,6-8 and 2-3,4-5) don't overlap, while the remaining four pairs (5-7,7-9, 2-8,3-7, 6-6,4-6, and 2-6,4-8) do overlap:

5-7,7-9 overlaps in a single section, 7.
2-8,3-7 overlaps all of the sections 3 through 7.
6-6,4-6 overlaps in a single section, 6.
2-6,4-8 overlaps in sections 4, 5, and 6.
So, in this example, the number of overlapping assignment pairs is 4.

In how many assignment pairs do the ranges overlap?

*/

static INPUT_FILE: &str = "data/day04/input.txt";

pub fn run() {
    println!("Day04: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day04: End");
}

struct Day {
    part1: bool,
    first_mins: Vec<u32>,
    first_maxs: Vec<u32>,
    second_mins: Vec<u32>,
    second_maxs: Vec<u32>,
}

impl Day {
    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            first_mins: Vec::new(),
            first_maxs: Vec::new(),
            second_mins: Vec::new(),
            second_maxs: Vec::new(),
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.count_contains();
            println!("Day04: Result1 {result1}");
            let expected = 602;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.count_overlaps();
            println!("Day04: Result2 {result2}");
            let expected = 891;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        for line in lines {
            // 2-6,4-8
            let comma_toks: Vec<&str> = line.split(',').collect();
            let toks: Vec<&str> = comma_toks[0].split('-').collect();
            let min = toks[0].parse().expect("Not a number");
            let max = toks[1].parse().expect("Not a number");
            self.first_mins.push(min);
            self.first_maxs.push(max);
            assert!(max >= min);
            let toks: Vec<&str> = comma_toks[1].split('-').collect();
            let min = toks[0].parse().expect("Not a number");
            let max = toks[1].parse().expect("Not a number");
            self.second_mins.push(min);
            self.second_maxs.push(max);
            assert!(max >= min);
        }
    }

    fn count_contains(&self) -> i64 {
        let count = self.first_mins.len();
        let mut total = 0_i64;
        for i in 0..count {
            let first_min = self.first_mins[i];
            let first_max = self.first_maxs[i];
            let second_min = self.second_mins[i];
            let second_max = self.second_maxs[i];
            if first_min >= second_min && first_max <= second_max {
                total += 1;
            } else if second_min >= first_min && second_max <= first_max {
                total += 1;
            }
        }
        return total;
    }

    fn count_overlaps(&self) -> i64 {
        let count = self.first_mins.len();
        let mut total = 0_i64;
        for i in 0..count {
            let first_min = self.first_mins[i];
            let first_max = self.first_maxs[i];
            let second_min = self.second_mins[i];
            let second_max = self.second_maxs[i];
            if first_max >= second_min && first_min <= second_max {
                total += 1;
            } else if second_max >= first_min && second_min <= first_max {
                total += 1;
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
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ];
        let lines = str_array_to_string_array(input);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.count_contains(), 2);
    }

    #[test]
    fn part2() {
        let input: Vec<&str> = vec![
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ];
        let lines = str_array_to_string_array(input);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.count_overlaps(), 4);
    }
}
