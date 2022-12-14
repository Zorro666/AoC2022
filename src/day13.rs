use crate::file_to_vec;

/*

--- Day 13: Distress Signal ---

You climb the hill and again try contacting the Elves.
However, you instead receive a signal you weren't expecting: a distress signal.

Your handheld device must still not be working properly; the packets from the distress signal got decoded out of order.
You'll need to re-order the list of received packets (your puzzle input) to decode the message.

Your list consists of pairs of packets; pairs are separated by a blank line.
You need to identify how many pairs of packets are in the right order.

For example:

[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]

Packet data consists of lists and integers.
Each list starts with [, ends with ], and contains zero or more comma-separated values (either integers or other lists).
Each packet is always a list and appears on its own line.

When comparing two values, the first value is called left and the second value is called right.
Then:

If both values are integers, the lower integer should come first.
If the left integer is lower than the right integer, the inputs are in the right order.
If the left integer is higher than the right integer, the inputs are not in the right order.

Otherwise, the inputs are the same integer; continue checking the next part of the input.
If both values are lists, compare the first value of each list, then the second value, and so on.
If the left list runs out of items first, the inputs are in the right order.
If the right list runs out of items first, the inputs are not in the right order.

If the lists are the same length and no comparison makes a decision about the order, continue checking the next part of the input.
If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison.
For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2); the result is then found by instead comparing [0,0,0] and [2].
Using these rules, you can determine which of the pairs in the example are in the right order:

== Pair 1 ==
- Compare [1,1,3,1,1] vs [1,1,5,1,1]
  - Compare 1 vs 1
  - Compare 1 vs 1
  - Compare 3 vs 5
    - Left side is smaller, so inputs are in the right order

== Pair 2 ==
- Compare [[1],[2,3,4]] vs [[1],4]
  - Compare [1] vs [1]
    - Compare 1 vs 1
  - Compare [2,3,4] vs 4
    - Mixed types; convert right to [4] and retry comparison
    - Compare [2,3,4] vs [4]
      - Compare 2 vs 4
        - Left side is smaller, so inputs are in the right order

== Pair 3 ==
- Compare [9] vs [[8,7,6]]
  - Compare 9 vs [8,7,6]
    - Mixed types; convert left to [9] and retry comparison
    - Compare [9] vs [8,7,6]
      - Compare 9 vs 8
        - Right side is smaller, so inputs are not in the right order

== Pair 4 ==
- Compare [[4,4],4,4] vs [[4,4],4,4,4]
  - Compare [4,4] vs [4,4]
    - Compare 4 vs 4
    - Compare 4 vs 4
  - Compare 4 vs 4
  - Compare 4 vs 4
  - Left side ran out of items, so inputs are in the right order

== Pair 5 ==
- Compare [7,7,7,7] vs [7,7,7]
  - Compare 7 vs 7
  - Compare 7 vs 7
  - Compare 7 vs 7
  - Right side ran out of items, so inputs are not in the right order

== Pair 6 ==
- Compare [] vs [3]
  - Left side ran out of items, so inputs are in the right order

== Pair 7 ==
- Compare [[[]]] vs [[]]
  - Compare [[]] vs []
    - Right side ran out of items, so inputs are not in the right order

== Pair 8 ==
- Compare [1,[2,[3,[4,[5,6,7]]]],8,9] vs [1,[2,[3,[4,[5,6,0]]]],8,9]
  - Compare 1 vs 1
  - Compare [2,[3,[4,[5,6,7]]]] vs [2,[3,[4,[5,6,0]]]]
    - Compare 2 vs 2
    - Compare [3,[4,[5,6,7]]] vs [3,[4,[5,6,0]]]
      - Compare 3 vs 3
      - Compare [4,[5,6,7]] vs [4,[5,6,0]]
        - Compare 4 vs 4
        - Compare [5,6,7] vs [5,6,0]
          - Compare 5 vs 5
          - Compare 6 vs 6
          - Compare 7 vs 0
            - Right side is smaller, so inputs are not in the right order

What are the indices of the pairs that are already in the right order?
(The first pair has index 1, the second pair has index 2, and so on.)
In the above example, the pairs in the right order are 1, 2, 4, and 6; the sum of these indices is 13.

Determine which pairs of packets are already in the right order.

What is the sum of the indices of those pairs?

Your puzzle answer was 5393.

--- Part Two ---

Now, you just need to put all of the packets in the right order.
Disregard the blank lines in your list of received packets.

The distress signal protocol also requires that you include two additional divider packets:

[[2]]
[[6]]
Using the same rules as before, organize all packets - the ones in your list of received packets as well as the two divider packets - into the correct order.

For the example above, the result of putting the packets in the correct order is:

[]
[[]]
[[[]]]
[1,1,3,1,1]
[1,1,5,1,1]
[[1],[2,3,4]]
[1,[2,[3,[4,[5,6,0]]]],8,9]
[1,[2,[3,[4,[5,6,7]]]],8,9]
[[1],4]
[[2]]
[3]
[[4,4],4,4]
[[4,4],4,4,4]
[[6]]
[7,7,7]
[7,7,7,7]
[[8,7,6]]
[9]

Afterward, locate the divider packets.

To find the decoder key for this distress signal, you need to determine the indices of the two divider packets and multiply them together. (The first packet is at index 1, the second packet is at index 2, and so on.) In this example, the divider packets are 10th and 14th, and so the decoder key is 140.

Organize all of the packets into the correct order. What is the decoder key for the distress signal?

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
    packets: Vec<Vec<u8>>,
}

impl Day {
    const LIST_START: u8 = '[' as u8;
    const LIST_END: u8 = ']' as u8;
    const SEPARATOR: u8 = ',' as u8;
    const ZERO: u8 = '0' as u8;
    const NINE: u8 = '9' as u8;

    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            packets: Vec::new(),
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.part1();
            println!("Day13: Result1 {result1}");
            let expected = 5393;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.part2();
            println!("Day13: Result2 {result2}");
            let expected = 26712;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        let mut left = true;
        let mut right = true;
        for line in lines {
            // [1,1,3,1,1]
            // [1,1,5,1,1]

            let data = line.as_bytes().to_vec();
            if left {
                self.packets.push(data);
                left = false;
                right = true;
                continue;
            } else if right {
                self.packets.push(data);
                left = false;
                right = false;
                continue;
            } else if data.is_empty() {
                left = true;
                right = false;
                continue;
            }
            panic!("Unknown line {line}");
        }
    }

    fn is_list_start(v: u8) -> bool {
        return v == Day::LIST_START;
    }

    fn is_list_end(v: u8) -> bool {
        return v == Day::LIST_END;
    }

    fn is_separator(v: u8) -> bool {
        return v == Day::SEPARATOR;
    }

    fn is_number(v: u8) -> bool {
        return Day::ZERO <= v && v <= Day::NINE;
    }

    // Recursive : return true/false
    fn compare_packets(
        &mut self,
        left_packet: usize,
        left_index: usize,
        right_packet: usize,
        right_index: usize,
    ) -> bool {
        // If the left list runs out of items first, the inputs are in the right order.
        let mut l_index = left_index;
        if l_index >= self.packets[left_packet].len() {
            return true;
        }

        // If the right list runs out of items first, the inputs are not in the right order.
        let mut r_index = right_index;
        if r_index >= self.packets[right_packet].len() {
            return false;
        }

        let l = self.packets[left_packet][l_index];
        let mut l_is_list_start = Day::is_list_start(l);
        let l_is_list_end = Day::is_list_end(l);
        let l_is_separator = Day::is_separator(l);
        let mut l_is_value = true;
        let mut l_value = l;
        if l_is_list_start {
            l_index += 1;
            l_is_value = false;
        } else if l_is_list_end {
            l_index += 1;
            l_is_value = false;
        } else if l_is_separator {
            l_index += 1;
            l_is_value = false;
        }
        let r = self.packets[right_packet][r_index];
        let r_is_list_start = Day::is_list_start(r);
        let r_is_list_end = Day::is_list_end(r);
        let r_is_separator = Day::is_separator(r);
        let mut r_is_value = true;
        let mut r_value = r;
        if r_is_list_start {
            r_index += 1;
            r_is_value = false;
        } else if r_is_list_end {
            r_index += 1;
            r_is_value = false;
        } else if r_is_separator {
            r_index += 1;
            r_is_value = false;
        }

        // If the left list runs out of items first, the inputs are in the right order.
        if l_is_list_end && !r_is_list_end {
            return true;
        }

        // If the right list runs out of items first, the inputs are not in the right order.
        if r_is_list_end && !l_is_list_end {
            return false;
        }

        if l_is_value && !r_is_value {
            assert!(r_is_list_start);
            assert!(!r_is_list_end);
            assert!(!r_is_separator);
            l_is_list_start = true;
            l_is_value = false;
            l_value = r_value;
            // Insert ] after l's value
            let mut index = l_index;
            let mut c = l;
            while Day::is_number(c) {
                index += 1;
                c = self.packets[left_packet][index];
            }
            assert!(c == Day::SEPARATOR || c == Day::LIST_END);
            self.packets[left_packet].insert(index, Day::LIST_END);
        }
        if !l_is_value && r_is_value {
            assert!(l_is_list_start);
            assert!(!l_is_list_end);
            assert!(!l_is_separator);
            r_is_value = false;
            r_value = l_value;
            // Insert ] after r's value
            let mut index = r_index;
            let mut c = r;
            while Day::is_number(c) {
                index += 1;
                c = self.packets[right_packet][index];
            }
            assert!(c == Day::SEPARATOR || c == Day::LIST_END);
            self.packets[right_packet].insert(index, Day::LIST_END);
        }
        if l_is_value {
            // Parse to find left value
            let mut v = 0;
            let mut index = l_index;
            let mut c = l;
            while Day::is_number(c) {
                v *= 10;
                v += c - Day::ZERO;
                index += 1;
                c = self.packets[left_packet][index];
            }
            l_value = v;
            l_index = index;
        }

        if r_is_value {
            // Parse to find right value
            let mut v = 0;
            let mut index = r_index;
            let mut c = r;
            while Day::is_number(c) {
                v *= 10;
                v += c - Day::ZERO;
                index += 1;
                c = self.packets[right_packet][index];
            }
            r_value = v;
            r_index = index;
        }

        if l_value == r_value {
            return self.compare_packets(left_packet, l_index, right_packet, r_index);
        }
        if l_value < r_value {
            return true;
        }
        assert!(l_value > r_value);
        return false;
    }

    fn part1(&mut self) -> usize {
        let count = self.packets.len() / 2;
        let mut total = 0;
        for i in 0..count {
            let left_packet = i * 2;
            if self.compare_packets(left_packet, 0, left_packet + 1, 0) {
                total += i + 1;
            }
        }
        return total;
    }

    fn part2(&mut self) -> usize {
        // Count how many less than "[[2]]"
        // Count how many less than "[[6]]"
        // Start from 1
        let mut two_index = 1;
        // Start from 1 and [[2]] is less than [[6]]
        let mut six_index = 2;
        let count = self.packets.len();
        self.packets.push(Vec::new());
        for i in 0..count {
            self.packets[count] = "[[2]".as_bytes().to_vec();
            if self.compare_packets(i, 0, count, 0) {
                two_index += 1;
            }
            self.packets[count] = "[[6]".as_bytes().to_vec();
            if self.compare_packets(i, 0, count, 0) {
                six_index += 1;
            }
        }
        return two_index * six_index;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::str_array_to_string_array;

    #[test]
    fn part1() {
        let tests = [
            (vec!["[1,1,3,1,1]", "[1,1,5,1,1]", ""], 1),
            (vec!["[[1],[2,3,4]]", "[[1],4]", ""], 1),
            (vec!["[9]", "[[8,7,6]]", ""], 0),
            (vec!["[[4,4],4,4]", "[[4,4],4,4,4]", ""], 1),
            (vec!["[7,7,7,7]", "[7,7,7]", ""], 0),
            (vec!["[]", "[3]", ""], 1),
            (vec!["[[[]]]", "[[]]", ""], 0),
            (
                vec!["[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"],
                0,
            ),
            (
                vec![
                    "[1,1,3,1,1]",
                    "[1,1,5,1,1]",
                    "",
                    "[[1],[2,3,4]]",
                    "[[1],4]",
                    "",
                    "[9]",
                    "[[8,7,6]]",
                    "",
                    "[[4,4],4,4]",
                    "[[4,4],4,4,4]",
                    "",
                    "[7,7,7,7]",
                    "[7,7,7]",
                    "",
                    "[]",
                    "[3]",
                    "",
                    "[[[]]]",
                    "[[]]",
                    "",
                    "[1,[2,[3,[4,[5,6,7]]]],8,9]",
                    "[1,[2,[3,[4,[5,6,0]]]],8,9]",
                ],
                13,
            ),
        ];
        for test in tests {
            let lines = str_array_to_string_array(test.0);
            let mut day = Day::instance(true);
            day.parse(&lines);
            assert_eq!(day.part1(), test.1);
        }
    }

    #[test]
    fn part2() {
        let input = (
            vec![
                "[1,1,3,1,1]",
                "[1,1,5,1,1]",
                "",
                "[[1],[2,3,4]]",
                "[[1],4]",
                "",
                "[9]",
                "[[8,7,6]]",
                "",
                "[[4,4],4,4]",
                "[[4,4],4,4,4]",
                "",
                "[7,7,7,7]",
                "[7,7,7]",
                "",
                "[]",
                "[3]",
                "",
                "[[[]]]",
                "[[]]",
                "",
                "[1,[2,[3,[4,[5,6,7]]]],8,9]",
                "[1,[2,[3,[4,[5,6,0]]]],8,9]",
            ],
            140,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(false);
        day.parse(&lines);
        assert_eq!(day.part2(), input.1);
    }
}
