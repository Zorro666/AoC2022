use crate::file_to_vec;

/*

--- Day 6: Tuning Trouble ---

The preparations are finally complete; you and the Elves leave camp on foot and begin to make your way toward the star fruit grove.

As you move through the dense undergrowth, one of the Elves gives you a handheld device.
He says that it has many fancy features, but the most important one to set up right now is the communication system.

However, because he's heard you have significant experience dealing with signal-based systems, he convinced the other Elves that it would be okay to give you their one malfunctioning device - surely you'll have no problem fixing it.

As if inspired by comedic timing, the device emits a few colorful sparks.

To be able to communicate with the Elves, the device needs to lock on to their signal.
The signal is a series of seemingly-random characters that the device receives one at a time.

To fix the communication system, you need to add a subroutine to the device that detects a start-of-packet marker in the datastream.
In the protocol being used by the Elves, the start of a packet is indicated by a sequence of four characters that are all different.

The device will send your subroutine a datastream buffer (your puzzle input); your subroutine needs to identify the first position where the four most recently received characters were all different.
Specifically, it needs to report the number of characters from the beginning of the buffer to the end of the first such four-character marker.

For example, suppose you receive the following datastream buffer:

mjqjpqmgbljsphdztnvjfqwrcgsmlb

After the first three characters (mjq) have been received, there haven't been enough characters received yet to find the marker.
The first time a marker could occur is after the fourth character is received, making the most recent four characters mjqj.
Because j is repeated, this isn't a marker.

The first time a marker appears is after the seventh character arrives.
Once it does, the last four characters received are jpqm, which are all different.
In this case, your subroutine should report the value 7, because the first start-of-packet marker is complete after 7 characters have been processed.

Here are a few more examples:

bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11
How many characters need to be processed before the first start-of-packet marker is detected?

Your puzzle answer was 1093.

--- Part Two ---

Your device's communication system is correctly detecting packets, but still isn't working.
It looks like it also needs to look for messages.

A start-of-message marker is just like a start-of-packet marker, except it consists of 14 distinct characters rather than 4.

Here are the first positions of start-of-message markers for all of the above examples:

mjqjpqmgbljsphdztnvjfqwrcgsmlb: first marker after character 19
bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 23
nppdvjthqldpwncqszvftbrmjlhg: first marker after character 23
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 29
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 26

How many characters need to be processed before the first start-of-message marker is detected?
*/

static INPUT_FILE: &str = "data/day06/input.txt";

pub fn run() {
    println!("Day06: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day06: End");
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
            let result1 = self.find_different(4);
            println!("Day06: Result1 {result1}");
            let expected = 1093;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.find_different(14);
            println!("Day06: Result2 {result2}");
            let expected = 3534;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        for line in lines {
            self.strings.push(line.as_bytes().to_vec());
        }
        assert_eq!(self.strings.len(), 1);
    }

    fn find_different(&self, length: usize) -> i64 {
        let line = &self.strings[0];
        let count = line.len();
        let mut four_cc = [0; 14];
        for i in 0..length {
            four_cc[i] = line[i];
        }
        for i in length..count {
            let mut same = false;
            for j in 0..length - 1 {
                for k in j + 1..length {
                    if four_cc[j] == four_cc[k] {
                        same = true;
                        break;
                    }
                }
                if same {
                    break;
                }
            }
            if same {
                for j in 0..length - 1 {
                    four_cc[j] = four_cc[j + 1];
                }
                four_cc[length - 1] = line[i];
                continue;
            }
            return i as i64;
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::str_array_to_string_array;

    #[test]
    fn part1() {
        let test_datas: Vec<(&str, i64)> = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        for test_data in test_datas {
            let input = vec![test_data.0];
            let lines = str_array_to_string_array(input);
            let mut day = Day::instance(true);
            day.parse(&lines);
            assert_eq!(day.find_different(4), test_data.1);
        }
    }

    #[test]
    fn part2() {
        let test_datas: Vec<(&str, i64)> = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];
        for test_data in test_datas {
            let input = vec![test_data.0];
            let lines = str_array_to_string_array(input);
            let mut day = Day::instance(true);
            day.parse(&lines);
            assert_eq!(day.find_different(14), test_data.1);
        }
    }
}
