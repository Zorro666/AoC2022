use crate::file_to_vec;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/*

--- Day 12: Hill Climbing Algorithm ---

You try contacting the Elves using your handheld device, but the river you're following must be too low to get a decent signal.

You ask the device for a heightmap of the surrounding area (your puzzle input).
The heightmap shows the local area from above broken into a grid; the elevation of each square of the grid is given by a single lowercase letter, where a is the lowest elevation, b is the next-lowest, and so on up to the highest elevation, z.

Also included on the heightmap are marks for your current position (S) and the location that should get the best signal (E).
Your current position (S) has elevation a, and the location that should get the best signal (E) has elevation z.

You'd like to reach E, but to save energy, you should do it in as few steps as possible.
During each step, you can move exactly one square up, down, left, or right.
To avoid needing to get out your climbing gear, the elevation of the destination square can be at most one higher than the elevation of your current square; that is, if your current elevation is m, you could step to elevation n, but not to elevation o.
(This also means that the elevation of the destination square can be much lower than the elevation of your current square.)

For example:

Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
Here, you start in the top-left corner; your goal is near the middle.
You could start by moving down or right, but eventually you'll need to head toward the e at the bottom.
From there, you can spiral around to the goal:

v..v<<<<
>v.vv<<^
.>vv>E^^
..v>>>^^
..>>>>>^
In the above diagram, the symbols indicate whether the path exits each square moving up (^), down (v), left (<), or right (>).
The location that should get the best signal is still E, and .  marks unvisited squares.

This path reaches the goal in 31 steps, the fewest possible.

What is the fewest steps required to move from your current position to the location that should get the best signal?

Your puzzle answer was 350.

--- Part Two ---

As you walk up the hill, you suspect that the Elves will want to turn this into a hiking trail. The beginning isn't very scenic, though; perhaps you can find a better starting point.

To maximize exercise while hiking, the trail should start as low as possible: elevation a. The goal is still the square marked E. However, the trail should still be direct, taking the fewest steps to reach its goal. So, you'll need to find the shortest path from any square at elevation a to the square marked E.

Again consider the example from above:

Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
Now, there are six choices for starting position (five marked a, plus the square marked S that counts as being at elevation a). If you start at the bottom-left square, you can reach the goal most quickly:

...v<<<<
...vv<<^
...v>E^^
.>v>>>^^
>^>>>>>^
This path reaches the goal in only 29 steps, the fewest possible.

What is the fewest steps required to move starting from any square with elevation a to the location that should get the best signal?

*/

static INPUT_FILE: &str = "data/day12/input.txt";

pub fn run() {
    println!("Day12: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day12: End");
}

struct Day {
    part1: bool,
    grid: Vec<usize>,
    dim_x: usize,
    dim_y: usize,
    start_index: usize,
    end_index: usize,
}

#[derive(PartialEq, Eq)]
struct Node {
    index: usize,
    length: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.length.cmp(&self.length)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day {
    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            grid: Vec::new(),
            dim_x: 0,
            dim_y: 0,
            start_index: 0,
            end_index: 0,
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.part1();
            println!("Day12: Result1 {result1}");
            let expected = 350;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.part2();
            println!("Day12: Result2 {result2}");
            let expected = 349;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        self.dim_y = lines.len();
        self.dim_x = lines[0].len();
        self.grid.resize(self.dim_x * self.dim_y, std::usize::MAX);
        let mut index = 0;
        for line in lines {
            assert_eq!(line.len(), self.dim_x);
            let bytes = line.as_bytes();
            for x in 0..self.dim_x {
                let b = bytes[x];
                // S = 'a'
                // E = 'z'
                let height: usize;
                if b == ('S' as u8) {
                    self.start_index = index;
                    height = 0;
                } else if b == ('E' as u8) {
                    self.end_index = index;
                    height = 25;
                } else {
                    height = b as usize - ('a' as usize);
                }
                assert!(height <= 25);
                self.grid[index] = height;
                index += 1;
            }
        }
    }

    fn find_shortest_path(&self, starting_point: usize, max_length: usize) -> usize {
        let mut visited_grid: Vec<usize> = Vec::new();
        let dim_x = self.dim_x;
        let dim_y = self.dim_y;
        visited_grid.resize(dim_x * dim_y, std::usize::MAX);
        let mut min_length = std::usize::MAX;
        let mut nodes = BinaryHeap::new();
        nodes.push(Node {
            index: starting_point,
            length: 0,
        });
        while !nodes.is_empty() {
            let node = nodes.pop().unwrap();
            let current_index = node.index;
            let current_length = node.length;
            if current_index == self.end_index {
                if current_length < min_length {
                    min_length = current_length;
                }
                continue;
            }
            if current_length >= visited_grid[current_index] {
                continue;
            }
            let new_length = current_length + 1;
            if new_length >= max_length {
                continue;
            }
            visited_grid[current_index] = current_length;
            let current_height = self.grid[current_index];
            let max_height = current_height + 1;
            let x = current_index % dim_x;
            let y = current_index / dim_x;
            assert_eq!(y * dim_x + x, current_index);
            // if up is valid : push up, len +=1
            if current_index >= dim_x {
                let new_index = current_index - dim_x;
                if self.grid[new_index] <= max_height {
                    nodes.push(Node {
                        index: new_index,
                        length: new_length,
                    });
                }
            }
            // if down is valid : push down, len +=1
            if y < dim_y - 1 {
                let new_index = current_index + dim_x;
                if self.grid[new_index] <= max_height {
                    nodes.push(Node {
                        index: new_index,
                        length: new_length,
                    });
                }
            }
            // if left is valid : push left, len +=1
            if x > 0 {
                let new_index = current_index - 1;
                if self.grid[new_index] <= max_height {
                    nodes.push(Node {
                        index: new_index,
                        length: new_length,
                    });
                }
            }
            // if right is valid : push right, len +=1
            if x < dim_x - 1 {
                let new_index = current_index + 1;
                if self.grid[new_index] <= max_height {
                    nodes.push(Node {
                        index: new_index,
                        length: new_length,
                    });
                }
            }
        }
        return min_length;
    }

    fn part1(&self) -> usize {
        let length = self.find_shortest_path(self.start_index, std::usize::MAX);
        return length;
    }

    fn part2(&self) -> usize {
        let mut min_length = std::usize::MAX;
        for i in 0..self.grid.len() {
            let h = self.grid[i];
            if h == 0 {
                let length = self.find_shortest_path(i, min_length);
                if length < min_length {
                    min_length = length;
                }
            }
        }
        return min_length;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::str_array_to_string_array;

    #[test]
    fn part1() {
        let input: (Vec<&str>, usize) = (
            vec!["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"],
            31,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.part1(), input.1);
    }

    #[test]
    fn part2() {
        let input: (Vec<&str>, usize) = (
            vec!["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"],
            29,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.part2(), input.1);
    }
}

/*

Walk graph

nodes.push (index:start_index, len:0)

while !nodes.empty
  node = nodes.pop;
  if node == end_index : min_len = min(len,min_len) : continue
  if up is valid : push up, len +=1
  if down is valid : push down, len +=1
  if left is valid : push left, len +=1
  if right is valid : push right, len +=1

*/
