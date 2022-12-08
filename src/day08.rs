use crate::file_to_vec;

/*

--- Day 8: Treetop Tree House ---

The expedition comes across a peculiar patch of tall trees all planted carefully in a grid.
The Elves explain that a previous expedition planted these trees as a reforestation effort.
Now, they're curious if this would be a good location for a tree house.

First, determine whether there is enough tree cover here to keep a tree house hidden.
To do this, you need to count the number of trees that are visible from outside the grid when looking directly along a row or column.

The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input).
For example:

30373
25512
65332
33549
35390

Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.

A tree is visible if all of the other trees between it and an edge of the grid are shorter than it.
Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.

All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view.
In this example, that only leaves the interior nine trees to consider:

The top-left 5 is visible from the left and top.
(It isn't visible from the right or bottom since other trees of height 5 are in the way.)
The top-middle 5 is visible from the top and right.
The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
The left-middle 5 is visible, but only from the right.
The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
The right-middle 3 is visible from the right.
In the bottom row, the middle 5 is visible, but the 3 and 4 are not.
With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.

Consider your map; how many trees are visible from outside the grid?

Your puzzle answer was 1792.

--- Part Two ---

Content with the amount of tree cover available, the Elves just need to know the best spot to build their tree house: they would like to be able to see a lot of trees.

To measure the viewing distance from a given tree, look up, down, left, and right from that tree; stop if you reach an edge or at the first tree that is the same height or taller than the tree under consideration.
(If a tree is right on the edge, at least one of its viewing distances will be zero.)

The Elves don't care about distant trees taller than those found by the rules above; the proposed tree house has large eaves to keep it dry, so they wouldn't be able to see higher than the tree house anyway.

In the example above, consider the middle 5 in the second row:

30373
25512
65332
33549
35390
Looking up, its view is not blocked; it can see 1 tree (of height 3).
Looking left, its view is blocked immediately; it can see only 1 tree (of height 5, right next to it).
Looking right, its view is not blocked; it can see 2 trees.
Looking down, its view is blocked eventually; it can see 2 trees (one of height 3, then the tree of height 5 that blocks its view).
A tree's scenic score is found by multiplying together its viewing distance in each of the four directions.
For this tree, this is 4 (found by multiplying 1 * 1 * 2 * 2).

However, you can do even better: consider the tree of height 5 in the middle of the fourth row:

30373
25512
65332
33549
35390
Looking up, its view is blocked at 2 trees (by another tree with a height of 5).
Looking left, its view is not blocked; it can see 2 trees.
Looking down, its view is also not blocked; it can see 1 tree.
Looking right, its view is blocked at 2 trees (by a massive tree of height 9).
This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the tree house.

Consider each tree on your map.

What is the highest scenic score possible for any tree?

*/

static INPUT_FILE: &str = "data/day08/input.txt";

pub fn run() {
    println!("Day08: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day08: End");
}

struct Day {
    part1: bool,
    grid: Vec<u8>,
    visibility: Vec<u8>,
    dim: usize,
}

impl Day {
    const FROM_LEFT: u8 = 1;
    const FROM_RIGHT: u8 = 2;
    const FROM_UP: u8 = 4;
    const FROM_DOWN: u8 = 8;

    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            grid: Vec::new(),
            visibility: Vec::new(),
            dim: 0,
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.count_visible();
            println!("Day08: Result1 {result1}");
            let expected = 1792;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.compute_scenic();
            println!("Day08: Result2 {result2}");
            let expected = 334880;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        let mut y = 0;
        self.dim = lines.len();
        let dim = self.dim;
        self.grid.resize(dim * dim, 0);
        self.visibility.resize(dim * dim, 0);
        for line in lines {
            let source = line.as_bytes().to_vec();
            for x in 0..dim {
                assert_eq!(source.len(), dim);
                let value = source[x] as u8 - '0' as u8;
                let index = y * dim + x;
                self.grid[index] = value + 1;
            }
            y += 1;
        }
        self.compute_visibility();
    }

    fn compute_visibility(&mut self) {
        let dim = self.dim;
        // Test if visible from left
        for y in 0..dim {
            let mut highest: u8 = 0;
            for x in 0..dim {
                let index = y * dim + x;
                let value = self.grid[index];
                if value > highest {
                    self.visibility[index] |= Day::FROM_LEFT;
                    highest = value;
                }
            }
        }
        // Test if visible from right
        for y in 0..dim {
            let mut highest: u8 = 0;
            for x in 0..dim {
                let index = y * dim + (dim - 1 - x);
                let value = self.grid[index];
                if value > highest {
                    self.visibility[index] |= Day::FROM_RIGHT;
                    highest = value;
                }
            }
            // Test if visible from top
            for x in 0..dim {
                let mut highest: u8 = 0;
                for y in 0..dim {
                    let index = y * dim + x;
                    let value = self.grid[index];
                    if value > highest {
                        self.visibility[index] |= Day::FROM_UP;
                        highest = value;
                    }
                }
            }
            // Test if visible from bottom
            for x in 0..dim {
                let mut highest: u8 = 0;
                for y in 0..dim {
                    let index = (dim - 1 - y) * dim + x;
                    let value = self.grid[index];
                    if value > highest {
                        self.visibility[index] |= Day::FROM_DOWN;
                        highest = value;
                    }
                }
            }
        }
    }

    fn count_visible(&self) -> i64 {
        let mut total = 0_i64;
        let dim = self.dim;
        for y in 0..dim {
            for x in 0..dim {
                let index = y * dim + x;
                if self.visibility[index] != 0 {
                    total += 1;
                }
            }
        }
        return total;
    }

    // stop at edge or first tree that is same height or taller than the starting tree
    fn compute_scenic_left(&self, x: usize, y: usize) -> i64 {
        let dim = self.dim;
        let mut index = y * dim + x;
        let start_height = self.grid[index];
        let mut count = 0;
        for _i in 0..x {
            count += 1;
            index -= 1;
            let height = self.grid[index];
            if height >= start_height {
                break;
            }
        }
        return count;
    }

    fn compute_scenic_right(&self, x: usize, y: usize) -> i64 {
        let dim = self.dim;
        let mut index = y * dim + x;
        let start_height = self.grid[index];
        let mut count = 0;
        for _i in x + 1..dim {
            count += 1;
            index += 1;
            let height = self.grid[index];
            if height >= start_height {
                break;
            }
        }
        return count;
    }

    fn compute_scenic_up(&self, x: usize, y: usize) -> i64 {
        let dim = self.dim;
        let mut index = y * dim + x;
        let start_height = self.grid[index];
        let mut count = 0;
        for _i in 0..y {
            count += 1;
            index -= dim;
            let height = self.grid[index];
            if height >= start_height {
                break;
            }
        }
        return count;
    }

    fn compute_scenic_down(&self, x: usize, y: usize) -> i64 {
        let dim = self.dim;
        let mut index = y * dim + x;
        let start_height = self.grid[index];
        let mut count = 0;
        for _i in y + 1..dim {
            count += 1;
            index += dim;
            let height = self.grid[index];
            if height >= start_height {
                break;
            }
        }
        return count;
    }

    fn compute_scenic(&self) -> i64 {
        let dim = self.dim;
        let mut max_scenic = 0;
        for y in 0..dim {
            for x in 0..dim {
                let l = self.compute_scenic_left(x, y);
                let r = self.compute_scenic_right(x, y);
                let u = self.compute_scenic_up(x, y);
                let d = self.compute_scenic_down(x, y);
                let scenic = l * r * u * d;
                if scenic > max_scenic {
                    max_scenic = scenic;
                }
            }
        }
        return max_scenic;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::str_array_to_string_array;

    #[test]
    fn part1() {
        let input: (Vec<&str>, i64) = (vec!["30373", "25512", "65332", "33549", "35390"], 21);
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.count_visible(), input.1);
    }

    #[test]
    fn part2() {
        let input: (Vec<&str>, i64) = (vec!["30373", "25512", "65332", "33549", "35390"], 8);
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.compute_scenic(), input.1);
    }

    #[test]
    fn compute_scene_left() {
        let input = vec!["30373", "25512", "65332", "33549", "35390"];
        let lines = str_array_to_string_array(input);
        let results = [1, 2];
        let xys: [(usize, usize); 2] = [(2, 1), (2, 3)];
        for i in 0..results.len() {
            let mut day = Day::instance(true);
            day.parse(&lines);
            assert_eq!(day.compute_scenic_left(xys[i].0, xys[i].1), results[i]);
        }
    }

    #[test]
    fn compute_scene_right() {
        let input = vec!["30373", "25512", "65332", "33549", "35390"];
        let lines = str_array_to_string_array(input);
        let results = [2, 2];
        let xys: [(usize, usize); 2] = [(2, 1), (2, 3)];
        for i in 0..results.len() {
            let mut day = Day::instance(true);
            day.parse(&lines);
            assert_eq!(day.compute_scenic_right(xys[i].0, xys[i].1), results[i]);
            if i == 0 {
                break;
            }
        }
    }

    #[test]
    fn compute_scene_up() {
        let input = vec!["30373", "25512", "65332", "33549", "35390"];
        let lines = str_array_to_string_array(input);
        let results = [1, 2];
        let xys: [(usize, usize); 2] = [(2, 1), (2, 3)];
        for i in 0..results.len() {
            let mut day = Day::instance(true);
            day.parse(&lines);
            assert_eq!(day.compute_scenic_up(xys[i].0, xys[i].1), results[i]);
        }
    }

    #[test]
    fn compute_scene_down() {
        let input = vec!["30373", "25512", "65332", "33549", "35390"];
        let lines = str_array_to_string_array(input);
        let results = [2, 1];
        let xys: [(usize, usize); 2] = [(2, 1), (2, 3)];
        for i in 0..results.len() {
            let mut day = Day::instance(true);
            day.parse(&lines);
            assert_eq!(day.compute_scenic_down(xys[i].0, xys[i].1), results[i]);
            if i == 0 {
                break;
            }
        }
    }
}
