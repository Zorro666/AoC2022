use crate::file_to_vec;

/*

--- Day 18: Boiling Boulders ---

You and the elephants finally reach fresh air.
You've emerged near the base of a large volcano that seems to be actively erupting! Fortunately, the lava seems to be flowing away from you and toward the ocean.

Bits of lava are still being ejected toward you, so you're sheltering in the cavern exit a little longer.
Outside the cave, you can see the lava landing in a pond and hear it loudly hissing as it solidifies.

Depending on the specific compounds in the lava and speed at which it cools, it might be forming obsidian!
The cooling rate should be based on the surface area of the lava droplets, so you take a quick scan of a droplet as it flies past you (your puzzle input).

Because of how quickly the lava is moving, the scan isn't very good; its resolution is quite low and, as a result, it approximates the shape of the lava droplet with 1x1x1 cubes on a 3D grid, each given as its x,y,z position.

To approximate the surface area, count the number of sides of each cube that are not immediately connected to another cube.
So, if your scan were only two adjacent cubes like 1,1,1 and 2,1,1, each cube would have a single side covered and five sides exposed, a total surface area of 10 sides.

Here's a larger example:

2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5

In the above example, after counting up all the sides that aren't connected to another cube, the total surface area is 64.

What is the surface area of your scanned lava droplet?

Your puzzle answer was 3448.

--- Part Two ---

Something seems off about your calculation.
The cooling rate depends on exterior surface area, but your calculation also included the surface area of air pockets trapped in the lava droplet.

Instead, consider only cube sides that could be reached by the water and steam as the lava droplet tumbles into the pond.
The steam will expand to reach as much as possible, completely displacing any air on the outside of the lava droplet but never expanding diagonally.

In the larger example above, exactly one cube of air is trapped within the lava droplet (at 2,2,5), so the exterior surface area of the lava droplet is 58.

What is the exterior surface area of your scanned lava droplet?

*/

static INPUT_FILE: &str = "data/day18/input.txt";

pub fn run() {
    println!("Day18: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day18: End");
}
struct Day {
    part1: bool,
    grid_data: Vec<u8>,
    grid_size: usize,
    grid_y_index: usize,
    grid_z_index: usize,
    cube_xs: Vec<usize>,
    cube_ys: Vec<usize>,
    cube_zs: Vec<usize>,
    count_cubes: usize,
}

impl Day {
    const AIR: u8 = 0;
    const LAVA: u8 = 1;
    const WATER: u8 = 2;
    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            grid_data: Vec::new(),
            grid_size: 0,
            grid_y_index: 0,
            grid_z_index: 0,
            cube_xs: Vec::new(),
            cube_ys: Vec::new(),
            cube_zs: Vec::new(),
            count_cubes: 0,
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.part1();
            println!("Day18: Result1 {result1}");
            let expected = 3448;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.part2();
            println!("Day18: Result2 {result2}");
            let expected = 2052;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        let mut max_grid: usize = usize::MIN;
        for line in lines {
            // 2,2,2
            let toks: Vec<&str> = line.trim().split(",").collect();
            let x: usize = toks[0].trim().parse().expect("Not a number");
            let y: usize = toks[1].trim().parse().expect("Not a number");
            let z: usize = toks[2].trim().parse().expect("Not a number");
            assert!(x < 100);
            assert!(y < 100);
            assert!(z < 100);
            max_grid = max_grid.max(x);
            max_grid = max_grid.max(y);
            max_grid = max_grid.max(z);
            self.cube_xs.push(x);
            self.cube_ys.push(y);
            self.cube_zs.push(z);
        }
        max_grid += 1;
        assert!(max_grid < 100);
        self.grid_size = max_grid;
        self.grid_y_index = self.grid_size;
        self.grid_z_index = self.grid_size * self.grid_size;
        self.grid_data
            .resize(max_grid * max_grid * max_grid as usize, Day::AIR);
        self.count_cubes = self.cube_xs.len();
        for i in 0..self.count_cubes {
            let x = self.cube_xs[i];
            let y = self.cube_ys[i];
            let z = self.cube_zs[i];
            let grid_index = self.grid_index(x, y, z);
            self.grid_data[grid_index] = Day::LAVA;
        }
    }

    fn grid_index(&self, x: usize, y: usize, z: usize) -> usize {
        return x + y * self.grid_y_index + z * self.grid_z_index;
    }

    fn count_surface_area(&self) -> usize {
        let count = self.count_cubes;
        let mut count_hidden = 0;
        // Total = 6 * cube_count - hidden_faces
        // Make faces : don't add if already exists
        // Define a face :
        // XYZ : -X
        // Four points or XYZ + DX,DY,DZ
        for i in 0..count {
            // Cube @ x,y,z
            let x = self.cube_xs[i];
            let y = self.cube_ys[i];
            let z = self.cube_zs[i];
            // -X YZ face : X-1, Y,   Z then face is hidden
            if x > 0 {
                let grid_index = self.grid_index(x - 1, y, z);
                if self.grid_data[grid_index] == Day::LAVA {
                    count_hidden += 1;
                }
            }
            // +X YZ face : X+1, Y,   Z then face is hidden
            if x < self.grid_size - 1 {
                let grid_index = self.grid_index(x + 1, y, z);
                if self.grid_data[grid_index] == Day::LAVA {
                    count_hidden += 1;
                }
            }

            // XZ -Y face : X, Y-1,   Z then face is hidden
            if y > 0 {
                let grid_index = self.grid_index(x, y - 1, z);
                if self.grid_data[grid_index] == Day::LAVA {
                    count_hidden += 1;
                }
            }
            // XZ +Y face : X, Y+1,   Z then face is hidden
            if y < self.grid_size - 1 {
                let grid_index = self.grid_index(x, y + 1, z);
                if self.grid_data[grid_index] == Day::LAVA {
                    count_hidden += 1;
                }
            }

            // XY -Z face : X,   Y, Z-1 then face is hidden
            if z > 0 {
                let grid_index = self.grid_index(x, y, z - 1);
                if self.grid_data[grid_index] == Day::LAVA {
                    count_hidden += 1;
                }
            }
            // XY +Z : face : X,   Y, Z+1 then face is hidden
            if z < self.grid_size - 1 {
                let grid_index = self.grid_index(x, y, z + 1);
                if self.grid_data[grid_index] == Day::LAVA {
                    count_hidden += 1;
                }
            }
        }
        assert_eq!(count_hidden % 2, 0);
        let total = 6 * count - count_hidden;
        return total;
    }

    fn part1(&self) -> usize {
        return self.count_surface_area();
    }

    fn part2(&mut self) -> usize {
        // Flood fill from any empty cube with water : start from 0,0,0
        let start_index = self.grid_index(0, 0, 0);
        assert_eq!(self.grid_data[start_index], Day::AIR);

        let mut cells_to_visit: Vec<usize> = Vec::new();
        cells_to_visit.push(start_index);

        while !cells_to_visit.is_empty() {
            let this_index = cells_to_visit.pop().unwrap();
            let this_cell = self.grid_data[this_index];
            if this_cell != Day::AIR {
                continue;
            }
            self.grid_data[this_index] = Day::WATER;
            let z = this_index / self.grid_z_index;
            let y = (this_index - z * self.grid_z_index) / self.grid_y_index;
            let x = this_index - z * self.grid_z_index - y * self.grid_y_index;
            assert_eq!(this_index, self.grid_index(x, y, z));
            // -X
            if x > 0 {
                let grid_index = self.grid_index(x - 1, y, z);
                if self.grid_data[grid_index] == Day::AIR {
                    cells_to_visit.push(grid_index);
                }
            }
            // +X
            if x < self.grid_size - 1 {
                let grid_index = self.grid_index(x + 1, y, z);
                if self.grid_data[grid_index] == Day::AIR {
                    cells_to_visit.push(grid_index);
                }
            }
            // -Y
            if y > 0 {
                let grid_index = self.grid_index(x, y - 1, z);
                if self.grid_data[grid_index] == Day::AIR {
                    cells_to_visit.push(grid_index);
                }
            }
            // +Y
            if y < self.grid_size - 1 {
                let grid_index = self.grid_index(x, y + 1, z);
                if self.grid_data[grid_index] == Day::AIR {
                    cells_to_visit.push(grid_index);
                }
            }
            // -Z
            if z > 0 {
                let grid_index = self.grid_index(x, y, z - 1);
                if self.grid_data[grid_index] == Day::AIR {
                    cells_to_visit.push(grid_index);
                }
            }
            // +Z
            if z < self.grid_size - 1 {
                let grid_index = self.grid_index(x, y, z + 1);
                if self.grid_data[grid_index] == Day::AIR {
                    cells_to_visit.push(grid_index);
                }
            }
        }

        // Convert all AIR cubes to LAVA cubes
        let count = self.count_cubes;
        for i in 0..self.grid_data.len() {
            // Cube @ x,y,z
            if self.grid_data[i] == Day::AIR {
                self.grid_data[i] = Day::LAVA;
            }
        }
        return self.count_surface_area();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::str_array_to_string_array;

    #[test]
    fn part1() {
        let inputs = vec![
            (vec!["1,1,1", "2,1,1"], 10),
            (
                vec![
                    "2,2,2", "1,2,2", "3,2,2", "2,1,2", "2,3,2", "2,2,1", "2,2,3", "2,2,4",
                    "2,2,6", "1,2,5", "3,2,5", "2,1,5", "2,3,5",
                ],
                64,
            ),
        ];
        for input in inputs {
            let lines = str_array_to_string_array(input.0);
            let mut day = Day::instance(true);
            day.parse(&lines);
            assert_eq!(day.part1(), input.1);
        }
    }

    #[test]
    fn part2() {
        let input = (
            vec![
                "2,2,2", "1,2,2", "3,2,2", "2,1,2", "2,3,2", "2,2,1", "2,2,3", "2,2,4", "2,2,6",
                "1,2,5", "3,2,5", "2,1,5", "2,3,5",
            ],
            58,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(false);
        day.parse(&lines);
        assert_eq!(day.part2(), input.1);
    }
}
