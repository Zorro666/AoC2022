use crate::file_to_vec;

/*

--- Day 22: Monkey Map ---

The monkeys take you on a surprisingly easy trail through the jungle.
They're even going in roughly the right direction according to your handheld device's Grove Positioning System.

As you walk, the monkeys explain that the grove is protected by a force field.
To pass through the force field, you have to enter a password; doing so involves tracing a specific path on a strangely-shaped board.

At least, you're pretty sure that's what you have to do; the elephants aren't exactly fluent in monkey.

The monkeys give you notes that they took when they last saw the password entered (your puzzle input).

For example:

        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5

The first half of the monkeys' notes is a map of the board.
It is comprised of a set of open tiles (on which you can move, drawn .) and solid walls (tiles which you cannot enter, drawn #).

The second half is a description of the path you must follow.
It consists of alternating numbers and letters:

A number indicates the number of tiles to move in the direction you are facing.
If you run into a wall, you stop moving forward and continue with the next instruction.
A letter indicates whether to turn 90 degrees clockwise (R) or counterclockwise (L).
Turning happens in-place; it does not change your current tile.
So, a path like 10R5 means "go forward 10 tiles, then turn clockwise 90 degrees, then go forward 5 tiles".

You begin the path in the leftmost open tile of the top row of tiles.
Initially, you are facing to the right (from the perspective of how the map is drawn).

If a movement instruction would take you off of the map, you wrap around to the other side of the board.
In other words, if your next tile is off of the board, you should instead look in the direction opposite of your current facing as far as you can until you find the opposite edge of the board, then reappear there.

For example, if you are at A and facing to the right, the tile in front of you is marked B; if you are at C and facing down, the tile in front of you is marked D:

        ...#
        .#..
        #...
        ....
...#.D.....#
........#...
B.#....#...A
.....C....#.
        ...#....
        .....#..
        .#......
        ......#.
It is possible for the next tile (after wrapping around) to be a wall; this still counts as there being a wall in front of you, and so movement stops before you actually wrap to the other side of the board.

By drawing the last facing you had with an arrow on each tile you visit, the full path taken by the above example looks like this:

        >>v#
        .#v.
        #.v.
        ..v.
...#...v..v#
>>>v...>#.>>
..#v...#....
...>>>>v..#.
        ...#....
        .....#..
        .#......
        ......#.

To finish providing the password to this strange input device, you need to determine numbers for your final row, column, and facing as your final position appears from the perspective of the original map.
Rows start from 1 at the top and count downward; columns start from 1 at the left and count rightward.
(In the above example, row 1, column 1 refers to the empty space with no tile on it in the top-left corner.) Facing is 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^).
The final password is the sum of 1000 times the row, 4 times the column, and the facing.

In the above example, the final row is 6, the final column is 8, and the final facing is 0.
So, the final password is 1000 * 6 + 4 * 8 + 0: 6032.

Follow the path given in the monkeys' notes.

What is the final password?

Your puzzle answer was 27492.

--- Part Two ---

As you reach the force field, you think you hear some Elves in the distance.
Perhaps they've already arrived?

You approach the strange input device, but it isn't quite what the monkeys drew in their notes.
Instead, you are met with a large cube; each of its six faces is a square of 50x50 tiles.

To be fair, the monkeys' map does have six 50x50 regions on it.
If you were to carefully fold the map, you should be able to shape it into a cube!

In the example above, the six (smaller, 4x4) faces of the cube are:

        1111
        1111
        1111
        1111
222233334444
222233334444
222233334444
222233334444
        55556666
        55556666
        55556666
        55556666

You still start in the same position and with the same facing as before, but the wrapping rules are different.
Now, if you would walk off the board, you instead proceed around the cube.
From the perspective of the map, this can look a little strange.
In the above example, if you are at A and move to the right, you would arrive at B facing down; if you are at C and move down, you would arrive at D facing up:

        ...#
        .#..
        #...
        ....
...#.......#
........#..A
..#....#....
.D........#.
        ...#..B.
        .....#..
        .#......
        ..C...#.

Walls still block your path, even if they are on a different face of the cube.
If you are at E facing up, your movement is blocked by the wall marked by the arrow:

        ...#
        .#..
     -->#...
        ....
...#..E....#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

Using the same method of drawing the last facing you had with an arrow on each tile you visit, the full path taken by the above example now looks like this:

        >>v#
        .#v.
        #.v.
        ..v.
...#..^...v#
.>>>>>^.#.>>
.^#....#....
.^........#.
        ...#..v.
        .....#v.
        .#v<<<<.
        ..v...#.

The final password is still calculated from your final position and facing from the perspective of the map.

In this example, the final row is 5, the final column is 7, and the final facing is 3, so the final password is 1000 * 5 + 4 * 7 + 3 = 5031.

Fold the map into a cube, then follow the path given in the monkeys' notes.

What is the final password?

*/

static INPUT_FILE: &str = "data/day22/input.txt";

pub fn run() {
    println!("Day22: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day22: End");
}

struct Day {
    part1: bool,
    map: Vec<Vec<u8>>,
    movement: Vec<u8>,
    x_mins: Vec<usize>,
    x_maxs: Vec<usize>,
    y_mins: Vec<usize>,
    y_maxs: Vec<usize>,
    face_size: usize,
    face_data: [Vec<u8>; 6],
    face_connect: [[(usize, usize); 4]; 6],
    face_y_mins: [usize; 6],
    face_x_mins: [usize; 6],
    monkey_x: usize,
    monkey_y: usize,
    monkey_face: usize,
    monkey_facing: usize,
}

impl Day {
    const LEFT: u8 = 'L' as u8;
    const RIGHT: u8 = 'R' as u8;
    const VOID: u8 = ' ' as u8;
    const CLEAR: u8 = '.' as u8;
    const WALL: u8 = '#' as u8;
    const FACE_R: usize = 0;
    const FACE_D: usize = 1;
    const FACE_L: usize = 2;
    const FACE_U: usize = 3;
    const EDGE_TOP: usize = 0;
    const EDGE_RIGHT: usize = 1;
    const EDGE_BOTTOM: usize = 2;
    const EDGE_LEFT: usize = 3;
    const EDGE_UNKNOWN: usize = 4;

    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            map: Vec::new(),
            x_mins: Vec::new(),
            x_maxs: Vec::new(),
            y_mins: Vec::new(),
            y_maxs: Vec::new(),
            movement: Vec::new(),
            face_size: 0,
            face_data: [(); 6].map(|_| Vec::new()),
            face_connect: [[(usize::MAX, usize::MAX); 4]; 6],
            face_y_mins: [usize::MAX; 6],
            face_x_mins: [usize::MAX; 6],
            monkey_x: 0,
            monkey_y: 0,
            monkey_face: 0,
            monkey_facing: 0,
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.part1();
            println!("Day22: Result1 {result1}");
            let expected = 27492;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.part2();
            println!("Day22: Result2 {result2}");
            let expected = 78291;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        let height = lines.len() - 2;
        let width = height;
        self.x_mins.resize(height, usize::MAX);
        self.x_maxs.resize(height, 0);
        self.y_mins.resize(width, usize::MAX);
        self.y_maxs.resize(width, 0);
        for (i, line) in lines.iter().enumerate() {
            if line.is_empty() {
                continue;
            }
            let data = line.as_bytes().to_vec();
            if i != lines.len() - 1 {
                let y = i;
                let x_min = data.iter().position(|&c| c != Day::VOID).unwrap();
                let x_max = data.iter().rposition(|&c| c != Day::VOID).unwrap();
                self.x_mins[y] = x_min;
                self.x_maxs[y] = x_max;
                //     AAAA
                //   BBBB
                // CCC

                //   AA
                // BBBB
                //    CCC
                if x_max >= self.y_mins.len() {
                    self.y_mins.resize(x_max + 1, usize::MAX);
                    self.y_maxs.resize(x_max + 1, 0);
                }
                for x in x_min..=x_max {
                    self.y_mins[x] = self.y_mins[x].min(y);
                    self.y_maxs[x] = self.y_maxs[x].max(y);
                }
                self.map.push(data);
            } else {
                self.movement = data;
            }
        }
        let face_width = (0..self.x_mins.len())
            .map(|y| self.x_maxs[y] - self.x_mins[y])
            .min()
            .unwrap()
            + 1;
        let face_height = (0..self.y_mins.len())
            .map(|x| {
                if self.y_mins[x] != usize::MAX {
                    return self.y_maxs[x] - self.y_mins[x];
                }
                return usize::MAX;
            })
            .min()
            .unwrap()
            + 1;
        assert_eq!(face_height, face_width);
        self.face_size = face_width;
        for f in 0..6 {
            self.face_data[f].resize(face_width * face_height, Day::VOID);
        }
        let mut face = 0;
        for map_y in (0..self.map.len()).step_by(face_height) {
            let x_min = self.x_mins[map_y];
            let x_max = self.x_maxs[map_y];
            let face_count = (x_max - x_min + 1) / face_width;
            for f in 0..face_count {
                let map_x = x_min + f * face_width;
                self.face_x_mins[face] = map_x;
                self.face_y_mins[face] = map_y;
                for y in 0..face_height {
                    for x in 0..face_width {
                        let index = x + y * face_width;
                        self.face_data[face][index] = self.map[y + map_y][x + map_x];
                    }
                }
                face += 1;
            }
        }
        if self.face_size == 50 {
            // Puzzle Input (face_size = 50)
            if self.part1 {
                // not wrapped
                //  12
                //  3
                // 45
                // 6
                // Face 1 TOP => Face 5 BOTTOM
                // Face 1 RIGHT => Face 2 LEFT
                // Face 1 BOTTOM => Face 3 TOP
                // Face 1 LEFT => Face 2 RIGHT
                self.face_connect[0] = [
                    (4, Day::EDGE_BOTTOM),
                    (1, Day::EDGE_LEFT),
                    (2, Day::EDGE_TOP),
                    (1, Day::EDGE_RIGHT),
                ];

                // Face 2 TOP => Face 2 BOTTOM
                // Face 2 RIGHT => Face 1 LEFT
                // Face 2 BOTTOM => Face 2 TOP
                // Face 2 LEFT => Face 1 RIGHT
                self.face_connect[1] = [
                    (1, Day::EDGE_BOTTOM),
                    (0, Day::EDGE_LEFT),
                    (1, Day::EDGE_TOP),
                    (0, Day::EDGE_RIGHT),
                ];

                // Face 3 TOP => Face 1 BOTTOM
                // Face 3 RIGHT => Face 3 LEFT
                // Face 3 BOTTOM => Face 5 TOP
                // Face 3 LEFT => Face 3 RIGHT
                self.face_connect[2] = [
                    (0, Day::EDGE_BOTTOM),
                    (2, Day::EDGE_LEFT),
                    (4, Day::EDGE_TOP),
                    (2, Day::EDGE_RIGHT),
                ];

                // Face 4 TOP => Face 6 BOTTOM
                // Face 4 RIGHT => Face 5 LEFT
                // Face 4 BOTTOM => Face 6 TOP
                // Face 4 LEFT => Face 5 RIGHT
                self.face_connect[3] = [
                    (5, Day::EDGE_BOTTOM),
                    (4, Day::EDGE_LEFT),
                    (5, Day::EDGE_TOP),
                    (4, Day::EDGE_RIGHT),
                ];

                // Face 5 TOP => Face 3 BOTTOM
                // Face 5 RIGHT => Face 4 LEFT
                // Face 5 BOTTOM => Face 1 TOP
                // Face 5 LEFT => Face 4 RIGHT
                self.face_connect[4] = [
                    (2, Day::EDGE_BOTTOM),
                    (3, Day::EDGE_LEFT),
                    (0, Day::EDGE_TOP),
                    (3, Day::EDGE_RIGHT),
                ];

                // Face 6 TOP => Face 4 BOTTOM
                // Face 6 RIGHT => Face 6 LEFT
                // Face 6 BOTTOM => Face 4 TOP
                // Face 6 LEFT => Face 6 RIGHT
                self.face_connect[5] = [
                    (3, Day::EDGE_BOTTOM),
                    (5, Day::EDGE_LEFT),
                    (3, Day::EDGE_TOP),
                    (5, Day::EDGE_RIGHT),
                ];
            } else {
                // wrapped like a cube
                //  12
                //  3
                // 45
                // 6
                // Face 1 TOP => Face 6 LEFT
                // Face 1 RIGHT => Face 2 LEFT
                // Face 1 BOTTOM => Face 3 TOP
                // Face 1 LEFT => Face 4 LEFT
                self.face_connect[0] = [
                    (5, Day::EDGE_LEFT),
                    (1, Day::EDGE_LEFT),
                    (2, Day::EDGE_TOP),
                    (3, Day::EDGE_LEFT),
                ];

                // Face 2 TOP => Face 6 BOTTOM
                // Face 2 RIGHT => Face 5 RIGHT
                // Face 2 BOTTOM => Face 3 RIGHT
                // Face 2 LEFT => Face 1 RIGHT
                self.face_connect[1] = [
                    (5, Day::EDGE_BOTTOM),
                    (4, Day::EDGE_RIGHT),
                    (2, Day::EDGE_RIGHT),
                    (0, Day::EDGE_RIGHT),
                ];

                //  12
                //  3
                // 45
                // 6
                // Face 3 TOP => Face 1 BOTTOM
                // Face 3 RIGHT => Face 2 BOTTOM
                // Face 3 BOTTOM => Face 5 TOP
                // Face 3 LEFT => Face 4 TOP
                self.face_connect[2] = [
                    (0, Day::EDGE_BOTTOM),
                    (1, Day::EDGE_BOTTOM),
                    (4, Day::EDGE_TOP),
                    (3, Day::EDGE_TOP),
                ];

                // Face 4 TOP => Face 3 LEFT
                // Face 4 RIGHT => Face 5 LEFT
                // Face 4 BOTTOM => Face 6 TOP
                // Face 4 LEFT => Face 1 LEFT
                self.face_connect[3] = [
                    (2, Day::EDGE_LEFT),
                    (4, Day::EDGE_LEFT),
                    (5, Day::EDGE_TOP),
                    (0, Day::EDGE_LEFT),
                ];

                //  12
                //  3
                // 45
                // 6
                // Face 5 TOP => Face 3 BOTTOM
                // Face 5 RIGHT => Face 2 RIGHT
                // Face 5 BOTTOM => Face 6 RIGHT
                // Face 5 LEFT => Face 4 RIGHT
                self.face_connect[4] = [
                    (2, Day::EDGE_BOTTOM),
                    (1, Day::EDGE_RIGHT),
                    (5, Day::EDGE_RIGHT),
                    (3, Day::EDGE_RIGHT),
                ];

                // Face 6 TOP => Face 4 BOTTOM
                // Face 6 RIGHT => Face 5 BOTTOM
                // Face 6 BOTTOM => Face 2 TOP
                // Face 6 LEFT => Face 1 TOP
                self.face_connect[5] = [
                    (3, Day::EDGE_BOTTOM),
                    (4, Day::EDGE_BOTTOM),
                    (1, Day::EDGE_TOP),
                    (0, Day::EDGE_TOP),
                ];
            }
        } else if self.face_size == 4 {
            // Test Input (face_size = 4)
            if self.part1 {
                // not wrapped
                //   1
                // 234
                //   56
                // Face 1 TOP => Face 5 BOTTOM
                // Face 1 RIGHT => Face 1 LEFT
                // Face 1 BOTTOM => Face 4 TOP
                // Face 1 LEFT => Face 1 RIGHT
                self.face_connect[0] = [
                    (4, Day::EDGE_BOTTOM),
                    (0, Day::EDGE_LEFT),
                    (3, Day::EDGE_TOP),
                    (0, Day::EDGE_RIGHT),
                ];

                // Face 2 TOP => Face 2 BOTTOM
                // Face 2 RIGHT => Face 3 LEFT
                // Face 2 BOTTOM => Face 2 TOP
                // Face 2 LEFT => Face 4 RIGHT
                self.face_connect[1] = [
                    (1, Day::EDGE_BOTTOM),
                    (2, Day::EDGE_LEFT),
                    (1, Day::EDGE_TOP),
                    (3, Day::EDGE_RIGHT),
                ];

                // Face 3 TOP => Face 3 BOTTOM
                // Face 3 RIGHT => Face 4 LEFT
                // Face 3 BOTTOM => Face 3 TOP
                // Face 3 LEFT => Face 2 RIGHT
                self.face_connect[2] = [
                    (2, Day::EDGE_BOTTOM),
                    (3, Day::EDGE_LEFT),
                    (2, Day::EDGE_TOP),
                    (1, Day::EDGE_RIGHT),
                ];

                // Face 4 TOP => Face 1 BOTTOM
                // Face 4 RIGHT => Face 2 LEFT
                // Face 4 BOTTOM => Face 5 TOP
                // Face 4 LEFT => Face 3 RIGHT
                self.face_connect[3] = [
                    (0, Day::EDGE_BOTTOM),
                    (1, Day::EDGE_LEFT),
                    (4, Day::EDGE_TOP),
                    (2, Day::EDGE_RIGHT),
                ];

                // Face 5 TOP => Face 4 BOTTOM
                // Face 5 RIGHT => Face 6 LEFT
                // Face 5 BOTTOM => Face 1 TOP
                // Face 5 LEFT => Face 6 RIGHT
                self.face_connect[4] = [
                    (3, Day::EDGE_BOTTOM),
                    (5, Day::EDGE_LEFT),
                    (0, Day::EDGE_TOP),
                    (5, Day::EDGE_RIGHT),
                ];

                // Face 6 TOP => Face 6 BOTTOM
                // Face 6 RIGHT => Face 5 LEFT
                // Face 6 BOTTOM => Face 6 TOP
                // Face 6 LEFT => Face 5 RIGHT
                self.face_connect[5] = [
                    (5, Day::EDGE_BOTTOM),
                    (4, Day::EDGE_LEFT),
                    (5, Day::EDGE_TOP),
                    (4, Day::EDGE_RIGHT),
                ];
            } else {
                // wrapped like a cube
                //   1
                // 234
                //   56
                // Face 1 TOP => Face 2 TOP
                // Face 1 RIGHT => Face 6 RIGHT
                // Face 1 BOTTOM => Face 4 TOP
                // Face 1 LEFT => Face 3 TOP
                self.face_connect[0] = [
                    (1, Day::EDGE_TOP),
                    (5, Day::EDGE_RIGHT),
                    (3, Day::EDGE_TOP),
                    (2, Day::EDGE_TOP),
                ];

                // Face 2 TOP => Face 1 TOP
                // Face 2 RIGHT => Face 3 LEFT
                // Face 2 BOTTOM => Face 5 BOTTOM
                // Face 2 LEFT => Face 6 BOTTOM
                self.face_connect[1] = [
                    (0, Day::EDGE_TOP),
                    (2, Day::EDGE_LEFT),
                    (4, Day::EDGE_BOTTOM),
                    (5, Day::EDGE_BOTTOM),
                ];

                // Face 3 TOP => Face 1 LEFT
                // Face 3 RIGHT => Face 4 LEFT
                // Face 3 BOTTOM => Face 5 LEFT
                // Face 3 LEFT => Face 2 RIGHT
                self.face_connect[2] = [
                    (0, Day::EDGE_LEFT),
                    (3, Day::EDGE_LEFT),
                    (4, Day::EDGE_LEFT),
                    (1, Day::EDGE_RIGHT),
                ];

                // Face 4 TOP => Face 1 BOTTOM
                // Face 4 RIGHT => Face 6 TOP
                // Face 4 BOTTOM => Face 5 TOP
                // Face 4 LEFT => Face 3 RIGHT
                self.face_connect[3] = [
                    (0, Day::EDGE_BOTTOM),
                    (5, Day::EDGE_TOP),
                    (4, Day::EDGE_TOP),
                    (2, Day::EDGE_RIGHT),
                ];

                //   1
                // 234
                //   56
                // Face 5 TOP => Face 4 BOTTOM
                // Face 5 RIGHT => Face 6 LEFT
                // Face 5 BOTTOM => Face 2 BOTTOM
                // Face 5 LEFT => Face 3 BOTTOM
                self.face_connect[4] = [
                    (3, Day::EDGE_BOTTOM),
                    (5, Day::EDGE_LEFT),
                    (1, Day::EDGE_BOTTOM),
                    (2, Day::EDGE_BOTTOM),
                ];

                // Face 6 TOP => Face 4 RIGHT
                // Face 6 RIGHT => Face 1 RIGHT
                // Face 6 BOTTOM => Face 2 LEFT
                // Face 6 LEFT => Face 5 RIGHT
                self.face_connect[5] = [
                    (3, Day::EDGE_RIGHT),
                    (0, Day::EDGE_RIGHT),
                    (1, Day::EDGE_LEFT),
                    (4, Day::EDGE_RIGHT),
                ];
            }
        } else {
            panic!("Unknown face size {face_width}");
        }
        for f in 0..6 {
            for d in 0..4 {
                let connect = self.face_connect[f][d];
                let connect_back = self.face_connect[connect.0][connect.1];
                assert_eq!(f, connect_back.0);
                assert_eq!(d, connect_back.1);
            }
        }
        self.monkey_face = 0;
        self.monkey_x = 0;
        self.monkey_y = 0;
        self.monkey_facing = Day::FACE_R;
    }

    fn get_map_xy(&self, face: usize, face_x: usize, face_y: usize) -> (usize, usize) {
        let map_x = self.face_x_mins[face] + face_x;
        let map_y = self.face_y_mins[face] + face_y;
        return (map_x, map_y);
    }

    fn move_monkey(&mut self, steps: u8) {
        let mut x = self.monkey_x;
        let mut y = self.monkey_y;
        let mut face = self.monkey_face;
        let mut facing = self.monkey_facing;
        for _ in 0..steps {
            let mut new_x = x;
            let mut new_y = y;
            let mut new_face = face;
            let mut new_facing = facing;
            let mut from_edge = Day::EDGE_UNKNOWN;
            if facing == Day::FACE_L {
                if x > 0 {
                    new_x = x - 1;
                } else {
                    from_edge = Day::EDGE_LEFT;
                }
            } else if facing == Day::FACE_R {
                if x < self.face_size - 1 {
                    new_x = x + 1;
                } else {
                    from_edge = Day::EDGE_RIGHT;
                }
            } else if facing == Day::FACE_U {
                if y > 0 {
                    new_y = y - 1;
                } else {
                    from_edge = Day::EDGE_TOP;
                }
            } else if facing == Day::FACE_D {
                if y < self.face_size - 1 {
                    new_y = y + 1;
                } else {
                    from_edge = Day::EDGE_BOTTOM;
                }
            } else {
                panic!("Unknown facing {facing}");
            }
            if from_edge != Day::EDGE_UNKNOWN {
                let connect = self.face_connect[face][from_edge];
                new_face = connect.0;
                let new_edge = connect.1;
                if new_edge == Day::EDGE_TOP {
                    new_x = match facing {
                        Day::FACE_R => self.face_size - 1 - new_y,
                        Day::FACE_D => new_x,
                        Day::FACE_L => new_y,
                        Day::FACE_U => self.face_size - 1 - new_x,
                        _ => panic!("Bad facing"),
                    };
                    new_y = 0;
                    new_facing = Day::FACE_D;
                } else if new_edge == Day::EDGE_RIGHT {
                    new_y = match facing {
                        Day::FACE_R => self.face_size - 1 - new_y,
                        Day::FACE_D => new_x,
                        Day::FACE_L => new_y,
                        Day::FACE_U => self.face_size - 1 - new_x,
                        _ => panic!("Bad facing"),
                    };
                    new_x = self.face_size - 1;
                    new_facing = Day::FACE_L;
                } else if new_edge == Day::EDGE_BOTTOM {
                    new_x = match facing {
                        Day::FACE_R => new_y,
                        Day::FACE_D => self.face_size - 1 - new_x,
                        Day::FACE_L => self.face_size - 1 - new_y,
                        Day::FACE_U => new_x,
                        _ => panic!("Bad facing"),
                    };
                    new_y = self.face_size - 1;
                    new_facing = Day::FACE_U;
                } else if new_edge == Day::EDGE_LEFT {
                    new_y = match facing {
                        Day::FACE_R => new_y,
                        Day::FACE_D => self.face_size - 1 - new_x,
                        Day::FACE_L => self.face_size - 1 - new_y,
                        Day::FACE_U => new_x,
                        _ => panic!("Bad facing"),
                    };
                    new_x = 0;
                    new_facing = Day::FACE_R;
                } else {
                    panic!("Unknown edge {new_edge}");
                };
            }
            let map = self.face_data[new_face][new_x + new_y * self.face_size];
            if map == Day::WALL {
                break;
            }
            assert_eq!(map, Day::CLEAR);
            x = new_x;
            y = new_y;
            face = new_face;
            facing = new_facing;
        }
        self.monkey_x = x;
        self.monkey_y = y;
        self.monkey_face = face;
        self.monkey_facing = facing;
    }

    fn simulate(&mut self) -> usize {
        let mut distance: u8 = 0;
        for i in 0..self.movement.len() {
            let m = self.movement[i];
            if m == Day::LEFT || m == Day::RIGHT {
                self.move_monkey(distance);
                distance = 0;
            }
            if m == Day::LEFT {
                self.monkey_facing = match self.monkey_facing {
                    Day::FACE_L => Day::FACE_D,
                    Day::FACE_D => Day::FACE_R,
                    Day::FACE_R => Day::FACE_U,
                    Day::FACE_U => Day::FACE_L,
                    _ => panic!("Unknown facing"),
                }
            } else if m == Day::RIGHT {
                self.monkey_facing = match self.monkey_facing {
                    Day::FACE_L => Day::FACE_U,
                    Day::FACE_U => Day::FACE_R,
                    Day::FACE_R => Day::FACE_D,
                    Day::FACE_D => Day::FACE_L,
                    _ => panic!("Unknown facing"),
                }
            } else {
                distance *= 10;
                distance += m - '0' as u8;
            }
        }
        self.move_monkey(distance);
        let (x, y) = self.get_map_xy(self.monkey_face, self.monkey_x, self.monkey_y);
        // let facing = self.get_map_facing(self.monkey_face, self.monkey_facing);
        return 1000 * (y + 1) + 4 * (x + 1) + self.monkey_facing;
    }

    fn part1(&mut self) -> usize {
        self.monkey_x = 0;
        self.monkey_y = 0;
        self.monkey_face = 0;
        self.monkey_facing = Day::FACE_R;
        return self.simulate();
    }

    fn part2(&mut self) -> usize {
        self.monkey_x = 0;
        self.monkey_y = 0;
        self.monkey_face = 0;
        self.monkey_facing = Day::FACE_R;
        return self.simulate();
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
                "        ...#",
                "        .#..",
                "        #...",
                "        ....",
                "...#.......#",
                "........#...",
                "..#....#....",
                "..........#.",
                "        ...#....",
                "        .....#..",
                "        .#......",
                "        ......#.",
                "",
                "10R5L5R10L4R5L5",
            ],
            6032,
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
                "        ...#",
                "        .#..",
                "        #...",
                "        ....",
                "...#.......#",
                "........#...",
                "..#....#....",
                "..........#.",
                "        ...#....",
                "        .....#..",
                "        .#......",
                "        ......#.",
                "",
                "10R5L5R10L4R5L5",
            ],
            5031,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(false);
        day.parse(&lines);
        assert_eq!(day.part2(), input.1);
    }
}
