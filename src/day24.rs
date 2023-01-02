use crate::file_to_vec;
use std::collections::HashMap;

/*

--- Day 24: Blizzard Basin ---

With everything replanted for next year (and with elephants and monkeys to tend the grove), you and the Elves leave for the extraction point.

Partway up the mountain that shields the grove is a flat, open area that serves as the extraction point.
It's a bit of a climb, but nothing the expedition can't handle.

At least, that would normally be true; now that the mountain is covered in snow, things have become more difficult than the Elves are used to.

As the expedition reaches a valley that must be traversed to reach the extraction site, you find that strong, turbulent winds are pushing small blizzards of snow and sharp ice around the valley.
It's a good thing everyone packed warm clothes!
To make it across safely, you'll need to find a way to avoid them.

Fortunately, it's easy to see all of this from the entrance to the valley, so you make a map of the valley and the blizzards (your puzzle input).
For example:

#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#

The walls of the valley are drawn as #; everything else is ground.
Clear ground - where there is currently no blizzard - is drawn as ..
Otherwise, blizzards are drawn with an arrow indicating their direction of motion: up (^), down (v), left (<), or right (>).

The above map includes two blizzards, one moving right (>) and one moving down (v).
In one minute, each blizzard moves one position in the direction it is pointing:

#.#####
#.....#
#.>...#
#.....#
#.....#
#...v.#
#####.#

Due to conservation of blizzard energy, as a blizzard reaches the wall of the valley, a new blizzard forms on the opposite side of the valley moving in the same direction.
After another minute, the bottom downward-moving blizzard has been replaced with a new downward-moving blizzard at the top of the valley instead:

#.#####
#...v.#
#..>..#
#.....#
#.....#
#.....#
#####.#

Because blizzards are made of tiny snowflakes, they pass right through each other.
After another minute, both blizzards temporarily occupy the same position, marked 2:

#.#####
#.....#
#...2.#
#.....#
#.....#
#.....#
#####.#

After another minute, the situation resolves itself, giving each blizzard back its personal space:

#.#####
#.....#
#....>#
#...v.#
#.....#
#.....#
#####.#

Finally, after yet another minute, the rightward-facing blizzard on the right is replaced with a new one on the left facing the same direction:

#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#

This process repeats at least as long as you are observing it, but probably forever.

Here is a more complex example:

#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#

Your expedition begins in the only non-wall position in the top row and needs to reach the only non-wall position in the bottom row.
On each minute, you can move up, down, left, or right, or you can wait in place.
You and the blizzards act simultaneously, and you cannot share a position with a blizzard.

In the above example, the fastest way to reach your goal requires 18 steps.

Drawing the position of the expedition as E, one way to achieve this is:

Initial state:
#E######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#

Minute 1, move down:
#.######
#E>3.<.#
#<..<<.#
#>2.22.#
#>v..^<#
######.#

Minute 2, move down:
#.######
#.2>2..#
#E^22^<#
#.>2.^>#
#.>..<.#
######.#

Minute 3, wait:
#.######
#<^<22.#
#E2<.2.#
#><2>..#
#..><..#
######.#

Minute 4, move up:
#.######
#E<..22#
#<<.<..#
#<2.>>.#
#.^22^.#
######.#

Minute 5, move right:
#.######
#2Ev.<>#
#<.<..<#
#.^>^22#
#.2..2.#
######.#

Minute 6, move right:
#.######
#>2E<.<#
#.2v^2<#
#>..>2>#
#<....>#
######.#

Minute 7, move down:
#.######
#.22^2.#
#<vE<2.#
#>>v<>.#
#>....<#
######.#

Minute 8, move left:
#.######
#.<>2^.#
#.E<<.<#
#.22..>#
#.2v^2.#
######.#

Minute 9, move up:
#.######
#<E2>>.#
#.<<.<.#
#>2>2^.#
#.v><^.#
######.#

Minute 10, move right:
#.######
#.2E.>2#
#<2v2^.#
#<>.>2.#
#..<>..#
######.#

Minute 11, wait:
#.######
#2^E^2>#
#<v<.^<#
#..2.>2#
#.<..>.#
######.#

Minute 12, move down:
#.######
#>>.<^<#
#.<E.<<#
#>v.><>#
#<^v^^>#
######.#

Minute 13, move down:
#.######
#.>3.<.#
#<..<<.#
#>2E22.#
#>v..^<#
######.#

Minute 14, move right:
#.######
#.2>2..#
#.^22^<#
#.>2E^>#
#.>..<.#
######.#

Minute 15, move right:
#.######
#<^<22.#
#.2<.2.#
#><2>E.#
#..><..#
######.#

Minute 16, move right:
#.######
#.<..22#
#<<.<..#
#<2.>>E#
#.^22^.#
######.#

Minute 17, move down:
#.######
#2.v.<>#
#<.<..<#
#.^>^22#
#.2..2E#
######.#

Minute 18, move down:
#.######
#>2.<.<#
#.2v^2<#
#>..>2>#
#<....>#
######E#

What is the fewest number of minutes required to avoid the blizzards and reach the goal?

Your puzzle answer was 281.

--- Part Two ---

As the expedition reaches the far side of the valley, one of the Elves looks especially dismayed:

He forgot his snacks at the entrance to the valley!

Since you're so good at dodging blizzards, the Elves humbly request that you go back for his snacks. From the same initial conditions, how quickly can you make it from the start to the goal, then back to the start, then back to the goal?

In the above example, the first trip to the goal takes 18 minutes, the trip back to the start takes 23 minutes, and the trip back to the goal again takes 13 minutes, for a total time of 54 minutes.

What is the fewest number of minutes required to reach the goal, go back to the start, then reach the goal again?

*/

static INPUT_FILE: &str = "data/day24/input.txt";

pub fn run() {
    println!("Day24: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day24: End");
}

struct Move {
    x: usize,
    y: usize,
    t: usize,
}

struct Day {
    part1: bool,
    initial_grid: Vec<u8>,
    grid_width: usize,
    grid_height: usize,
    moves: Vec<Move>,
    visited: HashMap<usize, Vec<usize>>,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    blizzard_width: usize,
    blizzard_height: usize,
}

impl Day {
    const EMPTY: u8 = '.' as u8;
    const WALL: u8 = '#' as u8;
    const START_END: u8 = '*' as u8;
    const BLIZZARD_UP: u8 = '^' as u8;
    const BLIZZARD_RIGHT: u8 = '>' as u8;
    const BLIZZARD_DOWN: u8 = 'v' as u8;
    const BLIZZARD_LEFT: u8 = '<' as u8;

    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            initial_grid: Vec::new(),
            grid_width: usize::MAX,
            grid_height: usize::MAX,
            moves: Vec::new(),
            visited: HashMap::new(),
            start_x: usize::MAX,
            start_y: usize::MAX,
            end_x: usize::MAX,
            end_y: usize::MAX,
            blizzard_width: usize::MAX,
            blizzard_height: usize::MAX,
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.part1();
            println!("Day24: Result1 {result1}");
            let expected = 281;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.part2();
            println!("Day24: Result2 {result2}");
            let expected = 807;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        self.grid_width = lines[0].len();
        self.grid_height = lines.len();
        self.blizzard_width = self.grid_width - 2;
        self.blizzard_height = self.grid_height - 2;
        self.start_x = lines[0].find(".").unwrap();
        self.start_y = 0;
        self.end_x = lines[lines.len() - 1].find(".").unwrap();
        self.end_y = self.grid_height - 1;
        self.initial_grid
            .resize(self.grid_width * self.grid_height, Day::EMPTY);
        for y in 0..lines.len() {
            let line = &lines[y];
            // #>>.<^<#
            let bytes = line.as_bytes();
            for x in 0..line.len() {
                let b = bytes[x];
                let idx = self.get_index_from_xy(x, y);
                self.initial_grid[idx] = b;
            }
        }
        let start_idx = self.get_index_from_xy(self.start_x, self.start_y);
        self.initial_grid[start_idx] = Day::START_END;
        let end_idx = self.get_index_from_xy(self.end_x, self.end_y);
        self.initial_grid[end_idx] = Day::START_END;
    }

    fn get_index_from_xy(&self, x: usize, y: usize) -> usize {
        return self.grid_width * y + x;
    }

    fn valid_move(&self, x: usize, y: usize, t: usize) -> bool {
        if x >= self.grid_width {
            return false;
        }
        if y >= self.grid_height {
            return false;
        }
        if self.initial_grid[self.get_index_from_xy(x, y)] == Day::WALL {
            return false;
        }
        if self.initial_grid[self.get_index_from_xy(x, y)] == Day::START_END {
            return true;
        }
        let width = self.blizzard_width;
        let blizzard_steps_x = t % width;
        // bx(<) = 1 + (x - 1 + width - blizzard_steps_x) % width
        let b_max_x = 1 + (x - 1 + blizzard_steps_x) % width;
        if self.initial_grid[self.get_index_from_xy(b_max_x, y)] == Day::BLIZZARD_LEFT {
            return false;
        }

        // bx(>) = 1 + (x - 1 + blizzard_steps_x) % width
        let b_min_x = 1 + (x - 1 + width - blizzard_steps_x) % width;
        if self.initial_grid[self.get_index_from_xy(b_min_x, y)] == Day::BLIZZARD_RIGHT {
            return false;
        }

        let height = self.blizzard_height;
        let blizzard_steps_y = t % height;
        // by(v) = 1 + (y - 1 + height - blizzard_steps_y) % height
        let b_min_y = 1 + (y - 1 + height - blizzard_steps_y) % height;
        if self.initial_grid[self.get_index_from_xy(x, b_min_y)] == Day::BLIZZARD_DOWN {
            return false;
        }
        // by(^) = 1 + (y - 1 + blizzard_steps_y) % height
        let b_max_y = 1 + (y - 1 + blizzard_steps_y) % height;
        if self.initial_grid[self.get_index_from_xy(x, b_max_y)] == Day::BLIZZARD_UP {
            return false;
        }

        return true;
    }

    fn add_move(
        &mut self,
        x: usize,
        y: usize,
        t: usize,
        end_x: usize,
        end_y: usize,
        is_wait: bool,
    ) -> bool {
        if x == end_x && y == end_y {
            return true;
        }
        if !self.valid_move(x, y, t) {
            return false;
        }
        if !is_wait {
            let idx = self.get_index_from_xy(x, y);
            if self.visited[&idx].contains(&t) {
                return false;
            }
            self.visited.entry(idx).and_modify(|v| v.push(t));
        }
        self.moves.push(Move { x, y, t });
        return false;
    }

    fn minimum_steps(
        &mut self,
        start_t: usize,
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize,
    ) -> usize {
        self.moves.clear();
        self.visited.clear();
        for idx in 0..self.grid_height * self.grid_width {
            self.visited.insert(idx, Vec::new());
        }
        self.moves.push(Move {
            x: start_x,
            y: start_y,
            t: start_t,
        });
        let max_t = start_t + (self.grid_width + self.grid_height) * 2;
        let mut min_steps = max_t;
        while !self.moves.is_empty() {
            let m = self.moves.pop().unwrap();
            let x = m.x;
            let y = m.y;
            let new_t = m.t + 1;
            if new_t >= min_steps {
                continue;
            }
            let mut at_end = false;

            at_end |= self.add_move(x, y, new_t, end_x, end_y, true);
            at_end |= self.add_move(x, y + 1, new_t, end_x, end_y, false);
            at_end |= self.add_move(x + 1, y, new_t, end_x, end_y, false);
            if x > 0 {
                at_end |= self.add_move(x - 1, y, new_t, end_x, end_y, false);
            }
            if y > 0 {
                at_end |= self.add_move(x, y - 1, new_t, end_x, end_y, false);
            }
            if at_end {
                min_steps = min_steps.min(new_t);
            }
        }
        assert_ne!(min_steps, max_t);
        return min_steps;
    }

    fn part1(&mut self) -> usize {
        return self.minimum_steps(0, self.start_x, self.start_y, self.end_x, self.end_y);
    }

    fn part2(&mut self) -> usize {
        let steps1 = self.minimum_steps(0, self.start_x, self.start_y, self.end_x, self.end_y);
        let steps2 = self.minimum_steps(steps1, self.end_x, self.end_y, self.start_x, self.start_y);
        let steps3 = self.minimum_steps(steps2, self.start_x, self.start_y, self.end_x, self.end_y);
        return steps3;
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
                "#.######", "#>>.<^<#", "#.<..<<#", "#>v.><>#", "#<^v^^>#", "######.#",
            ],
            18,
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
                "#.######", "#>>.<^<#", "#.<..<<#", "#>v.><>#", "#<^v^^>#", "######.#",
            ],
            54,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(false);
        day.parse(&lines);
        assert_eq!(day.part2(), input.1);
    }
}
