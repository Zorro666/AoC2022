use crate::file_to_vec;
use std::i64;

/*

--- Day 1: Calorie Counting ---

Santa's reindeer typically eat regular reindeer food, but they need a lot of magical energy to deliver presents on Christmas. 
For that, their favorite snack is a special type of star fruit that only grows deep in the jungle. 
The Elves have brought you on their annual expedition to the grove where the fruit grows.

To supply enough magical energy, the expedition needs to retrieve a minimum of fifty stars by December 25th. 
Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.

Collect stars by solving puzzles. 
Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. 
Each puzzle grants one star. Good luck!

The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot. 
As your boats approach land, the Elves begin taking inventory of their supplies. 
One important consideration is food - in particular, the number of Calories each Elf is carrying (your puzzle input).

The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. 
Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.

For example, suppose the Elves finish writing their items' Calories and end up with the following list:

1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

This list represents the Calories of the food carried by five Elves:

The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
The second Elf is carrying one food item with 4000 Calories.
The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
The fifth Elf is carrying one food item with 10000 Calories.

In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. 
In the example above, this is 24000 (carried by the fourth Elf).

Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?

Your puzzle answer was 71502.

--- Part Two ---

By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories of food might eventually run out of snacks.

To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the top three Elves carrying the most Calories. 
That way, even if one of those Elves runs out of snacks, they still have two backups.

In the example above, the top three Elves are the fourth Elf (with 24000 Calories), then the third Elf (with 11000 Calories), then the fifth Elf (with 10000 Calories). 
The sum of the Calories carried by these three elves is 45000.

Find the top three Elves carrying the most Calories. 
How many Calories are those Elves carrying in total?

*/

static INPUT_FILE: &str = "data/day01/input.txt";

pub fn run() {
    println!("Day01: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day01: End");
}

struct Day {
    part1: bool,
    totals: Vec<i64>,
}

impl Day {

fn instance(part1: bool) -> Day {
    Day { part1: part1, totals: Vec::new() }
}

fn execute(&mut self) {
    let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
    self.parse(&lines);

    if self.part1 {
        let result1 = self.most_calories();
        println!("Day01: Result1 {result1}");
        let expected = 71502;
        if result1 != expected {
            panic!("Part1 is broken {result1} != {expected}");
        }
    }
    else {
        let result2 = self.most_calories_top3();
        println!("Day01: Result2 {result2}");
        let expected = 208191;
        if result2 != expected {
            panic!("Part2 is broken {result2} != {expected}");
        }
    }
}

fn parse(&mut self, lines: &Vec<String>) {
    let mut total: i64 = 0;
    for line in lines {
        if !line.is_empty() {
            let value: i64 = line.parse().expect("Not a number");
            total += value;
        }
        else {
            self.totals.push(total);
            total = 0;
        }
    }
    if total != 0 {
        self.totals.push(total);
    }
    self.totals.sort();
    self.totals.reverse();
}

fn most_calories(&self) -> i64 {
    return self.totals[0];
}

fn most_calories_top3(&self) -> i64 {
    return self.totals[0] + self.totals[1] + self.totals[2];
}
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::str_array_to_string_array;

    #[test]
    fn most_calories() {
        let input: Vec<&str> = vec![
"1000",
"2000",
"3000",
"",
"4000",
"",
"5000",
"6000",
"",
"7000",
"8000",
"9000",
"",
"10000"];
        let lines = str_array_to_string_array(input);
        let mut day = Day::instance(false);
        day.parse(&lines);
        assert_eq!(day.most_calories(), 24000);
    }

    #[test]
    fn most_calories_top3() {
        let input: Vec<&str> = vec![
"1000",
"2000",
"3000",
"",
"4000",
"",
"5000",
"6000",
"",
"7000",
"8000",
"9000",
"",
"10000"];
        let lines = str_array_to_string_array(input);
        let mut day = Day::instance(false);
        day.parse(&lines);
        assert_eq!(day.most_calories_top3(), 45000);
    }
}
