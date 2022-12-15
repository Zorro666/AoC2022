use crate::file_to_vec;

/*

--- Day 15: Beacon Exclusion Zone ---

You feel the ground rumble again as the distress signal leads you to a large network of subterranean tunnels.
You don't have time to search them all, but you don't need to: your pack contains a set of deployable sensors that you imagine were originally built to locate lost Elves.

The sensors aren't very powerful, but that's okay; your handheld device indicates that you're close enough to the source of the distress signal to use them.
You pull the emergency sensor system out of your pack, hit the big button on top, and the sensors zoom off down the tunnels.

Once a sensor finds a spot it thinks will give it a good reading, it attaches itself to a hard surface and begins monitoring for the nearest signal source beacon.
Sensors and beacons always exist at integer coordinates.
Each sensor knows its own position and can determine the position of a beacon precisely;
however, sensors can only lock on to the one beacon closest to the sensor as measured by the Manhattan distance.
(There is never a tie where two beacons are the same distance to a sensor.)

It doesn't take long for the sensors to report back their positions and closest beacons (your puzzle input).
For example:

Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3

So, consider the sensor at 2,18; the closest beacon to it is at -2,15.
For the sensor at 9,16, the closest beacon to it is at 10,16.

Drawing sensors as S and beacons as B, the above arrangement of sensors and beacons looks like this:

               1    1    2    2
     0    5    0    5    0    5
 0 ....S.......................
 1 ......................S.....
 2 ...............S............
 3 ................SB..........
 4 ............................
 5 ............................
 6 ............................
 7 ..........S.......S.........
 8 ............................
 9 ............................
10 ....B.......................
11 ..S.........................
12 ............................
13 ............................
14 ..............S.......S.....
15 B...........................
16 ...........SB...............
17 ................S..........B
18 ....S.......................
19 ............................
20 ............S......S........
21 ............................
22 .......................B....

This isn't necessarily a comprehensive map of all beacons in the area, though.
Because each sensor only identifies its closest beacon, if a sensor detects a beacon, you know there are no other beacons that close or closer to that sensor.
There could still be beacons that just happen to not be the closest beacon to any sensor.
Consider the sensor at 8,7:

               1    1    2    2
     0    5    0    5    0    5
-2 ..........#.................
-1 .........###................
 0 ....S...#####...............
 1 .......#######........S.....
 2 ......#########S............
 3 .....###########SB..........
 4 ....#############...........
 5 ...###############..........
 6 ..#################.........
 7 .#########S#######S#........
 8 ..#################.........
 9 ...###############..........
10 ....B############...........
11 ..S..###########............
12 ......#########.............
13 .......#######..............
14 ........#####.S.......S.....
15 B........###................
16 ..........#SB...............
17 ................S..........B
18 ....S.......................
19 ............................
20 ............S......S........
21 ............................
22 .......................B....

This sensor's closest beacon is at 2,10, and so you know there are no beacons that close or closer (in any positions marked #).

None of the detected beacons seem to be producing the distress signal, so you'll need to work out where the distress beacon is by working out where it isn't.
For now, keep things simple by counting the positions where a beacon cannot possibly be along just a single row.

So, suppose you have an arrangement of beacons and sensors like in the example above and, just in the row where y=10, you'd like to count the number of positions a beacon cannot possibly exist.
The coverage from all sensors near that row looks like this:

                 1    1    2    2
       0    5    0    5    0    5
 9 ...#########################...
10 ..####B######################..
11 .###S#############.###########.

In this example, in the row where y=10, there are 26 positions where a beacon cannot be present.

Consult the report from the sensors you just deployed.

In the row where y=2000000, how many positions cannot contain a beacon?

Your puzzle answer was 4883971.

--- Part Two ---

Your handheld device indicates that the distress signal is coming from a beacon nearby.
The distress beacon is not detected by any sensor, but the distress beacon must have x and y coordinates each no lower than 0 and no larger than 4000000.

To isolate the distress beacon's signal, you need to determine its tuning frequency, which can be found by multiplying its x coordinate by 4000000 and then adding its y coordinate.

In the example above, the search space is smaller: instead, the x and y coordinates can each be at most 20.
With this reduced search area, there is only a single position that could have a beacon: x=14, y=11.
The tuning frequency for this distress beacon is 56000011.

Find the only possible position for the distress beacon. What is its tuning frequency?

*/

static INPUT_FILE: &str = "data/day15/input.txt";

pub fn run() {
    println!("Day15: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day15: End");
}

struct Day {
    part1: bool,
    sensors: Vec<(i32, i32)>,
    unique_beacons: Vec<(i32, i32)>,
    min_distances: Vec<u32>,
    grid_min: (i32, i32),
    grid_max: (i32, i32),
}

impl Day {
    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            sensors: Vec::new(),
            unique_beacons: Vec::new(),
            min_distances: Vec::new(),
            grid_min: (i32::MAX, i32::MAX),
            grid_max: (i32::MIN, i32::MIN),
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.part1(2000000);
            println!("Day15: Result1 {result1}");
            let expected = 4883971;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.part2(4000000);
            println!("Day15: Result2 {result2}");
            let expected = 12691026767556;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        for line in lines {
            // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
            let toks: Vec<&str> = line[line.find("x=").expect("Bad parse") + 2..]
                .split(",")
                .collect();
            let sx: i32 = toks[0].parse().expect("Not a number");
            let toks: Vec<&str> = line[line.find("y=").expect("Bad parse") + 2..]
                .split(":")
                .collect();
            let sy: i32 = toks[0].parse().expect("Not a number");
            self.sensors.push((sx, sy));

            // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
            let toks: Vec<&str> = line[line.rfind("x=").expect("Bad parse") + 2..]
                .split(",")
                .collect();
            let bx: i32 = toks[0].parse().expect("Not a number");
            let toks: Vec<&str> = line[line.rfind("y=").expect("Bad parse") + 2..]
                .split(",")
                .collect();
            let by: i32 = toks[0].parse().expect("Not a number");
            let distance = bx.abs_diff(sx) + by.abs_diff(sy);
            self.min_distances.push(distance);
            let sensor_min_x = sx - distance as i32;
            let sensor_max_x = sx + distance as i32;
            self.grid_min.0 = self.grid_min.0.min(sx - sensor_min_x);
            self.grid_min.1 = self.grid_min.1.min(sy);

            self.grid_min.0 = self.grid_min.0.min(bx);
            self.grid_min.1 = self.grid_min.1.min(by);

            self.grid_max.0 = self.grid_max.0.max(sx + sensor_max_x);
            self.grid_max.1 = self.grid_max.1.max(sy);

            self.grid_max.0 = self.grid_max.0.max(bx);
            self.grid_max.1 = self.grid_max.1.max(by);

            // Unique beacons
            let mut found = false;
            for b in 0..self.unique_beacons.len() {
                if self.unique_beacons[b].0 == bx && self.unique_beacons[b].1 == by {
                    found = true;
                    break;
                }
            }
            if !found {
                self.unique_beacons.push((bx, by));
            }
        }
    }

    fn part1(&self, row: i32) -> usize {
        let mut total = 0;
        let y = row;
        let sensor_count = self.sensors.len();
        let mut ranges: Vec<(i32, i32)> = Vec::new();
        let mut beacon_count = 0;

        for b in 0..self.unique_beacons.len() {
            if self.unique_beacons[b].1 == y {
                beacon_count += 1;
            }
        }

        // Compute possible ranges of overlap
        let mut range_extent = (i32::MAX, i32::MIN);
        for s in 0..sensor_count {
            let sx = self.sensors[s].0;
            let sy = self.sensors[s].1;
            let min_distance = self.min_distances[s];
            let ydist = y.abs_diff(sy);
            if ydist >= min_distance {
                continue;
            }
            let max_xrange = (min_distance - ydist) as i32;
            let min_x = sx - max_xrange;
            let max_x = sx + max_xrange;
            let mut new_range = true;
            for r in 0..ranges.len() {
                let min_r = ranges[r].0;
                let max_r = ranges[r].1;
                // min_r-----------------max_r
                //      min_x------max_x
                if min_r <= min_x && min_x <= max_r && max_x <= max_r {
                    new_range = false;
                    break;
                }
                //      min_r------max_r
                // min_x-----------------max_x
                else if min_x <= min_r && min_r <= max_x && max_r <= max_x {
                    ranges[r].0 = min_x;
                    ranges[r].1 = max_x;
                    new_range = false;
                    range_extent.0 = range_extent.0.min(min_x);
                    range_extent.1 = range_extent.1.max(max_x);
                    break;
                }
                // min_r------max_r
                //    min_x------max_x
                else if min_r <= min_x && min_x <= max_r && max_x > max_r {
                    ranges[r].1 = max_x;
                    new_range = false;
                    range_extent.1 = range_extent.1.max(max_x);
                    break;
                }
                // min_x------max_x
                //      min_r------max_r
                else if min_x < min_r && min_r <= max_x && max_r >= max_x {
                    ranges[r].0 = min_x;
                    new_range = false;
                    range_extent.0 = range_extent.0.min(min_x);
                    break;
                }
            }
            if new_range {
                ranges.push((min_x, max_x));
                range_extent.0 = range_extent.0.min(min_x);
                range_extent.1 = range_extent.1.max(max_x);
            }
            // TODO: merge ranges
        }
        for x in range_extent.0..=range_extent.1 {
            let mut detected = false;
            for r in 0..ranges.len() {
                let min_x = ranges[r].0;
                let max_x = ranges[r].1;
                if x >= min_x && x <= max_x {
                    detected = true;
                    break;
                }
            }
            if detected {
                total += 1;
            }
        }
        return total - beacon_count;
    }

    fn part2(&self, max_row: i32) -> i64 {
        let sensor_count = self.sensors.len();
        for y in 0..max_row {
            let mut ranges: Vec<(i32, i32)> = Vec::new();

            // Compute possible ranges of overlap
            for s in 0..sensor_count {
                let sx = self.sensors[s].0;
                let sy = self.sensors[s].1;
                let min_distance = self.min_distances[s];
                let ydist = y.abs_diff(sy);
                if ydist >= min_distance {
                    continue;
                }
                let max_xrange = (min_distance - ydist) as i32;
                let min_x = (sx - max_xrange).max(0);
                let max_x = (sx + max_xrange).min(max_row);
                let mut new_range = true;
                for r in 0..ranges.len() {
                    let min_r = ranges[r].0;
                    let max_r = ranges[r].1;
                    //      min_r------max_r
                    // min_x-----------------max_x
                    if min_x <= min_r && min_r <= max_x && max_r <= max_x {
                        ranges[r].0 = min_x;
                        ranges[r].1 = max_x;
                        new_range = false;
                        break;
                    }
                    // min_r-----------------max_r
                    //      min_x------max_x
                    else if min_r <= min_x && min_x <= max_r && max_x <= max_r {
                        new_range = false;
                        break;
                    }
                    // min_r------max_r
                    //    min_x------max_x
                    else if min_r <= min_x && min_x <= max_r && max_x > max_r {
                        ranges[r].1 = max_x;
                        new_range = false;
                        break;
                    }
                    // min_x------max_x
                    //      min_r------max_r
                    else if min_x < min_r && min_r <= max_x && max_r >= max_x {
                        ranges[r].0 = min_x;
                        new_range = false;
                        break;
                    }
                }
                if new_range {
                    ranges.push((min_x, max_x));
                }
            }
            for i in 0..ranges.len() - 1 {
                for j in i + 1..ranges.len() {
                    let min_i = ranges[i].0;
                    let max_i = ranges[i].1;
                    let min_j = ranges[j].0;
                    let max_j = ranges[j].1;
                    // Primary sort minimum of min
                    if min_i < min_j {
                        continue;
                    }
                    // Secondary sort maximum of max
                    if min_i == min_j {
                        if max_j > max_i {
                            ranges[i].1 = max_j;
                            ranges[j].1 = max_i;
                        }
                        continue;
                    }
                    ranges[i].0 = min_j;
                    ranges[j].0 = min_i;
                    ranges[i].1 = max_j;
                    ranges[j].1 = max_i;
                }
            }

            let mut detected = false;
            for r in 0..ranges.len() {
                let min_x = ranges[r].0;
                let max_x = ranges[r].1;
                if min_x == 0 && max_x == max_row {
                    detected = true;
                    break;
                }
            }
            if !detected {
                // try to merge ranges into each other
                for r in 0..ranges.len() {
                    if ranges[r].0 == 0 && ranges[r].1 == 0 {
                        continue;
                    }

                    let mut new_range = true;
                    for rj in 0..ranges.len() {
                        if rj == r {
                            continue;
                        }
                        let min_r = ranges[r].0;
                        let max_r = ranges[r].1;
                        let min_x = ranges[rj].0;
                        let max_x = ranges[rj].1;
                        if min_x == 0 && max_x == 0 {
                            continue;
                        }
                        //      min_r------max_r
                        // min_x-----------------max_x
                        if min_x <= min_r && min_r <= max_x && max_r <= max_x {
                            ranges[r].0 = min_x;
                            ranges[r].1 = max_x;
                            new_range = false;
                        }
                        // min_r-----------------max_r
                        //      min_x------max_x
                        else if min_r <= min_x && min_x <= max_r && max_x <= max_r {
                            new_range = false;
                        }
                        // min_r------max_r
                        //    min_x------max_x
                        else if min_r <= min_x && min_x <= max_r && max_x > max_r {
                            ranges[r].1 = max_x;
                            new_range = false;
                        }
                        // min_x------max_x
                        //      min_r------max_r
                        else if min_x < min_r && min_r <= max_x && max_r >= max_x {
                            ranges[r].0 = min_x;
                            new_range = false;
                        }
                        if !new_range {
                            ranges[rj].0 = 0;
                            ranges[rj].1 = 0;
                        }
                    }
                }
                for r in 0..ranges.len() {
                    let min_x = ranges[r].0;
                    let max_x = ranges[r].1;
                    if min_x == 0 && max_x == 0 {
                        continue;
                    }
                    if min_x == 0 && max_x == max_row {
                        detected = true;
                        break;
                    }
                }

                if !detected {
                    let mut x_min = max_row;
                    let mut x_max = max_row;
                    for r in 0..ranges.len() {
                        let min_x = ranges[r].0;
                        let max_x = ranges[r].1;
                        if min_x == 0 && max_x == 0 {
                            continue;
                        }
                        x_min = x_min.min(min_x);
                        x_max = x_max.min(max_x);
                    }
                    // Check: 0 - min of min's
                    for x in 0..x_min {
                        detected = false;
                        for r in 0..ranges.len() {
                            let min_x = ranges[r].0;
                            let max_x = ranges[r].1;
                            if min_x == 0 && max_x == 0 {
                                continue;
                            }
                            if min_x <= x && x <= max_x {
                                detected = true;
                                break;
                            }
                        }
                        if !detected {
                            return x as i64 * 4000000 + y as i64;
                        }
                    }
                    // Check: min of max's - max_row
                    for x in x_max..=max_row {
                        detected = false;
                        for r in 0..ranges.len() {
                            let min_x = ranges[r].0;
                            let max_x = ranges[r].1;
                            if min_x == 0 && max_x == 0 {
                                continue;
                            }
                            if min_x <= x && x <= max_x {
                                detected = true;
                                break;
                            }
                        }
                        if !detected {
                            return x as i64 * 4000000 + y as i64;
                        }
                    }
                }
            }
        }
        panic!("Did not find it");
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
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
                "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
                "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
                "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
                "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
                "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
                "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
                "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
                "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
                "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
                "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
                "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
                "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
                "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
            ],
            26,
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
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
                "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
                "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
                "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
                "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
                "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
                "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
                "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
                "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
                "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
                "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
                "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
                "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
                "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
            ],
            56000011,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(false);
        day.parse(&lines);
        assert_eq!(day.part2(20), input.1);
    }
}
