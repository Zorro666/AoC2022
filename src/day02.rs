use crate::file_to_vec;

/*

--- Day 2: Rock Paper Scissors ---

The Elves begin to set up camp on the beach. 
To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. 
Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. 
Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. 
If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. 
"The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. 
The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. 
Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. 
Your total score is the sum of your scores for each round. 
The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z
This strategy guide predicts and recommends the following:

In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). 
This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
In the second round, your opponent will choose Paper (B), and you should choose Rock (X). 
This ends in a loss for you with a score of 1 (1 + 0).
The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?

Your puzzle answer was 15691.

--- Part Two ---

The Elf finishes helping with the tent and sneaks back over to you. 
"Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?

*/

static INPUT_FILE: &str = "data/day02/input.txt";

pub fn run() {
    println!("Day02: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day02: End");
}

struct Day {
    part1: bool,
    left_plays: Vec<usize>,
    right_plays: Vec<usize>,
    round_scores: [[i64;3];3],
    right_for_outcome: [[usize;3];3]
}

impl Day {

fn instance(part1: bool) -> Day {
    let mut round_scores: [[i64;3];3] = [[0;3];3];
    let mut right_for_outcome: [[usize;3];3] = [[0;3];3];
    for l in 0..3 {
        for r in 0..3 {
            round_scores[l][r] = Day::compute_score(l, r);
            right_for_outcome[l][r] = Day::compute_right(l, r);
        }
    }
    Day { part1: part1, left_plays: Vec::new(), right_plays: Vec::new(), round_scores: round_scores, right_for_outcome: right_for_outcome }
}

fn execute(&mut self) {
    let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
    self.parse(&lines);

    if self.part1 {
        let result1 = self.score();
        println!("Day02: Result1 {result1}");
        let expected = 15691;
        if result1 != expected {
            panic!("Part1 is broken {result1} != {expected}");
        }
    }
    else {
        let result2 = self.score();
        println!("Day02: Result2 {result2}");
        let expected = 12989;
        if result2 != expected {
            panic!("Part2 is broken {result2} != {expected}");
        }
    }
}

fn parse(&mut self, lines: &Vec<String>) {
    for line in lines {
        let bytes = line.trim().as_bytes();
        let left = bytes[0] as usize - 'A' as usize;
        let right = bytes[2] as usize - 'X' as usize;
        self.left_plays.push(left);
        self.right_plays.push(right);
    }
}

fn compute_score(left: usize, right: usize) -> i64 {
    // Rock defeats Scissors
    // Scissors defeats Paper
    // Paper defeats Rock. 

    // 0 - Rock
    // 1 - Paper
    // 2 - Scissors

    // 0 defeats 2
    // 1 defeats 0
    // 2 defeats 1
    let result: i64 = right as i64 + 1;
    if left == right {
        return result + 3;
    }
    if right == 0 && left == 2 {
        return result + 6;
    }
    if right == 1 && left == 0 {
        return result + 6;
    }
    if right == 2 && left == 1 {
        return result + 6;
    }

    return result;
}

fn compute_right(left: usize, outcome: usize) -> usize {
    // outcome for right
    // 0 = lose
    // 1 = draw
    // 2 = win
    if outcome == 1 {
        return left;
    }
    if outcome == 2 {
        if left == 0 {
            return 1;
        }
        if left == 1 {
            return 2;
        }
        if left == 2 {
            return 0;
        }
    }
    if left == 0 {
        return 2;
    }
    if left == 1 {
        return 0;
    }
    return 1;
}

fn score(&self) -> i64 {
    let count_rounds = self.left_plays.len();
    let mut total: i64 = 0;
    for i in 0..count_rounds {
        let l = self.left_plays[i];
        let r = self.right_plays[i];
        if self.part1 {
            total += self.round_scores[l][r];
        }
        else {
            let new_r = self.right_for_outcome[l][r];
            total += self.round_scores[l][new_r];
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
"A Y",
"B X",
"C Z"];
        let lines = str_array_to_string_array(input);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.score(), 15);
    }

    #[test]
    fn part2() {
        let input: Vec<&str> = vec![
"A Y",
"B X",
"C Z"];
        let lines = str_array_to_string_array(input);
        let mut day = Day::instance(false);
        day.parse(&lines);
        assert_eq!(day.score(), 12);
    }
}