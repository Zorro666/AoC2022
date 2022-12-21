use crate::file_to_vec;

/*

--- Day 21: Monkey Math ---

The monkeys are back! You're worried they're going to try to steal your stuff again, but it seems like they're just holding their ground and making various monkey noises at you.

Eventually, one of the elephants realizes you don't speak monkey and comes over to interpret.
As it turns out, they overheard you talking about trying to find the grove; they can show you a shortcut if you answer their riddle.

Each monkey is given a job: either to yell a specific number or to yell the result of a math operation.
All of the number-yelling monkeys know their number from the start; however, the math operation monkeys need to wait for two other monkeys to yell a number, and those two other monkeys might also be waiting on other monkeys.

Your job is to work out the number the monkey named root will yell before the monkeys figure it out themselves.

For example:

root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32

Each line contains the name of a monkey, a colon, and then the job of that monkey:

A lone number means the monkey's job is simply to yell that number.

- A job like aaaa + bbbb means the monkey waits for monkeys aaaa and bbbb to yell each of their numbers; the monkey then yells the sum of those two numbers.
- aaaa - bbbb means the monkey yells aaaa's number minus bbbb's number.
- Job aaaa * bbbb will yell aaaa's number multiplied by bbbb's number.
- Job aaaa / bbbb will yell aaaa's number divided by bbbb's number.

So, in the above example, monkey drzm has to wait for monkeys hmdt and zczc to yell their numbers.
Fortunately, both hmdt and zczc have jobs that involve simply yelling a single number, so they do this immediately: 32 and 2.
Monkey drzm can then yell its number by finding 32 minus 2: 30.

Then, monkey sjmn has one of its numbers (30, from monkey drzm), and already has its other number, 5, from dbpl.
This allows it to yell its own number by finding 30 multiplied by 5: 150.

This process continues until root yells a number: 152.

However, your actual situation involves considerably more monkeys.

What number will the monkey named root yell?

Your puzzle answer was 145167969204648.

--- Part Two ---

Due to some kind of monkey-elephant-human mistranslation, you seem to have misunderstood a few key details about the riddle.

First, you got the wrong job for the monkey named root; specifically, you got the wrong math operation.
The correct operation for monkey root should be =, which means that it still listens for two numbers (from the same two monkeys as before), but now checks that the two numbers match.

Second, you got the wrong monkey for the job starting with humn:.
It isn't a monkey - it's you.
Actually, you got the job wrong, too: you need to figure out what number you need to yell so that root's equality check passes.
(The number that appears after humn: in your input is now irrelevant.)

In the above example, the number you need to yell to pass root's equality test is 301.
(This causes root to get the same number, 150, from both of its monkeys.)

What number do you yell to pass root's equality test?

*/

static INPUT_FILE: &str = "data/day21/input.txt";

pub fn run() {
    println!("Day21: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day21: End");
}

struct Day {
    part1: bool,
    monkey_names: Vec<String>,
    monkey_has_value: Vec<bool>,
    monkey_values: Vec<i64>,
    monkey_lhs: Vec<usize>,
    monkey_rhs: Vec<usize>,
    monkey_uses_human: Vec<bool>,
    monkey_has_human: Vec<bool>,
    monkey_op: Vec<u8>,
    monkey_root: usize,
    monkey_humn: usize,
}

impl Day {
    const VAL: u8 = 0;
    const ADD: u8 = 1;
    const SUB: u8 = 2;
    const MUL: u8 = 3;
    const DIV: u8 = 4;
    const EQU: u8 = 5;

    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            monkey_names: Vec::new(),
            monkey_has_value: Vec::new(),
            monkey_values: Vec::new(),
            monkey_lhs: Vec::new(),
            monkey_rhs: Vec::new(),
            monkey_op: Vec::new(),
            monkey_uses_human: Vec::new(),
            monkey_has_human: Vec::new(),
            monkey_root: usize::MAX,
            monkey_humn: usize::MAX,
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.part1();
            println!("Day21: Result1 {result1}");
            let expected = 145167969204648;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.part2();
            println!("Day21: Result2 {result2}");
            let expected = 3330805295850;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        let count = lines.len();
        let mut monkey_lhs_names: Vec<String> = Vec::new();
        let mut monkey_rhs_names: Vec<String> = Vec::new();
        self.monkey_root = usize::MAX;
        self.monkey_humn = usize::MAX;
        for line in lines {
            // root: pppw + sjmn
            // dbpl: 5
            let mut iter = line.split_ascii_whitespace().into_iter();
            let name = iter.next().unwrap().trim_end_matches(":");
            let value: i64;
            let lhs = usize::MAX;
            let op;
            let rhs = usize::MAX;
            let value_lhs_string = iter.next().unwrap();
            let result = value_lhs_string.parse();
            let has_value = result.is_ok();
            let lhs_name;
            let rhs_name;
            let uses_human;
            let has_human;
            if has_value {
                value = result.unwrap();
                op = Day::VAL;
                lhs_name = "";
                rhs_name = "";
            } else {
                value = i64::MAX;
                lhs_name = value_lhs_string;
                let op_name = iter.next().unwrap().trim();
                op = match op_name {
                    "+" => Day::ADD,
                    "-" => Day::SUB,
                    "*" => Day::MUL,
                    "/" => Day::DIV,
                    _ => panic!("Unknown operation {op_name}"),
                };
                rhs_name = iter.next().unwrap().trim();
            }
            if name == "root" {
                self.monkey_root = self.monkey_names.len();
            }
            if name == "humn" {
                self.monkey_humn = self.monkey_names.len();
                uses_human = true;
                has_human = true;
            } else {
                uses_human = false;
                has_human = has_value;
            }
            monkey_lhs_names.push(lhs_name.to_string());
            monkey_rhs_names.push(rhs_name.to_string());
            self.monkey_names.push(name.to_string());
            self.monkey_values.push(value);
            self.monkey_has_value.push(has_value);
            self.monkey_lhs.push(lhs);
            self.monkey_op.push(op);
            self.monkey_rhs.push(rhs);
            self.monkey_uses_human.push(uses_human);
            self.monkey_has_human.push(has_human);
        }
        assert!(self.monkey_root < self.monkey_names.len());
        assert!(self.monkey_humn < self.monkey_names.len());
        // Loop over and resolve names -> indexes
        for i in 0..count {
            let lhs_name = &monkey_lhs_names[i];
            let rhs_name = &monkey_rhs_names[i];
            if lhs_name.is_empty() {
                assert!(rhs_name.is_empty());
                continue;
            }
            if rhs_name.is_empty() {
                assert!(lhs_name.is_empty());
                continue;
            }
            let lhs = self
                .monkey_names
                .iter()
                .position(|n| n == lhs_name)
                .unwrap();
            let rhs = self
                .monkey_names
                .iter()
                .position(|n| n == rhs_name)
                .unwrap();
            self.monkey_lhs[i] = lhs;
            self.monkey_rhs[i] = rhs;
        }
    }

    fn reset_equations(&mut self) {
        for i in 0..self.monkey_names.len() {
            if self.monkey_op[i] != Day::VAL {
                self.monkey_has_value[i] = false;
            }
        }
    }

    fn find_human_value(&mut self, monkey: usize, value: i64) -> i64 {
        if monkey == self.monkey_humn {
            return value;
        }
        let result = value;
        let lhs = self.monkey_lhs[monkey];
        let rhs = self.monkey_rhs[monkey];
        assert_ne!(lhs, rhs);
        let op = self.monkey_op[monkey];
        if self.monkey_uses_human[lhs] {
            assert!(!self.monkey_uses_human[rhs]);
            assert!(self.monkey_has_value[rhs]);
            let rhs_value = self.monkey_values[rhs];
            let lhs_value = match op {
                Day::ADD => result - rhs_value, // result = Z + rhs
                Day::SUB => result + rhs_value, // result = Z - rhs
                Day::MUL => result / rhs_value, // result = Z * rhs
                Day::DIV => result * rhs_value, // result = Z / rhs
                _ => panic!("Unknown operation"),
            };
            return self.find_human_value(lhs, lhs_value);
        } else if self.monkey_uses_human[rhs] {
            assert!(!self.monkey_uses_human[lhs]);
            assert!(self.monkey_has_value[lhs]);
            let lhs_value = self.monkey_values[lhs];
            let rhs_value = match op {
                Day::ADD => result - lhs_value, // result = lhs + Z
                Day::SUB => lhs_value - result, // result = lhs - Z
                Day::MUL => result / lhs_value, // result = lhs * Z
                // Day::DIV => lhs_value / result, // result = lhs / Z
                _ => panic!("Unknown operation"),
            };
            return self.find_human_value(rhs, rhs_value);
        }
        panic!("Bad state");
    }

    fn resolve(&mut self, monkey: usize) -> i64 {
        if self.monkey_has_value[monkey] {
            return self.monkey_values[monkey];
        }
        let lhs_value = self.resolve(self.monkey_lhs[monkey]);
        let rhs_value = self.resolve(self.monkey_rhs[monkey]);
        let result = match self.monkey_op[monkey] {
            Day::ADD => lhs_value + rhs_value,
            Day::SUB => lhs_value - rhs_value,
            Day::MUL => lhs_value * rhs_value,
            Day::DIV => lhs_value / rhs_value,
            Day::EQU => {
                if lhs_value == rhs_value {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Unknown operation"),
        };
        self.monkey_has_value[monkey] = true;
        self.monkey_values[monkey] = result;
        return result;
    }

    fn track_humn_usage(&mut self, monkey: usize) -> bool {
        if self.monkey_has_human[monkey] {
            return self.monkey_uses_human[monkey];
        }
        let lhs_human = self.track_humn_usage(self.monkey_lhs[monkey]);
        let rhs_human = self.track_humn_usage(self.monkey_rhs[monkey]);
        self.monkey_has_human[monkey] = true;
        let result = lhs_human | rhs_human;
        self.monkey_uses_human[monkey] = result;
        return result;
    }

    fn part1(&mut self) -> i64 {
        return self.resolve(self.monkey_root);
    }

    fn part2(&mut self) -> i64 {
        assert!(self.monkey_has_value[self.monkey_humn]);
        // Compute initial values
        _ = self.resolve(self.monkey_root);

        // Track which equations use "humn" and propagate it
        assert!(self.track_humn_usage(self.monkey_root));

        // lhs nor rhs can use human
        let monkey_root_lhs = self.monkey_lhs[self.monkey_root];
        let monkey_root_rhs = self.monkey_rhs[self.monkey_root];
        let lhs_human = self.monkey_uses_human[monkey_root_lhs];
        let rhs_human = self.monkey_uses_human[monkey_root_rhs];

        assert!(lhs_human ^ rhs_human);

        let target_start;
        let target_value;
        if lhs_human {
            target_start = monkey_root_lhs;
            target_value = self.monkey_values[monkey_root_rhs];
        } else {
            target_start = monkey_root_rhs;
            target_value = self.monkey_values[monkey_root_lhs];
        }
        // Inverse recurse undo one eqation at a time with a known result
        let human_value = self.find_human_value(target_start, target_value);

        // Change monkey root equation to equals to check the answer
        self.monkey_values[self.monkey_humn] = human_value;
        self.reset_equations();
        self.monkey_op[self.monkey_root] = Day::EQU;
        assert_eq!(self.resolve(self.monkey_root), 1);
        return human_value;
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
                "root: pppw + sjmn",
                "dbpl: 5",
                "cczh: sllz + lgvd",
                "zczc: 2",
                "ptdq: humn - dvpt",
                "dvpt: 3",
                "lfqf: 4",
                "humn: 5",
                "ljgn: 2",
                "sjmn: drzm * dbpl",
                "sllz: 4",
                "pppw: cczh / lfqf",
                "lgvd: ljgn * ptdq",
                "drzm: hmdt - zczc",
                "hmdt: 32",
            ],
            152,
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
                "root: pppw + sjmn",
                "dbpl: 5",
                "cczh: sllz + lgvd",
                "zczc: 2",
                "ptdq: humn - dvpt",
                "dvpt: 3",
                "lfqf: 4",
                "humn: 5",
                "ljgn: 2",
                "sjmn: drzm * dbpl",
                "sllz: 4",
                "pppw: cczh / lfqf",
                "lgvd: ljgn * ptdq",
                "drzm: hmdt - zczc",
                "hmdt: 32",
            ],
            301,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(false);
        day.parse(&lines);
        assert_eq!(day.part2(), input.1);
    }
}
