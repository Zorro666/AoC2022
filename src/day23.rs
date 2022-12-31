use crate::file_to_vec;
use std::collections::HashMap;

/*

--- Day 23: Unstable Diffusion ---

You enter a large crater of gray dirt where the grove is supposed to be.
All around you, plants you imagine were expected to be full of fruit are instead withered and broken.
A large group of Elves has formed in the middle of the grove.

"...but this volcano has been dormant for months.
Without ash, the fruit can't grow!"

You look up to see a massive, snow-capped mountain towering above you.

"It's not like there are other active volcanoes here; we've looked everywhere."

"But our scanners show active magma flows; clearly it's going somewhere."

They finally notice you at the edge of the grove, your pack almost overflowing from the random star fruit you've been collecting.
Behind you, elephants and monkeys explore the grove, looking concerned.
Then, the Elves recognize the ash cloud slowly spreading above your recent detour.

"Why do you--" "How is--" "Did you just--"

Before any of them can form a complete question, another Elf speaks up: "Okay, new plan.
We have almost enough fruit already, and ash from the plume should spread here eventually.
If we quickly plant new seedlings now, we can still make it to the extraction point.
Spread out!"

The Elves each reach into their pack and pull out a tiny plant.
The plants rely on important nutrients from the ash, so they can't be planted too close together.

There isn't enough time to let the Elves figure out where to plant the seedlings themselves; you quickly scan the grove (your puzzle input) and note their positions.

For example:

....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..

The scan shows Elves # and empty ground .; outside your scan, more empty ground extends a long way in every direction.
The scan is oriented so that north is up; orthogonal directions are written N (north), S (south), W (west), and E (east), while diagonal directions are written NE, NW, SE, SW.

The Elves follow a time-consuming process to figure out where they should each go; you can speed up this process considerably.
The process consists of some number of rounds during which Elves alternate between considering where to move and actually moving.

During the first half of each round, each Elf considers the eight positions adjacent to themself.
If no other Elves are in one of those eight positions, the Elf does not do anything during this round.
Otherwise, the Elf looks in each of four directions in the following order and proposes moving one step in the first valid direction:

If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.

After each Elf has had a chance to propose a move, the second half of the round can begin.

Simultaneously, each Elf moves to their proposed destination tile if they were the only Elf to propose moving to that position.
If two or more Elves propose moving to the same position, none of those Elves move.

Finally, at the end of the round, the first direction the Elves considered is moved to the end of the list of directions.
For example, during the second round, the Elves would try proposing a move to the south first, then west, then east, then north.
On the third round, the Elves would first consider west, then east, then north, then south.

As a smaller example, consider just these five Elves:

.....
..##.
..#..
.....
..##.
.....
The northernmost two Elves and southernmost two Elves all propose moving north, while the middle Elf cannot move north and proposes moving south.
The middle Elf proposes the same destination as the southwest Elf, so neither of them move, but the other three do:

..##.
.....
..#..
...#.
..#..
.....

Next, the northernmost two Elves and the southernmost Elf all propose moving south.
Of the remaining middle two Elves, the west one cannot move south and proposes moving west, while the east one cannot move south or west and proposes moving east.
All five Elves succeed in moving to their proposed positions:

.....
..##.
.#...
....#
.....
..#..

Finally, the southernmost two Elves choose not to move at all.
Of the remaining three Elves, the west one proposes moving west, the east one proposes moving east, and the middle one proposes moving north; all three succeed in moving:

..#..
....#
#....
....#
.....
..#..

At this point, no Elves need to move, and so the process ends.

The larger example above proceeds as follows:

== Initial State ==
..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............

== End of Round 1 ==
..............
.......#......
.....#...#....
...#..#.#.....
.......#..#...
....#.#.##....
..#..#.#......
..#.#.#.##....
..............
....#..#......
..............
..............

== End of Round 2 ==
..............
.......#......
....#.....#...
...#..#.#.....
.......#...#..
...#..#.#.....
.#...#.#.#....
..............
..#.#.#.##....
....#..#......
..............
..............

== End of Round 3 ==
..............
.......#......
.....#....#...
..#..#...#....
.......#...#..
...#..#.#.....
.#..#.....#...
.......##.....
..##.#....#...
...#..........
.......#......
..............

== End of Round 4 ==
..............
.......#......
......#....#..
..#...##......
...#.....#.#..
.........#....
.#...###..#...
..#......#....
....##....#...
....#.........
.......#......
..............

== End of Round 5 ==
.......#......
..............
..#..#.....#..
.........#....
......##...#..
.#.#.####.....
...........#..
....##..#.....
..#...........
..........#...
....#..#......
..............
After a few more rounds...

== End of Round 10 ==
.......#......
...........#..
..#.#..#......
......#.......
...#.....#..#.
.#......##....
.....##.......
..#........#..
....#.#..#....
..............
....#..#..#...
..............

To make sure they're on the right track, the Elves like to check after round 10 that they're making good progress toward covering enough ground.
To do this, count the number of empty ground tiles contained by the smallest rectangle that contains every Elf.
(The edges of the rectangle should be aligned to the N/S/E/W directions; the Elves do not have the patience to calculate arbitrary rectangles.)
In the above example, that rectangle is:

......#.....
..........#.
.#.#..#.....
.....#......
..#.....#..#
#......##...
....##......
.#........#.
...#.#..#...
............
...#..#..#..

In this region, the number of empty ground tiles is 110.

Simulate the Elves' process and find the smallest rectangle that contains the Elves after 10 rounds.

How many empty ground tiles does that rectangle contain?

Your puzzle answer was 4109.

--- Part Two ---

It seems you're on the right track.
Finish simulating the process and figure out where the Elves need to go.
How many rounds did you save them?

In the example above, the first round where no Elf moved was round 20:

.......#......
....#......#..
..#.....#.....
......#.......
...#....#.#..#
#.............
....#.....#...
..#.....#.....
....#.#....#..
.........#....
....#......#..
.......#......

Figure out where the Elves need to go.

What is the number of the first round where no Elf moves?

*/

static INPUT_FILE: &str = "data/day23/input.txt";

pub fn run() {
    println!("Day23: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day23: End");
}

struct Day {
    part1: bool,
    elves: Vec<usize>,
    grid: Vec<bool>,
    elf_count: usize,
    grid_min_x: usize,
    grid_min_y: usize,
    grid_max_x: usize,
    grid_max_y: usize,
}

impl Day {
    // const EMPTY: char = '.';
    const ELF: char = '#';
    const MOVE_FREE: usize = usize::MAX;
    const MOVE_COLLIDE: usize = usize::MAX - 1;
    const GRID_WIDTH: usize = 2 * 1024;

    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            elves: Vec::new(),
            grid: Vec::new(),
            elf_count: 0,
            grid_min_x: usize::MAX,
            grid_min_y: usize::MAX,
            grid_max_x: usize::MIN,
            grid_max_y: usize::MIN,
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.part1(10);
            println!("Day23: Result1 {result1}");
            let expected = 4109;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.part2();
            println!("Day23: Result2 {result2}");
            let expected = 1055;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        assert!(Day::GRID_WIDTH > lines.len() * 16);
        let y0 = Day::GRID_WIDTH / 2;
        let x0 = Day::GRID_WIDTH / 2;
        let mut y = y0;
        self.grid.resize(2048 * 2048, false);
        for line in lines {
            assert!(Day::GRID_WIDTH > line.len() * 16);
            let bytes = line.bytes();
            let mut x = x0;
            for c in bytes {
                let index = Day::get_index_from_xy(x, y);
                if c == Day::ELF as u8 {
                    self.elf_count += 1;
                    self.elves.push(index);
                    self.grid[index] = true;
                }
                x += 1;
            }
            y += 1;
        }
    }

    fn get_index_from_xy(x: usize, y: usize) -> usize {
        return y * Day::GRID_WIDTH + x;
    }

    fn get_xy_from_index(index: usize) -> (usize, usize) {
        let y = index / Day::GRID_WIDTH;
        let x = index % Day::GRID_WIDTH;
        assert_eq!(index, Day::get_index_from_xy(x, y));
        return (x, y);
    }

    fn compute_min_max(&mut self) {
        for e in 0..self.elf_count {
            let index = self.elves[e];
            let (x, y) = Day::get_xy_from_index(index);
            self.grid_min_x = self.grid_min_x.min(x);
            self.grid_max_x = self.grid_max_x.max(x);
            self.grid_min_y = self.grid_min_y.min(y);
            self.grid_max_y = self.grid_max_y.max(y);
        }
    }

    fn simulate_round(&mut self, round: usize) -> bool {
        let mut new_moves: HashMap<usize, usize> = HashMap::new();
        for elf in 0..self.elf_count {
            let mut neighbours = [0; 9];
            let mut count_neighbours = 0;
            let from_index = self.elves[elf];
            let (x, y) = Day::get_xy_from_index(from_index);
            for iy in 0..3 {
                let yn = y - 1 + iy;
                for ix in 0..3 {
                    let xn = x - 1 + ix;
                    let idx = Day::get_index_from_xy(xn, yn);
                    if idx != from_index {
                        if self.grid[idx] {
                            let n_index = ix + iy * 3;
                            neighbours[n_index] = 1;
                            count_neighbours += 1;
                        }
                    }
                }
            }
            if count_neighbours == 0 {
                continue;
            }
            let nw = neighbours[0];
            let n = neighbours[1];
            let ne = neighbours[2];
            let w = neighbours[3];
            let e = neighbours[5];
            let sw = neighbours[6];
            let s = neighbours[7];
            let se = neighbours[8];
            let mut proposed_moves = [
                Day::MOVE_FREE,
                Day::MOVE_FREE,
                Day::MOVE_FREE,
                Day::MOVE_FREE,
            ];
            // no Elf N, NE, or NW positions, go north
            if n == 0 && ne == 0 && nw == 0 {
                proposed_moves[0] = Day::get_index_from_xy(x, y - 1);
            };
            // no Elf S, SE, or SW positions, go south
            if s == 0 && se == 0 && sw == 0 {
                proposed_moves[1] = Day::get_index_from_xy(x, y + 1);
            };
            // no Elf W, NW, or SW positions, go west
            if w == 0 && nw == 0 && sw == 0 {
                proposed_moves[2] = Day::get_index_from_xy(x - 1, y);
            };
            // no Elf E, NE, or SE positions, go east
            if e == 0 && ne == 0 && se == 0 {
                proposed_moves[3] = Day::get_index_from_xy(x + 1, y);
            };
            let mut to_index = Day::MOVE_FREE;
            let rule = round % 4;

            for i in 0..4 {
                let m = (rule + i) % 4;
                if proposed_moves[m] != Day::MOVE_FREE {
                    to_index = proposed_moves[m];
                    break;
                }
            }
            if to_index == Day::MOVE_FREE {
                continue;
            }
            let e = new_moves.entry(to_index);
            e.and_modify(|v| *v = Day::MOVE_COLLIDE).or_insert(elf);
        }

        let mut moved = false;
        for (to_index, elf) in new_moves {
            if elf == Day::MOVE_COLLIDE {
                continue;
            }
            let from_index = self.elves[elf];
            assert_ne!(to_index, from_index);
            self.grid[from_index] = false;
            self.grid[to_index] = true;
            self.elves[elf] = to_index;
            moved = true;
        }
        return moved;
    }

    fn part1(&mut self, count_rounds: usize) -> usize {
        for r in 0..count_rounds {
            // self.compute_min_max();
            // self.output_grid();
            self.simulate_round(r);
        }
        self.compute_min_max();
        let empty_count = (self.grid_max_y - self.grid_min_y + 1)
            * (self.grid_max_x - self.grid_min_x + 1)
            - self.elf_count;
        return empty_count;
    }

    fn part2(&mut self) -> usize {
        let max_rounds: usize = 2000;
        for r in 0..max_rounds {
            if !self.simulate_round(r) {
                return r + 1;
            }
        }
        panic!("Ran out of rounds {max_rounds}");
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
                "....#..", "..###.#", "#...#.#", ".#...##", "#.###..", "##.#.##", ".#..#..",
            ],
            110,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.part1(10), input.1);
    }

    #[test]
    fn part2() {
        let input = (
            vec![
                "....#..", "..###.#", "#...#.#", ".#...##", "#.###..", "##.#.##", ".#..#..",
            ],
            20,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(false);
        day.parse(&lines);
        assert_eq!(day.part2(), input.1);
    }
}
