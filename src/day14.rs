use crate::file_to_vec;

/*

--- Day 14: Regolith Reservoir ---

The distress signal leads you to a giant waterfall! Actually, hang on - the signal seems like it's coming from the waterfall itself, and that doesn't make any sense.
However, you do notice a little path that leads behind the waterfall.

Correction: the distress signal leads you behind a giant waterfall!
There seems to be a large cave system here, and the signal definitely leads further inside.

As you begin to make your way deeper underground, you feel the ground rumble for a moment.
Sand begins pouring into the cave!
If you don't quickly figure out where the sand is going, you could quickly become trapped!

Fortunately, your familiarity with analyzing the path of falling material will come in handy here.
You scan a two-dimensional vertical slice of the cave above you (your puzzle input) and discover that it is mostly air with structures made of rock.

Your scan traces the path of each solid rock structure and reports the x,y coordinates that form the shape of the path, where x represents distance to the right and y represents distance down.
Each path appears as a single line of text in your scan.
After the first point of each path, each point indicates the end of a straight horizontal or vertical line to be drawn from the previous point.
For example:

498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
This scan means that there are two paths of rock; the first path consists of two straight lines, and the second path consists of three straight lines.
(Specifically, the first path consists of a line of rock from 498,4 through 498,6 and another line of rock from 498,6 through 496,6.)

The sand is pouring into the cave from point 500,0.

Drawing rock as #, air as ., and the source of the sand as +, this becomes:


  4     5  5
  9     0  0
  4     0  3
0 ......+...
1 ..........
2 ..........
3 ..........
4 ....#...##
5 ....#...#.
6 ..###...#.
7 ........#.
8 ........#.
9 #########.

Sand is produced one unit at a time, and the next unit of sand is not produced until the previous unit of sand comes to rest.
A unit of sand is large enough to fill one tile of air in your scan.

A unit of sand always falls down one step if possible.
If the tile immediately below is blocked (by rock or sand), the unit of sand attempts to instead move diagonally one step down and to the left.
If that tile is blocked, the unit of sand attempts to instead move diagonally one step down and to the right.
Sand keeps moving as long as it is able to do so, at each step trying to move down, then down-left, then down-right.
If all three possible destinations are blocked, the unit of sand comes to rest and no longer moves, at which point the next unit of sand is created back at the source.

So, drawing sand that has come to rest as o, the first unit of sand simply falls straight down and then stops:

......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
......o.#.
#########.

The second unit of sand then falls straight down, lands on the first one, and then comes to rest to its left:

......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
.....oo.#.
#########.

After a total of five units of sand have come to rest, they form this pattern:

......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
......o.#.
....oooo#.
#########.
After a total of 22 units of sand:

......+...
..........
......o...
.....ooo..
....#ooo##
....#ooo#.
..###ooo#.
....oooo#.
...ooooo#.
#########.

Finally, only two more units of sand can possibly come to rest:

......+...
..........
......o...
.....ooo..
....#ooo##
...o#ooo#.
..###ooo#.
....oooo#.
.o.ooooo#.
#########.

Once all 24 units of sand shown above have come to rest, all further sand flows out the bottom, falling into the endless void.

Just for fun, the path any new sand takes before falling forever is shown here with ~:

.......+...
.......~...
......~o...
.....~ooo..
....~#ooo##
...~o#ooo#.
..~###ooo#.
..~..oooo#.
.~o.ooooo#.
~#########.
~..........
~..........
~..........

Using your scan, simulate the falling sand.

How many units of sand come to rest before sand starts flowing into the abyss below?

Your puzzle answer was 1003.

--- Part Two ---

You realize you misread the scan.
There isn't an endless void at the bottom of the scan - there's floor, and you're standing on it!

You don't have time to scan the floor, so assume the floor is an infinite horizontal line with a y coordinate equal to two plus the highest y coordinate of any point in your scan.

In the example above, the highest y coordinate of any point is 9, and so the floor is at y=11.
(This is as if your scan contained one extra rock path like -infinity,11 -> infinity,11.) With the added floor, the example above now looks like this:

        ...........+........
        ....................
        ....................
        ....................
        .........#...##.....
        .........#...#......
        .......###...#......
        .............#......
        .............#......
        .....#########......
        ....................
<-- etc #################### etc -->

To find somewhere safe to stand, you'll need to simulate falling sand until a unit of sand comes to rest at 500,0, blocking the source entirely and stopping the flow of sand into the cave.
In the example above, the situation finally looks like this after 93 units of sand come to rest:

............o............
...........ooo...........
..........ooooo..........
.........ooooooo.........
........oo#ooo##o........
.......ooo#ooo#ooo.......
......oo###ooo#oooo......
.....oooo.oooo#ooooo.....
....oooooooooo#oooooo....
...ooo#########ooooooo...
..ooooo.......ooooooooo..
#########################

Using your scan, simulate the falling sand until the source of the sand becomes blocked.

How many units of sand come to rest?

*/

static INPUT_FILE: &str = "data/day14/input.txt";

pub fn run() {
    println!("Day14: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day14: End");
}

struct Day {
    part1: bool,
    line_starts: Vec<(i32, i32)>,
    line_ends: Vec<(i32, i32)>,
    grid: Vec<u8>,
    grid_min: (i32, i32),
    grid_max: (i32, i32),
    grid_size: (i32, i32),
    rock_bottom: i32,
}

impl Day {
    const EMPTY: u8 = 0;
    const ROCK: u8 = 1;
    const WATER: u8 = 2;

    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            line_starts: Vec::new(),
            line_ends: Vec::new(),
            grid: Vec::new(),
            grid_min: (0, 0),
            grid_max: (0, 0),
            grid_size: (0, 0),
            rock_bottom: 0,
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.part1();
            println!("Day14: Result1 {result1}");
            let expected = 1003;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.part2();
            println!("Day14: Result2 {result2}");
            let expected = 25771;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        // Parse input to make lines to fill then compute maximum size of the required grid
        let mut min_x = std::i32::MAX;
        let mut max_x = std::i32::MIN;
        let mut min_y = std::i32::MAX;
        let mut max_y = std::i32::MIN;
        for line in lines {
            let point_toks: Vec<&str> = line.split(" -> ").collect();
            // 498,4 -> 498,6 -> 496,6
            // line 0 498,4 to 498,6
            // line 1 498,6 to 496,6
            let xy_toks: Vec<&str> = point_toks[0].split(",").collect();
            let mut start = (
                xy_toks[0].parse().expect("Not a number"),
                xy_toks[1].parse().expect("Not a number"),
            );
            min_x = std::cmp::min(min_x, start.0);
            min_y = std::cmp::min(min_y, start.1);
            max_x = std::cmp::max(max_x, start.0);
            max_y = std::cmp::max(max_y, start.1);
            for p in 1..point_toks.len() {
                self.line_starts.push(start);
                // 498,4 -> x,y
                let xy_toks: Vec<&str> = point_toks[p].split(",").collect();
                let end = (
                    xy_toks[0].parse().expect("Not a number"),
                    xy_toks[1].parse().expect("Not a number"),
                );
                min_x = std::cmp::min(min_x, end.0);
                min_y = std::cmp::min(min_y, end.1);
                max_x = std::cmp::max(max_x, end.0);
                max_y = std::cmp::max(max_y, end.1);
                self.line_ends.push(end);
                start = end;
            }
        }
        assert!(min_x >= 10);
        assert!(min_x < 500);
        assert!(max_x > 500);
        self.rock_bottom = max_y;
        min_x = 0;
        min_x -= max_y;
        min_y = 0;
        max_x += max_y;
        max_y += 3;
        self.grid_min = (min_x, min_y);
        self.grid_max = (max_x, max_y);
        self.grid_size = (
            self.grid_max.0 - self.grid_min.0,
            self.grid_max.1 - self.grid_min.1,
        );
        self.grid
            .resize((self.grid_size.0 * self.grid_size.1) as usize, Day::EMPTY);
        assert_eq!(self.line_starts.len(), self.line_ends.len());
        self.fill_grid();
    }

    fn grid_index(&self, x: i32, y: i32) -> usize {
        let index = (y - self.grid_min.1) * self.grid_size.0 + (x - self.grid_min.0);
        return index as usize;
    }

    fn fill_grid(&mut self) {
        for l in 0..self.line_starts.len() {
            let mut start = self.line_starts[l];
            let mut end = self.line_ends[l];
            if start.0 > end.0 {
                let t = end;
                end = start;
                start = t;
            } else if start.1 > end.1 {
                let t = end;
                end = start;
                start = t;
            }
            assert!(start.0 <= end.0);
            assert!(start.1 <= end.1);
            assert!((start.0 < end.0 && start.1 == end.1) || (start.1 < end.1 && start.0 == end.0));
            let len: i32;
            let dx;
            let dy;
            if start.0 < end.0 {
                len = end.0 - start.0 + 1;
                dx = 1;
                dy = 0;
            } else {
                assert!(start.1 < end.1);
                len = end.1 - start.1 + 1;
                dx = 0;
                dy = 1;
            }
            let mut x = start.0;
            let mut y = start.1;
            for _p in 0..len {
                let index = self.grid_index(x, y);
                self.grid[index] = Day::ROCK;
                x += dx;
                y += dy;
            }
        }
    }

    fn drop_sand(&mut self, x0: i32, y0: i32) -> bool {
        let mut x = x0;
        let mut y = y0;
        while y <= self.rock_bottom {
            assert!(x >= self.grid_min.0);
            assert!(x < self.grid_max.0);
            assert!(y >= self.grid_min.1);
            assert!(y < self.grid_max.1);
            // Sand tries to move down, then down-left, then down-right.
            // If all three possible destinations are blocked, sand comes to rest
            let index = self.grid_index(x, y + 1);
            if self.grid[index] == Day::EMPTY {
                y = y + 1;
                continue;
            }
            let index = self.grid_index(x - 1, y + 1);
            if self.grid[index] == Day::EMPTY {
                x = x - 1;
                y = y + 1;
                continue;
            }
            let index = self.grid_index(x + 1, y + 1);
            if self.grid[index] == Day::EMPTY {
                x = x + 1;
                y = y + 1;
                continue;
            }
            let index = self.grid_index(x, y);
            self.grid[index] = Day::WATER;
            return true;
        }
        return false;
    }

    fn output_grid(&self) {
        println!("");
        for y in self.grid_min.1..self.grid_max.1 {
            for x in self.grid_min.0..self.grid_max.0 {
                let grid = self.grid[self.grid_index(x, y)];
                let c;
                if grid == Day::EMPTY {
                    c = ".";
                } else if grid == Day::ROCK {
                    c = "#";
                } else if grid == Day::WATER {
                    c = "o";
                } else {
                    panic!("Unknown grid value {grid}");
                }
                print!("{c}");
            }
            println!("");
        }
        println!("");
    }

    fn part1(&mut self) -> usize {
        return self.simulate(100000);
    }

    fn simulate(&mut self, max_loops: usize) -> usize {
        let mut total = 0;
        let start_index = self.grid_index(500, 0);
        for _ in 0..max_loops {
            // Part1 drop sand from 500,0 until it doesn't stop
            // Part2 drop sand from 500,0 until it stops at 500,0
            if !self.drop_sand(500, 0) {
                return total;
            }
            total += 1;
            if self.grid[start_index] == Day::WATER {
                return total;
            }
        }
        self.output_grid();
        panic!("Did not settle ran out of loops {max_loops}");
    }

    fn part2(&mut self) -> usize {
        self.rock_bottom += 2;
        let y = self.rock_bottom;
        for x in self.grid_min.0..self.grid_max.0 {
            let index = self.grid_index(x, y);
            self.grid[index] = Day::ROCK;
        }
        return self.simulate(100000);
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
                "498,4 -> 498,6 -> 496,6",
                "503,4 -> 502,4 -> 502,9 -> 494,9",
            ],
            24,
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
                "498,4 -> 498,6 -> 496,6",
                "503,4 -> 502,4 -> 502,9 -> 494,9",
            ],
            93,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(false);
        day.parse(&lines);
        assert_eq!(day.part2(), input.1);
    }
}
