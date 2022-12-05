use crate::file_to_vec;

/*

--- Day 5: Supply Stacks ---

The expedition can depart as soon as the final supplies have been unloaded from the ships.
Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks.
To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps.
After the crates are rearranged, the desired crates will be at the top of each stack.

The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.

They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input).
For example:

    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
In this example, there are three stacks of crates.
Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top.
Stack 2 contains three crates; from bottom to top, they are crates M, C, and D.
Finally, stack 3 contains a single crate, P.

Then, the rearrangement procedure is given.
In each step of the procedure, a quantity of crates is moved from one stack to a different stack.
In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

[D]
[N] [C]
[Z] [M] [P]
 1   2   3
In the second step, three crates are moved from stack 1 to stack 3.
Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:

        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3
Then, both crates are moved from stack 2 to stack 1.
Again, because crates are moved one at a time, crate C ends up below crate M:

        [Z]
        [N]
[M]     [D]
[C]     [P]
 1   2   3
Finally, one crate is moved from stack 1 to stack 2:

        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3
The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.

After the rearrangement procedure completes, what crate ends up on top of each stack?

Your puzzle answer was ZSQVCCJLL.

--- Part Two ---

As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.

Some mud was covering the writing on the side of the crane, and you quickly wipe it away.
The crane isn't a CrateMover 9000 - it's a CrateMover 9001.

The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.

Again considering the example above, the crates begin in the same configuration:

    [D]
[N] [C]
[Z] [M] [P]
 1   2   3
Moving a single crate from stack 2 to stack 1 behaves the same as before:

[D]
[N] [C]
[Z] [M] [P]
 1   2   3
However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:

        [D]
        [N]
    [C] [Z]
    [M] [P]
 1   2   3
Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:

        [D]
        [N]
[C]     [Z]
[M]     [P]
 1   2   3
Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:

        [D]
        [N]
        [Z]
[M] [C] [P]
 1   2   3
In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.

Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies.
After the rearrangement procedure completes, what crate ends up on top of each stack?

*/

static INPUT_FILE: &str = "data/day05/input.txt";

pub fn run() {
    println!("Day05: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day05: End");
}

struct Day {
    part1: bool,
    stacks: Vec<Vec<u8>>,
    move_counts: Vec<usize>,
    move_froms: Vec<usize>,
    move_tos: Vec<usize>,
}

impl Day {
    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            stacks: Vec::new(),
            move_counts: Vec::new(),
            move_froms: Vec::new(),
            move_tos: Vec::new(),
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.rearrange_top_crates();
            println!("Day05: Result1 {result1}");
            let expected = "ZSQVCCJLL";
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.rearrange_top_crates();
            println!("Day05: Result2 {result2}");
            let expected = "QZFJRWHGS";
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        let count = lines.len();
        let mut do_stack = true;
        let mut do_moves = false;
        for i in 0..count {
            let line = &lines[i];
            if line.is_empty() {
                assert!(do_stack == false);
                assert!(do_moves == false);
                do_moves = true;
                continue;
            }
            if do_stack {
                if line.starts_with(" 1") {
                    //  1   2   3
                    do_stack = false;
                    continue;
                }
                //[Z] [M] [P]
                //01234567890
                let count_stacks = 1 + (line.len() - 2) / 4;
                if count_stacks > self.stacks.len() {
                    self.stacks.resize(count_stacks, Vec::new());
                }
                for s in 0..count_stacks {
                    let index = 1 + s * 4;
                    let value = line.as_bytes()[index];
                    if value != 32 {
                        assert!(value >= 'A' as u8);
                        assert!(value <= 'Z' as u8);
                        self.stacks[s].push(value);
                    }
                }
                continue;
            }
            if do_moves {
                // move 1 from 2 to 1
                assert!(line.starts_with("move"));
                let f = line.find("from").unwrap();
                let move_count: usize = line[4..f].trim().parse().expect("Not a number");
                let t = line.find("to").unwrap();
                let move_from: usize = line[f + 4..t].trim().parse().expect("Not a number");
                let move_to: usize = line[t + 2..].trim().parse().expect("Not a number");
                self.move_counts.push(move_count);
                self.move_froms.push(move_from - 1);
                self.move_tos.push(move_to - 1);
                continue;
            }
        }
    }

    fn rearrange_top_crates(&mut self) -> String {
        let count_stacks = self.stacks.len();
        let count_moves = self.move_counts.len();
        for m in 0..count_moves {
            let count = self.move_counts[m];
            let from = self.move_froms[m];
            let to = self.move_tos[m];
            for i in 0..count {
                let mut index = 0;
                if !self.part1 {
                    index = count - 1 - i;
                }
                let value = self.stacks[from].remove(index);
                self.stacks[to].insert(0, value);
            }
        }

        let mut top = String::new();
        for s in 0..count_stacks {
            top.push(self.stacks[s][0] as char);
        }
        return top;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::str_array_to_string_array;

    #[test]
    fn part1() {
        let input: Vec<&str> = vec![
            "    [D]",
            "[N] [C]",
            "[Z] [M] [P]",
            " 1   2   3",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];
        let lines = str_array_to_string_array(input);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.rearrange_top_crates(), "CMZ");
    }

    #[test]
    fn part2() {
        let input: Vec<&str> = vec![
            "    [D]",
            "[N] [C]",
            "[Z] [M] [P]",
            " 1   2   3",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];
        let lines = str_array_to_string_array(input);
        let mut day = Day::instance(false);
        day.parse(&lines);
        assert_eq!(day.rearrange_top_crates(), "MCD");
    }
}
