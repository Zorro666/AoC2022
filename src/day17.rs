use crate::file_to_vec;
use std::collections::HashMap;

/*

--- Day 17: Pyroclastic Flow ---

Your handheld device has located an alternative exit from the cave for you and the elephants.
The ground is rumbling almost continuously now, but the strange valves bought you some time.
It's definitely getting warmer in here, though.

The tunnels eventually open into a very tall, narrow chamber.
Large, oddly-shaped rocks are falling into the chamber from above, presumably due to all the rumbling.
If you can't work out where the rocks will fall next, you might be crushed!

The five types of rocks have the following peculiar shapes, where # is rock and . is empty space:

####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##

The rocks fall in the order shown above: first the - shape, then the + shape, and so on.
Once the end of the list is reached, the same order repeats: the - shape falls first, sixth, 11th, 16th, etc.

The rocks don't spin, but they do get pushed around by jets of hot gas coming out of the walls themselves.
A quick scan reveals the effect the jets of hot gas will have on the rocks as they fall (your puzzle input).

For example, suppose this was the jet pattern in your cave:

>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>

In jet patterns, < means a push to the left, while > means a push to the right.
The pattern above means that the jets will push a falling rock right, then right, then right, then left, then left, then right, and so on.
If the end of the list is reached, it repeats.

The tall, vertical chamber is exactly seven units wide.
Each rock appears so that its left edge is two units away from the left wall and its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one).

After a rock appears, it alternates between being pushed by a jet of hot gas one unit (in the direction indicated by the next symbol in the jet pattern) and then falling one unit down.
If any movement would cause any part of the rock to move into the walls, floor, or a stopped rock, the movement instead does not occur.
If a downward movement would have caused a falling rock to move into the floor or an already-fallen rock, the falling rock stops where it is (having landed on something) and a new rock immediately begins falling.

Drawing falling rocks with @ and stopped rocks with #, the jet pattern in the example above manifests as follows:

The first rock begins falling:
|..@@@@.|
|.......|
|.......|
|.......|
+-------+

Jet of gas pushes rock right:
|...@@@@|
|.......|
|.......|
|.......|
+-------+

Rock falls 1 unit:
|...@@@@|
|.......|
|.......|
+-------+

Jet of gas pushes rock right, but nothing happens:
|...@@@@|
|.......|
|.......|
+-------+

Rock falls 1 unit:
|...@@@@|
|.......|
+-------+

Jet of gas pushes rock right, but nothing happens:
|...@@@@|
|.......|
+-------+

Rock falls 1 unit:
|...@@@@|
+-------+

Jet of gas pushes rock left:
|..@@@@.|
+-------+

Rock falls 1 unit, causing it to come to rest:
|..####.|
+-------+

A new rock begins falling:
|...@...|
|..@@@..|
|...@...|
|.......|
|.......|
|.......|
|..####.|
+-------+

Jet of gas pushes rock left:
|..@....|
|.@@@...|
|..@....|
|.......|
|.......|
|.......|
|..####.|
+-------+

Rock falls 1 unit:
|..@....|
|.@@@...|
|..@....|
|.......|
|.......|
|..####.|
+-------+

Jet of gas pushes rock right:
|...@...|
|..@@@..|
|...@...|
|.......|
|.......|
|..####.|
+-------+

Rock falls 1 unit:
|...@...|
|..@@@..|
|...@...|
|.......|
|..####.|
+-------+

Jet of gas pushes rock left:
|..@....|
|.@@@...|
|..@....|
|.......|
|..####.|
+-------+

Rock falls 1 unit:
|..@....|
|.@@@...|
|..@....|
|..####.|
+-------+

Jet of gas pushes rock right:
|...@...|
|..@@@..|
|...@...|
|..####.|
+-------+

Rock falls 1 unit, causing it to come to rest:
|...#...|
|..###..|
|...#...|
|..####.|
+-------+

A new rock begins falling:
|....@..|
|....@..|
|..@@@..|
|.......|
|.......|
|.......|
|...#...|
|..###..|
|...#...|
|..####.|
+-------+
The moment each of the next few rocks begins falling, you would see this:

|..@....|
|..@....|
|..@....|
|..@....|
|.......|
|.......|
|.......|
|..#....|
|..#....|
|####...|
|..###..|
|...#...|
|..####.|
+-------+

|..@@...|
|..@@...|
|.......|
|.......|
|.......|
|....#..|
|..#.#..|
|..#.#..|
|#####..|
|..###..|
|...#...|
|..####.|
+-------+

|..@@@@.|
|.......|
|.......|
|.......|
|....##.|
|....##.|
|....#..|
|..#.#..|
|..#.#..|
|#####..|
|..###..|
|...#...|
|..####.|
+-------+

|...@...|
|..@@@..|
|...@...|
|.......|
|.......|
|.......|
|.####..|
|....##.|
|....##.|
|....#..|
|..#.#..|
|..#.#..|
|#####..|
|..###..|
|...#...|
|..####.|
+-------+

|....@..|
|....@..|
|..@@@..|
|.......|
|.......|
|.......|
|..#....|
|.###...|
|..#....|
|.####..|
|....##.|
|....##.|
|....#..|
|..#.#..|
|..#.#..|
|#####..|
|..###..|
|...#...|
|..####.|
+-------+

|..@....|
|..@....|
|..@....|
|..@....|
|.......|
|.......|
|.......|
|.....#.|
|.....#.|
|..####.|
|.###...|
|..#....|
|.####..|
|....##.|
|....##.|
|....#..|
|..#.#..|
|..#.#..|
|#####..|
|..###..|
|...#...|
|..####.|
+-------+

|..@@...|
|..@@...|
|.......|
|.......|
|.......|
|....#..|
|....#..|
|....##.|
|....##.|
|..####.|
|.###...|
|..#....|
|.####..|
|....##.|
|....##.|
|....#..|
|..#.#..|
|..#.#..|
|#####..|
|..###..|
|...#...|
|..####.|
+-------+

|..@@@@.|
|.......|
|.......|
|.......|
|....#..|
|....#..|
|....##.|
|##..##.|
|######.|
|.###...|
|..#....|
|.####..|
|....##.|
|....##.|
|....#..|
|..#.#..|
|..#.#..|
|#####..|
|..###..|
|...#...|
|..####.|
+-------+

To prove to the elephants your simulation is accurate, they want to know how tall the tower will get after 2022 rocks have stopped (but before the 2023rd rock begins falling).
In this example, the tower of rocks will be 3068 units tall.

How many units tall will the tower of rocks be after 2022 rocks have stopped falling?

Your puzzle answer was 3109.

--- Part Two ---

The elephants are not impressed by your simulation.
They demand to know how tall the tower will be after 1000000000000 rocks have stopped!
Only then will they feel confident enough to proceed through the cave.

In the example above, the tower would be 1514285714288 units tall!

How tall will the tower be after 1000000000000 rocks have stopped?

*/

static INPUT_FILE: &str = "data/day17/input.txt";

pub fn run() {
    println!("Day17: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day17: End");
}

struct Day {
    part1: bool,
    jets: Vec<u8>,
    jet_index: usize,
    rock_index: usize,
    grid: Vec<u8>,
    grid_width: i32,
    grid_height: i32,
    rock_max_height: i32,
    rock_grids: Vec<Vec<u8>>,
    rock_widths: Vec<i32>,
    rock_heights: Vec<i32>,
}

impl Day {
    const EMPTY: u8 = '.' as u8;
    const ROCK: u8 = '#' as u8;
    const JET_LEFT: u8 = '<' as u8;
    const JET_RIGHT: u8 = '>' as u8;

    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            jets: Vec::new(),
            grid: Vec::new(),
            jet_index: 0,
            rock_index: 0,
            grid_width: 7,
            grid_height: 10000,
            rock_max_height: 0,
            rock_grids: Vec::new(),
            rock_widths: Vec::new(),
            rock_heights: Vec::new(),
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.part1(2022);
            println!("Day17: Result1 {result1}");
            let expected = 3109;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.part2(1000000000000);
            println!("Day17: Result2 {result2}");
            let expected = 1540462427753;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        self.jets = lines[0].as_bytes().to_vec();
        assert_eq!(lines.len(), 1);
        self.jet_index = 0;
        self.grid
            .resize((self.grid_height * self.grid_width) as usize, Day::EMPTY);

        self.rock_grids.resize(5, Vec::new());
        self.rock_widths.resize(5, 0);
        self.rock_heights.resize(5, 0);

        // Rocks are inverted to match grid
        // '####'
        self.rock_widths[0] = 4;
        self.rock_heights[0] = 1;
        self.rock_grids[0] = vec![Day::ROCK, Day::ROCK, Day::ROCK, Day::ROCK];

        // '.#.'
        // '###'
        // '.#.'
        self.rock_widths[1] = 3;
        self.rock_heights[1] = 3;
        self.rock_grids[1] = vec![
            Day::EMPTY,
            Day::ROCK,
            Day::EMPTY,
            Day::ROCK,
            Day::ROCK,
            Day::ROCK,
            Day::EMPTY,
            Day::ROCK,
            Day::EMPTY,
        ];

        // ###
        // ..#
        // ..#
        self.rock_widths[2] = 3;
        self.rock_heights[2] = 3;
        self.rock_grids[2] = vec![
            Day::ROCK,
            Day::ROCK,
            Day::ROCK,
            Day::EMPTY,
            Day::EMPTY,
            Day::ROCK,
            Day::EMPTY,
            Day::EMPTY,
            Day::ROCK,
        ];

        // #
        // #
        // #
        // #
        self.rock_widths[3] = 1;
        self.rock_heights[3] = 4;
        self.rock_grids[3] = vec![Day::ROCK, Day::ROCK, Day::ROCK, Day::ROCK];

        // ##
        // ##
        self.rock_widths[4] = 2;
        self.rock_heights[4] = 2;
        self.rock_grids[4] = vec![Day::ROCK, Day::ROCK, Day::ROCK, Day::ROCK];
        for i in 0..self.rock_grids.len() {
            assert_eq!(
                self.rock_grids[i].len(),
                (self.rock_widths[i] * self.rock_heights[i]) as usize
            );
        }
    }

    fn grid_index(&self, x: i32, y: i32) -> usize {
        return (x + y * self.grid_width) as usize;
    }

    /*
    fn print_grid(&self, y_min: i32, y_max: i32) {
        println!("");
        for i in y_min..y_max {
            print!("|");
            let y = y_max - i - 1;
            for x in 0..self.grid_width {
                let grid_index = self.grid_index(x, y);
                let grid = self.grid[grid_index];
                let c = grid as char;
                print!("{c}");
            }
            println!("|");
        }
        println!("+-------+");
    }
    */

    fn drop_rock(&mut self) {
        let jet_max = self.jets.len();

        // rock start
        // left edge is at x = 2
        // bottom edge is at rock_height + 3
        let rock_width = self.rock_widths[self.rock_index];
        let rock_height = self.rock_heights[self.rock_index];
        let mut rock_x = 2;
        let mut rock_y = self.rock_max_height + 3;
        let mut falling = true;
        while falling {
            // Jet movement left or right
            let jet_movement = self.jets[self.jet_index];
            let move_left = jet_movement == Day::JET_LEFT;
            let move_right = jet_movement == Day::JET_RIGHT;
            assert!(move_left || move_right);
            let new_rock_x;
            if move_left {
                new_rock_x = rock_x - 1;
            } else {
                new_rock_x = rock_x + 1;
            }
            let mut blocked = false;
            if new_rock_x < 0 {
                blocked = true;
            }
            if new_rock_x + rock_width > 7 {
                blocked = true;
            }
            if !blocked {
                for local_rock_y in 0..rock_height {
                    for local_rock_x in 0..rock_width {
                        let grid_x = local_rock_x + new_rock_x;
                        let grid_y = local_rock_y + rock_y;
                        let grid_index = self.grid_index(grid_x, grid_y);
                        let grid = self.grid[grid_index];
                        if grid == Day::ROCK {
                            let local_rock_index =
                                (local_rock_x + local_rock_y * rock_width) as usize;
                            let rock = self.rock_grids[self.rock_index][local_rock_index];
                            if rock == Day::ROCK {
                                blocked = true;
                                break;
                            }
                        }
                    }
                }
                if !blocked {
                    rock_x = new_rock_x;
                }
            }
            // Falling movement down
            let new_rock_y = rock_y - 1;
            falling = false;
            if new_rock_y >= 0 {
                falling = true;
                for local_rock_y in 0..rock_height {
                    for local_rock_x in 0..rock_width {
                        let grid_x = local_rock_x + rock_x;
                        let grid_y = local_rock_y + new_rock_y;
                        let grid_index = self.grid_index(grid_x, grid_y);
                        let grid = self.grid[grid_index];
                        if grid == Day::ROCK {
                            let local_rock_index =
                                (local_rock_x + local_rock_y * rock_width) as usize;
                            let rock = self.rock_grids[self.rock_index][local_rock_index];
                            if rock == Day::ROCK {
                                falling = false;
                                break;
                            }
                        }
                    }
                }
            }
            if falling {
                rock_y = new_rock_y
            }
            self.jet_index += 1;
            self.jet_index %= jet_max;
        }
        // Put the rock into the grid
        for local_rock_y in 0..rock_height {
            for local_rock_x in 0..rock_width {
                let grid_x = local_rock_x + rock_x;
                let grid_y = local_rock_y + rock_y;
                let grid_index = self.grid_index(grid_x, grid_y);
                let local_rock_index = (local_rock_x + local_rock_y * rock_width) as usize;
                let rock = self.rock_grids[self.rock_index][local_rock_index];
                if rock == Day::ROCK {
                    assert!(self.grid[grid_index] == Day::EMPTY);
                    self.grid[grid_index] = rock;
                }
            }
        }
        self.rock_max_height = self.rock_max_height.max(rock_y + rock_height);
    }

    fn part1(&mut self, max_count_rocks: usize) -> i32 {
        self.jet_index = 0;
        self.rock_index = 0;
        let rock_max = self.rock_grids.len();
        for i in 0..max_count_rocks {
            self.rock_index = i;
            self.rock_index %= rock_max;
            self.drop_rock();
        }
        return self.rock_max_height;
    }

    fn part2(&mut self, max_count_rocks: usize) -> usize {
        // Run the simulation store : jet_index, rock_index, total rock count, rock_height
        // Look for when when jet_index, rock_index : match
        self.jet_index = 0;
        self.rock_index = 0;
        let rock_max = self.rock_grids.len();
        let mut previous_cycles: Vec<(usize, usize)> = Vec::new();
        let mut previous_heights: Vec<usize> = Vec::new();
        for i in 0..5000 {
            self.rock_index = i;
            self.rock_index %= rock_max;
            self.drop_rock();
            let cycle = (self.jet_index, self.rock_index);
            previous_cycles.push(cycle);
            previous_heights.push(self.rock_max_height as usize);
        }
        let mut found_cycle_lengths: HashMap<(usize, usize), usize> = HashMap::new();
        let mut found_cycle_counts: HashMap<(usize, usize), usize> = HashMap::new();
        let mut found_cycle_heights: HashMap<(usize, usize), usize> = HashMap::new();
        // Find cycles
        for i in 0..2000 - 1 {
            let cycle = previous_cycles[i];
            let height_before = previous_heights[i];
            for j in i + 1..2000 {
                let this = previous_cycles[j];
                let height_after = previous_heights[j];
                if this.0 == cycle.0 && this.1 == cycle.1 {
                    let cycle_len = j - i;
                    // Ignore cycle lengths which are multiples of previous cycles
                    if !found_cycle_lengths.contains_key(&cycle) {
                        let cycle_height = height_after - height_before;
                        found_cycle_lengths.insert(cycle, cycle_len);
                        found_cycle_heights.insert(cycle, cycle_height);
                        let entry = found_cycle_counts.entry(cycle).or_insert(0);
                        *entry += 1;
                    }
                }
            }
        }
        let mut cycle_length = 0;
        let mut cycle_height = 0;
        // Use the cycle with the biggest count
        let mut max_count = 0;
        for (cycle, count) in found_cycle_counts {
            if count > max_count {
                max_count = count;
                cycle_length = found_cycle_lengths[&cycle];
                cycle_height = found_cycle_heights[&cycle];
            }
        }
        assert!(cycle_length > 0);
        let cycle_count = max_count_rocks / cycle_length;
        let start_loops = max_count_rocks - cycle_count * cycle_length;
        assert!(cycle_count > 0);
        assert!(cycle_height > 0);
        println!("Cycle {cycle_length} Count {cycle_count} Start Loops {start_loops} Cycle Height {cycle_height}");
        let start_height = previous_heights[start_loops - 1];
        let rock_max_height = start_height + cycle_count * cycle_height;
        return rock_max_height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::str_array_to_string_array;

    #[test]
    fn part1() {
        let input = (vec![">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"], 3068);
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.part1(2022), input.1);
    }

    #[test]
    fn part2() {
        let input = (
            vec![">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"],
            1514285714288,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(false);
        day.parse(&lines);
        assert_eq!(day.part2(1000000000000), input.1);
    }
}
