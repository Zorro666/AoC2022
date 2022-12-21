use crate::file_to_vec;

/*

--- Day 20: Grove Positioning System ---

It's finally time to meet back up with the Elves.
When you try to contact them, however, you get no reply.
Perhaps you're out of range?

You know they're headed to the grove where the star fruit grows, so if you can figure out where that is, you should be able to meet back up with them.

Fortunately, your handheld device has a file (your puzzle input) that contains the grove's coordinates! Unfortunately, the file is encrypted - just in case the device were to fall into the wrong hands.

Maybe you can decrypt it?

When you were still back at the camp, you overheard some Elves talking about coordinate file encryption.
The main operation involved in decrypting the file is called mixing.

The encrypted file is a list of numbers.
To mix the file, move each number forward or backward in the file a number of positions equal to the value of the number being moved.
The list is circular, so moving a number off one end of the list wraps back around to the other end as if the ends were connected.

For example, to move the 1 in a sequence like 4, 5, 6, 1, 7, 8, 9, the 1 moves one position forward: 4, 5, 6, 7, 1, 8, 9.
To move the -2 in a sequence like 4, -2, 5, 6, 7, 8, 9, the -2 moves two positions backward, wrapping around: 4, 5, 6, 7, 8, -2, 9.

The numbers should be moved in the order they originally appear in the encrypted file.
Numbers moving around during the mixing process do not change the order in which the numbers are moved.

Consider this encrypted file:

1
2
-3
3
-2
0
4
Mixing this file proceeds as follows:

Initial arrangement:
1, 2, -3, 3, -2, 0, 4

1 moves between 2 and -3:
2, 1, -3, 3, -2, 0, 4

2 moves between -3 and 3:
1, -3, 2, 3, -2, 0, 4

-3 moves between -2 and 0:
1, 2, 3, -2, -3, 0, 4

3 moves between 0 and 4:
1, 2, -2, -3, 0, 3, 4

-2 moves between 4 and 1:
1, 2, -3, 0, 3, 4, -2

0 does not move:
1, 2, -3, 0, 3, 4, -2

4 moves between -3 and 0:
1, 2, -3, 4, 0, 3, -2

Then, the grove coordinates can be found by looking at the 1000th, 2000th, and 3000th numbers after the value 0, wrapping around the list as necessary.

In the above example, the 1000th number after 0 is 4, the 2000th is -3, and the 3000th is 2; adding these together produces 3.

Mix your encrypted file exactly once.

What is the sum of the three numbers that form the grove coordinates?

Your puzzle answer was 19559.

--- Part Two ---

The grove coordinate values seem nonsensical.
While you ponder the mysteries of Elf encryption, you suddenly remember the rest of the decryption routine you overheard back at camp.

First, you need to apply the decryption key, 811589153.
Multiply each number by the decryption key before you begin; this will produce the actual list of numbers to mix.

Second, you need to mix the list of numbers ten times.
The order in which the numbers are mixed does not change during mixing; the numbers are still moved in the order they appeared in the original, pre-mixed list.
(So, if -3 appears fourth in the original list of numbers to mix, -3 will be the fourth number to move during each round of mixing.)

Using the same example as above:

Initial arrangement:
811589153, 1623178306, -2434767459, 2434767459, -1623178306, 0, 3246356612

After 1 round of mixing:
0, -2434767459, 3246356612, -1623178306, 2434767459, 1623178306, 811589153

After 2 rounds of mixing:
0, 2434767459, 1623178306, 3246356612, -2434767459, -1623178306, 811589153

After 3 rounds of mixing:
0, 811589153, 2434767459, 3246356612, 1623178306, -1623178306, -2434767459

After 4 rounds of mixing:
0, 1623178306, -2434767459, 811589153, 2434767459, 3246356612, -1623178306

After 5 rounds of mixing:
0, 811589153, -1623178306, 1623178306, -2434767459, 3246356612, 2434767459

After 6 rounds of mixing:
0, 811589153, -1623178306, 3246356612, -2434767459, 1623178306, 2434767459

After 7 rounds of mixing:
0, -2434767459, 2434767459, 1623178306, -1623178306, 811589153, 3246356612

After 8 rounds of mixing:
0, 1623178306, 3246356612, 811589153, -2434767459, 2434767459, -1623178306

After 9 rounds of mixing:
0, 811589153, 1623178306, -2434767459, 3246356612, 2434767459, -1623178306

After 10 rounds of mixing:
0, -2434767459, 1623178306, 3246356612, -1623178306, 2434767459, 811589153

The grove coordinates can still be found in the same way.

Here, the 1000th number after 0 is 811589153, the 2000th is 2434767459, and the 3000th is -1623178306; adding these together produces 1623178306.
Apply the decryption key and mix your encrypted file ten times.

What is the sum of the three numbers that form the grove coordinates?

*/

static INPUT_FILE: &str = "data/day20/input.txt";

pub fn run() {
    println!("Day20: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day20: End");
}

struct Day {
    part1: bool,
    starting_values: Vec<i64>,
}

impl Day {
    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            starting_values: Vec::new(),
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.part1();
            println!("Day20: Result1 {result1}");
            let expected = 19559;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.part2();
            println!("Day20: Result2 {result2}");
            let expected = 912226207972;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        for line in lines {
            let v = line.parse().expect("Not anumber");
            self.starting_values.push(v);
        }
    }

    fn decipher(&self, key: i64, count_mixes: usize) -> i64 {
        let values: Vec<i64> = self.starting_values.iter().map(|&v| v * key).collect();
        let count = values.len();
        let mut mixed_indexes: Vec<usize> = (0..count).collect();
        for _ in 0..count_mixes {
            for index in 0..count {
                let v = values[index];
                let mixed_index = mixed_indexes.iter().position(|&x| x == index).unwrap();
                mixed_indexes.remove(mixed_index);
                let new_place = mixed_index as i64 + v;
                let wrap_len = mixed_indexes.len();
                // Want the positive remainder of new_place/wrap_len
                let new_place_wrapped = new_place.rem_euclid(wrap_len as i64);
                mixed_indexes.insert(new_place_wrapped as usize, index);
            }
        }

        // From the index of the 0 find the next 1000th value + 2000th value + 3000th value
        let zero_starting_index = values.iter().position(|&x| x == 0).unwrap();
        let zero_final_index = mixed_indexes
            .iter()
            .position(|&x| x == zero_starting_index)
            .unwrap();
        let sum = [1000, 2000, 3000]
            .iter()
            .map(|offset| {
                let mixed_index = (zero_final_index + offset) % count;
                let value_index = mixed_indexes[mixed_index];
                return values[value_index];
            })
            .sum();
        return sum;
    }

    fn part1(&self) -> i64 {
        return self.decipher(1, 1);
    }

    fn part2(&self) -> i64 {
        return self.decipher(811589153, 10);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::str_array_to_string_array;

    #[test]
    fn part1() {
        let input = (vec!["1", "2", "-3", "3", "-2", "0", "4"], 3);
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.part1(), input.1);
    }

    #[test]
    fn part2() {
        let input = (vec!["1", "2", "-3", "3", "-2", "0", "4"], 1623178306);
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(false);
        day.parse(&lines);
        assert_eq!(day.part2(), input.1);
    }
}
