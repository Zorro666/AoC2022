use crate::file_to_vec;

/*

--- Day 9: Rope Bridge ---

This rope bridge creaks as you walk along it.
You aren't sure how old it is, or whether it can even support your weight.

It seems to support the Elves just fine, though.
The bridge spans a gorge which was carved out by the massive river far below you.

You step carefully; as you do, the ropes stretch and twist.
You decide to distract yourself by modeling rope physics; maybe you can even figure out where not to step.

Consider a rope with a knot at each end; these knots mark the head and the tail of the rope.
If the head moves far enough away from the tail, the tail is pulled toward the head.

Due to nebulous reasoning involving Planck lengths, you should be able to model the positions of the knots on a two-dimensional grid.
Then, by following a hypothetical series of motions (your puzzle input) for the head, you can determine how the tail will move.

Due to the aforementioned Planck lengths, the rope must be quite short; in fact, the head (H) and tail (T) must always be touching (diagonally adjacent and even overlapping both count as touching):

....
.TH.
....

....
.H..
..T.
....

...
.H. (H covers T)
...
If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one step in that direction so it remains close enough:

.....    .....    .....
.TH.. -> .T.H. -> ..TH.
.....    .....    .....

...    ...    ...
.T.    .T.    ...
.H. -> ... -> .T.
...    .H.    .H.
...    ...    ...

Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step diagonally to keep up:

.....    .....    .....
.....    ..H..    ..H..
..H.. -> ..... -> ..T..
.T...    .T...    .....
.....    .....    .....

.....    .....    .....
.....    .....    .....
..H.. -> ...H. -> ..TH.
.T...    .T...    .....
.....    .....    .....

You just need to work out where the tail goes as the head follows a series of motions.
Assume the head and the tail both start at the same position, overlapping.

For example:

R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2

This series of motions moves the head right four steps, then up four steps, then left three steps, then down one step, and so on.
After each step, you'll need to update the position of the tail if the step means the head is no longer adjacent to the tail.
Visually, these motions occur as follows (s marks the starting position as a reference point):

== Initial State ==

......
......
......
......
H.....  (H covers T, s)

== R 4 ==

......
......
......
......
TH....  (T covers s)

......
......
......
......
sTH...

......
......
......
......
s.TH..

......
......
......
......
s..TH.

== U 4 ==

......
......
......
....H.
s..T..

......
......
....H.
....T.
s.....

......
....H.
....T.
......
s.....

....H.
....T.
......
......
s.....

== L 3 ==

...H..
....T.
......
......
s.....

..HT..
......
......
......
s.....

.HT...
......
......
......
s.....

== D 1 ==

..T...
.H....
......
......
s.....

== R 4 ==

..T...
..H...
......
......
s.....

..T...
...H..
......
......
s.....

......
...TH.
......
......
s.....

......
....TH
......
......
s.....

== D 1 ==

......
....T.
.....H
......
s.....

== L 5 ==

......
....T.
....H.
......
s.....

......
....T.
...H..
......
s.....

......
......
..HT..
......
s.....

......
......
.HT...
......
s.....

......
......
HT....
......
s.....

== R 2 ==

......
......
.H....  (H covers T)
......
s.....

......
......
.TH...
......
s.....

After simulating the rope, you can count up all of the positions the tail visited at least once.
In this diagram, s again marks the starting position (which the tail also visited) and # marks other positions the tail visited:

..##..
...##.
.####.
....#.
s###..

So, there are 13 positions the tail visited at least once.

Simulate your complete hypothetical series of motions.

How many positions does the tail of the rope visit at least once?

Your puzzle answer was 6087.

--- Part Two ---

A rope snaps!
Suddenly, the river is getting a lot closer than you remember.
The bridge is still there, but some of the ropes that broke are now whipping toward you as you fall through the air!

The ropes are moving too quickly to grab; you only have a few seconds to choose how to arch your body to avoid being hit.
Fortunately, your simulation can be extended to support longer ropes.

Rather than two knots, you now must simulate a rope consisting of ten knots.
One knot is still the head of the rope and moves according to the series of motions.
Each knot further down the rope follows the knot in front of it using the same rules as before.

Using the same series of motions as the above example, but with the knots marked H, 1, 2, ..., 9, the motions now occur as follows:

== Initial State ==

......
......
......
......
H.....  (H covers 1, 2, 3, 4, 5, 6, 7, 8, 9, s)

== R 4 ==

......
......
......
......
1H....  (1 covers 2, 3, 4, 5, 6, 7, 8, 9, s)

......
......
......
......
21H...  (2 covers 3, 4, 5, 6, 7, 8, 9, s)

......
......
......
......
321H..  (3 covers 4, 5, 6, 7, 8, 9, s)

......
......
......
......
4321H.  (4 covers 5, 6, 7, 8, 9, s)

== U 4 ==

......
......
......
....H.
4321..  (4 covers 5, 6, 7, 8, 9, s)

......
......
....H.
.4321.
5.....  (5 covers 6, 7, 8, 9, s)

......
....H.
....1.
.432..
5.....  (5 covers 6, 7, 8, 9, s)

....H.
....1.
..432.
.5....
6.....  (6 covers 7, 8, 9, s)

== L 3 ==

...H..
....1.
..432.
.5....
6.....  (6 covers 7, 8, 9, s)

..H1..
...2..
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

.H1...
...2..
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

== D 1 ==

..1...
.H.2..
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

== R 4 ==

..1...
..H2..
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

..1...
...H..  (H covers 2)
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

......
...1H.  (1 covers 2)
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

......
...21H
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

== D 1 ==

......
...21.
..43.H
.5....
6.....  (6 covers 7, 8, 9, s)

== L 5 ==

......
...21.
..43H.
.5....
6.....  (6 covers 7, 8, 9, s)

......
...21.
..4H..  (H covers 3)
.5....
6.....  (6 covers 7, 8, 9, s)

......
...2..
..H1..  (H covers 4; 1 covers 3)
.5....
6.....  (6 covers 7, 8, 9, s)

......
...2..
.H13..  (1 covers 4)
.5....
6.....  (6 covers 7, 8, 9, s)

......
......
H123..  (2 covers 4)
.5....
6.....  (6 covers 7, 8, 9, s)

== R 2 ==

......
......
.H23..  (H covers 1; 2 covers 4)
.5....
6.....  (6 covers 7, 8, 9, s)

......
......
.1H3..  (H covers 2, 4)
.5....
6.....  (6 covers 7, 8, 9, s)

Now, you need to keep track of the positions the new tail, 9, visits.
In this example, the tail never moves, and so it only visits 1 position.
However, be careful: more types of motion are possible than before, so you might want to visually compare your simulated rope to the one above.

Here's a larger example:

R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20

These motions occur as follows (individual steps are not shown):

== Initial State ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
...........H..............  (H covers 1, 2, 3, 4, 5, 6, 7, 8, 9, s)
..........................
..........................
..........................
..........................
..........................

== R 5 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
...........54321H.........  (5 covers 6, 7, 8, 9, s)
..........................
..........................
..........................
..........................
..........................

== U 8 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
................H.........
................1.........
................2.........
................3.........
...............54.........
..............6...........
.............7............
............8.............
...........9..............  (9 covers s)
..........................
..........................
..........................
..........................
..........................

== L 8 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
........H1234.............
............5.............
............6.............
............7.............
............8.............
............9.............
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
..........................

== D 3 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
.........2345.............
........1...6.............
........H...7.............
............8.............
............9.............
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
..........................

== R 17 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
................987654321H
..........................
..........................
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
..........................

== D 10 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
...........s.........98765
.........................4
.........................3
.........................2
.........................1
.........................H

== L 25 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
H123456789................

== U 20 ==

H.........................
1.........................
2.........................
3.........................
4.........................
5.........................
6.........................
7.........................
8.........................
9.........................
..........................
..........................
..........................
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
..........................

Now, the tail (9) visits 36 positions (including s) at least once:

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
#.........................
#.............###.........
#............#...#........
.#..........#.....#.......
..#..........#.....#......
...#........#.......#.....
....#......s.........#....
.....#..............#.....
......#............#......
.......#..........#.......
........#........#........
.........########.........

Simulate your complete series of motions on a larger rope with ten knots.

How many positions does the tail of the rope visit at least once?

*/

static INPUT_FILE: &str = "data/day09/input.txt";

pub fn run() {
    println!("Day09: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day09: End");
}

struct Day {
    part1: bool,
    move_dxs: Vec<i32>,
    move_dys: Vec<i32>,
    step_counts: Vec<i32>,
}

impl Day {
    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            move_dxs: Vec::new(),
            move_dys: Vec::new(),
            step_counts: Vec::new(),
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.count_tail_positions(2);
            println!("Day09: Result1 {result1}");
            let expected = 6087;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.count_tail_positions(10);
            println!("Day09: Result2 {result2}");
            let expected = 2493;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        for line in lines {
            // R 4
            let bytes = line.as_bytes();
            let move_dir = bytes[0];
            let mut dx = 0;
            let mut dy = 0;
            if move_dir == 'R' as u8 {
                dx = 1;
            } else if move_dir == 'L' as u8 {
                dx = -1;
            } else if move_dir == 'U' as u8 {
                dy = 1;
            } else if move_dir == 'D' as u8 {
                dy = -1;
            }
            self.move_dxs.push(dx);
            self.move_dys.push(dy);
            let step_count: i32 = line[2..].parse().expect("Not a number");
            self.step_counts.push(step_count);
        }
    }

    fn count_tail_positions(&self, rope_length: usize) -> i64 {
        let count = self.move_dxs.len();
        let mut knot_xs = Vec::new();
        let mut knot_ys = Vec::new();
        knot_xs.resize(rope_length, 0);
        knot_ys.resize(rope_length, 0);
        let mut tail_xys: Vec<(i32, i32)> = Vec::new();
        tail_xys.push((0, 0));
        for i in 0..count {
            let dx = self.move_dxs[i];
            let dy = self.move_dys[i];
            let step_count = self.step_counts[i];
            for _s in 0..step_count {
                knot_xs[0] += dx;
                knot_ys[0] += dy;
                for k in 1..rope_length {
                    let mut knot_x = knot_xs[k];
                    let mut knot_y = knot_ys[k];
                    let mut knot_dx = knot_xs[k - 1] - knot_x;
                    let mut knot_dy = knot_ys[k - 1] - knot_y;
                    // the head and tail (T) must always be touching (including diagonally adjacent and overlapping)
                    if knot_dx >= -1 && knot_dx <= 1 && knot_dy >= -1 && knot_dy <= 1 {
                        continue;
                    }
                    // if head is two steps directly up, down, left, or right from tail, tail must move one step in that direction
                    // head and tail aren't touching and not in same row or column, tail moves one step diagonally to keep up
                    if knot_dx < 0 {
                        knot_dx = -1;
                    } else if knot_dx > 0 {
                        knot_dx = 1;
                    }
                    if knot_dy < 0 {
                        knot_dy = -1;
                    } else if knot_dy > 0 {
                        knot_dy = 1;
                    }
                    knot_x += knot_dx;
                    knot_y += knot_dy;
                    knot_xs[k] = knot_x;
                    knot_ys[k] = knot_y;
                }
                let tail_x = knot_xs[rope_length - 1];
                let tail_y = knot_ys[rope_length - 1];
                if !tail_xys.contains(&(tail_x, tail_y)) {
                    tail_xys.push((tail_x, tail_y));
                }
            }
        }
        return tail_xys.len() as i64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::str_array_to_string_array;

    #[test]
    fn part1() {
        let input: (Vec<&str>, i64) = (
            vec!["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"],
            13,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.count_tail_positions(2), input.1);
    }

    #[test]
    fn part2() {
        let input: (Vec<&str>, i64) = (
            vec!["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"],
            36,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(false);
        day.parse(&lines);
        assert_eq!(day.count_tail_positions(10), input.1);
    }
}
