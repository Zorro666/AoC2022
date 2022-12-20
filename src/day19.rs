use std::collections::VecDeque;

use crate::file_to_vec;

/*

--- Day 19: Not Enough Minerals ---

Your scans show that the lava did indeed form obsidian!

The wind has changed direction enough to stop sending lava droplets toward you, so you and the elephants exit the cave.
As you do, you notice a collection of geodes around the pond.
Perhaps you could use the obsidian to create some geode-cracking robots and break them open?

To collect the obsidian from the bottom of the pond, you'll need waterproof obsidian-collecting robots.
Fortunately, there is an abundant amount of clay nearby that you can use to make them waterproof.

In order to harvest the clay, you'll need special-purpose clay-collecting robots.
To make any type of robot, you'll need ore, which is also plentiful but in the opposite direction from the clay.

Collecting ore requires ore-collecting robots with big drills.
Fortunately, you have exactly one ore-collecting robot in your pack that you can use to kickstart the whole operation.

Each robot can collect 1 of its resource type per minute.
It also takes one minute for the robot factory (also conveniently from your pack) to construct any type of robot, although it consumes the necessary resources available when construction begins.

The robot factory has many blueprints (your puzzle input) you can choose from, but once you've configured it with a blueprint, you can't change it.
You'll need to work out which blueprint is best.

For example:

Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian.
(Blueprints have been line-wrapped here for legibility.
    The robot factory's actual assortment of blueprints are provided one blueprint per line.)

The elephants are starting to look hungry, so you shouldn't take too long; you need to figure out which blueprint would maximize the number of opened geodes after 24 minutes by figuring out which robots to build and when to build them.

Using blueprint 1 in the example above, the largest number of geodes you could open in 24 minutes is 9.
One way to achieve that is:

== Minute 1 ==
1 ore-collecting robot collects 1 ore; you now have 1 ore.

== Minute 2 ==
1 ore-collecting robot collects 1 ore; you now have 2 ore.

== Minute 3 ==
Spend 2 ore to start building a clay-collecting robot.
1 ore-collecting robot collects 1 ore; you now have 1 ore.
The new clay-collecting robot is ready; you now have 1 of them.

== Minute 4 ==
1 ore-collecting robot collects 1 ore; you now have 2 ore.
1 clay-collecting robot collects 1 clay; you now have 1 clay.

== Minute 5 ==
Spend 2 ore to start building a clay-collecting robot.
1 ore-collecting robot collects 1 ore; you now have 1 ore.
1 clay-collecting robot collects 1 clay; you now have 2 clay.
The new clay-collecting robot is ready; you now have 2 of them.

== Minute 6 ==
1 ore-collecting robot collects 1 ore; you now have 2 ore.
2 clay-collecting robots collect 2 clay; you now have 4 clay.

== Minute 7 ==
Spend 2 ore to start building a clay-collecting robot.
1 ore-collecting robot collects 1 ore; you now have 1 ore.
2 clay-collecting robots collect 2 clay; you now have 6 clay.
The new clay-collecting robot is ready; you now have 3 of them.

== Minute 8 ==
1 ore-collecting robot collects 1 ore; you now have 2 ore.
3 clay-collecting robots collect 3 clay; you now have 9 clay.

== Minute 9 ==
1 ore-collecting robot collects 1 ore; you now have 3 ore.
3 clay-collecting robots collect 3 clay; you now have 12 clay.

== Minute 10 ==
1 ore-collecting robot collects 1 ore; you now have 4 ore.
3 clay-collecting robots collect 3 clay; you now have 15 clay.

== Minute 11 ==
Spend 3 ore and 14 clay to start building an obsidian-collecting robot.
1 ore-collecting robot collects 1 ore; you now have 2 ore.
3 clay-collecting robots collect 3 clay; you now have 4 clay.
The new obsidian-collecting robot is ready; you now have 1 of them.

== Minute 12 ==
Spend 2 ore to start building a clay-collecting robot.
1 ore-collecting robot collects 1 ore; you now have 1 ore.
3 clay-collecting robots collect 3 clay; you now have 7 clay.
1 obsidian-collecting robot collects 1 obsidian; you now have 1 obsidian.
The new clay-collecting robot is ready; you now have 4 of them.

== Minute 13 ==
1 ore-collecting robot collects 1 ore; you now have 2 ore.
4 clay-collecting robots collect 4 clay; you now have 11 clay.
1 obsidian-collecting robot collects 1 obsidian; you now have 2 obsidian.

== Minute 14 ==
1 ore-collecting robot collects 1 ore; you now have 3 ore.
4 clay-collecting robots collect 4 clay; you now have 15 clay.
1 obsidian-collecting robot collects 1 obsidian; you now have 3 obsidian.

== Minute 15 ==
Spend 3 ore and 14 clay to start building an obsidian-collecting robot.
1 ore-collecting robot collects 1 ore; you now have 1 ore.
4 clay-collecting robots collect 4 clay; you now have 5 clay.
1 obsidian-collecting robot collects 1 obsidian; you now have 4 obsidian.
The new obsidian-collecting robot is ready; you now have 2 of them.

== Minute 16 ==
1 ore-collecting robot collects 1 ore; you now have 2 ore.
4 clay-collecting robots collect 4 clay; you now have 9 clay.
2 obsidian-collecting robots collect 2 obsidian; you now have 6 obsidian.

== Minute 17 ==
1 ore-collecting robot collects 1 ore; you now have 3 ore.
4 clay-collecting robots collect 4 clay; you now have 13 clay.
2 obsidian-collecting robots collect 2 obsidian; you now have 8 obsidian.

== Minute 18 ==
Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
1 ore-collecting robot collects 1 ore; you now have 2 ore.
4 clay-collecting robots collect 4 clay; you now have 17 clay.
2 obsidian-collecting robots collect 2 obsidian; you now have 3 obsidian.
The new geode-cracking robot is ready; you now have 1 of them.

== Minute 19 ==
1 ore-collecting robot collects 1 ore; you now have 3 ore.
4 clay-collecting robots collect 4 clay; you now have 21 clay.
2 obsidian-collecting robots collect 2 obsidian; you now have 5 obsidian.
1 geode-cracking robot cracks 1 geode; you now have 1 open geode.

== Minute 20 ==
1 ore-collecting robot collects 1 ore; you now have 4 ore.
4 clay-collecting robots collect 4 clay; you now have 25 clay.
2 obsidian-collecting robots collect 2 obsidian; you now have 7 obsidian.
1 geode-cracking robot cracks 1 geode; you now have 2 open geodes.

== Minute 21 ==
Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
1 ore-collecting robot collects 1 ore; you now have 3 ore.
4 clay-collecting robots collect 4 clay; you now have 29 clay.
2 obsidian-collecting robots collect 2 obsidian; you now have 2 obsidian.
1 geode-cracking robot cracks 1 geode; you now have 3 open geodes.
The new geode-cracking robot is ready; you now have 2 of them.

== Minute 22 ==
1 ore-collecting robot collects 1 ore; you now have 4 ore.
4 clay-collecting robots collect 4 clay; you now have 33 clay.
2 obsidian-collecting robots collect 2 obsidian; you now have 4 obsidian.
2 geode-cracking robots crack 2 geodes; you now have 5 open geodes.

== Minute 23 ==
1 ore-collecting robot collects 1 ore; you now have 5 ore.
4 clay-collecting robots collect 4 clay; you now have 37 clay.
2 obsidian-collecting robots collect 2 obsidian; you now have 6 obsidian.
2 geode-cracking robots crack 2 geodes; you now have 7 open geodes.

== Minute 24 ==
1 ore-collecting robot collects 1 ore; you now have 6 ore.
4 clay-collecting robots collect 4 clay; you now have 41 clay.
2 obsidian-collecting robots collect 2 obsidian; you now have 8 obsidian.
2 geode-cracking robots crack 2 geodes; you now have 9 open geodes.
However, by using blueprint 2 in the example above, you could do even better: the largest number of geodes you could open in 24 minutes is 12.

Determine the quality level of each blueprint by multiplying that blueprint's ID number with the largest number of geodes that can be opened in 24 minutes using that blueprint.
In this example, the first blueprint has ID 1 and can open 9 geodes, so its quality level is 9.
The second blueprint has ID 2 and can open 12 geodes, so its quality level is 24.
Finally, if you add up the quality levels of all of the blueprints in the list, you get 33.

Determine the quality level of each blueprint using the largest number of geodes it could produce in 24 minutes.

What do you get if you add up the quality level of all of the blueprints in your list?

Your puzzle answer was 1266.

The first half of this puzzle is complete! It provides one gold star: *

--- Part Two ---

While you were choosing the best blueprint, the elephants found some food on their own, so you're not in as much of a hurry; you figure you probably have 32 minutes before the wind changes direction again and you'll need to get out of range of the erupting volcano.

Unfortunately, one of the elephants ate most of your blueprint list! Now, only the first three blueprints in your list are intact.

In 32 minutes, the largest number of geodes blueprint 1 (from the example above) can open is 56. One way to achieve that is:

== Minute 1 ==
1 ore-collecting robot collects 1 ore; you now have 1 ore.

== Minute 2 ==
1 ore-collecting robot collects 1 ore; you now have 2 ore.

== Minute 3 ==
1 ore-collecting robot collects 1 ore; you now have 3 ore.

== Minute 4 ==
1 ore-collecting robot collects 1 ore; you now have 4 ore.

== Minute 5 ==
Spend 4 ore to start building an ore-collecting robot.
1 ore-collecting robot collects 1 ore; you now have 1 ore.
The new ore-collecting robot is ready; you now have 2 of them.

== Minute 6 ==
2 ore-collecting robots collect 2 ore; you now have 3 ore.

== Minute 7 ==
Spend 2 ore to start building a clay-collecting robot.
2 ore-collecting robots collect 2 ore; you now have 3 ore.
The new clay-collecting robot is ready; you now have 1 of them.

== Minute 8 ==
Spend 2 ore to start building a clay-collecting robot.
2 ore-collecting robots collect 2 ore; you now have 3 ore.
1 clay-collecting robot collects 1 clay; you now have 1 clay.
The new clay-collecting robot is ready; you now have 2 of them.

== Minute 9 ==
Spend 2 ore to start building a clay-collecting robot.
2 ore-collecting robots collect 2 ore; you now have 3 ore.
2 clay-collecting robots collect 2 clay; you now have 3 clay.
The new clay-collecting robot is ready; you now have 3 of them.

== Minute 10 ==
Spend 2 ore to start building a clay-collecting robot.
2 ore-collecting robots collect 2 ore; you now have 3 ore.
3 clay-collecting robots collect 3 clay; you now have 6 clay.
The new clay-collecting robot is ready; you now have 4 of them.

== Minute 11 ==
Spend 2 ore to start building a clay-collecting robot.
2 ore-collecting robots collect 2 ore; you now have 3 ore.
4 clay-collecting robots collect 4 clay; you now have 10 clay.
The new clay-collecting robot is ready; you now have 5 of them.

== Minute 12 ==
Spend 2 ore to start building a clay-collecting robot.
2 ore-collecting robots collect 2 ore; you now have 3 ore.
5 clay-collecting robots collect 5 clay; you now have 15 clay.
The new clay-collecting robot is ready; you now have 6 of them.

== Minute 13 ==
Spend 2 ore to start building a clay-collecting robot.
2 ore-collecting robots collect 2 ore; you now have 3 ore.
6 clay-collecting robots collect 6 clay; you now have 21 clay.
The new clay-collecting robot is ready; you now have 7 of them.

== Minute 14 ==
Spend 3 ore and 14 clay to start building an obsidian-collecting robot.
2 ore-collecting robots collect 2 ore; you now have 2 ore.
7 clay-collecting robots collect 7 clay; you now have 14 clay.
The new obsidian-collecting robot is ready; you now have 1 of them.

== Minute 15 ==
2 ore-collecting robots collect 2 ore; you now have 4 ore.
7 clay-collecting robots collect 7 clay; you now have 21 clay.
1 obsidian-collecting robot collects 1 obsidian; you now have 1 obsidian.

== Minute 16 ==
Spend 3 ore and 14 clay to start building an obsidian-collecting robot.
2 ore-collecting robots collect 2 ore; you now have 3 ore.
7 clay-collecting robots collect 7 clay; you now have 14 clay.
1 obsidian-collecting robot collects 1 obsidian; you now have 2 obsidian.
The new obsidian-collecting robot is ready; you now have 2 of them.

== Minute 17 ==
Spend 3 ore and 14 clay to start building an obsidian-collecting robot.
2 ore-collecting robots collect 2 ore; you now have 2 ore.
7 clay-collecting robots collect 7 clay; you now have 7 clay.
2 obsidian-collecting robots collect 2 obsidian; you now have 4 obsidian.
The new obsidian-collecting robot is ready; you now have 3 of them.

== Minute 18 ==
2 ore-collecting robots collect 2 ore; you now have 4 ore.
7 clay-collecting robots collect 7 clay; you now have 14 clay.
3 obsidian-collecting robots collect 3 obsidian; you now have 7 obsidian.

== Minute 19 ==
Spend 3 ore and 14 clay to start building an obsidian-collecting robot.
2 ore-collecting robots collect 2 ore; you now have 3 ore.
7 clay-collecting robots collect 7 clay; you now have 7 clay.
3 obsidian-collecting robots collect 3 obsidian; you now have 10 obsidian.
The new obsidian-collecting robot is ready; you now have 4 of them.

== Minute 20 ==
Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
2 ore-collecting robots collect 2 ore; you now have 3 ore.
7 clay-collecting robots collect 7 clay; you now have 14 clay.
4 obsidian-collecting robots collect 4 obsidian; you now have 7 obsidian.
The new geode-cracking robot is ready; you now have 1 of them.

== Minute 21 ==
Spend 3 ore and 14 clay to start building an obsidian-collecting robot.
2 ore-collecting robots collect 2 ore; you now have 2 ore.
7 clay-collecting robots collect 7 clay; you now have 7 clay.
4 obsidian-collecting robots collect 4 obsidian; you now have 11 obsidian.
1 geode-cracking robot cracks 1 geode; you now have 1 open geode.
The new obsidian-collecting robot is ready; you now have 5 of them.

== Minute 22 ==
Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
2 ore-collecting robots collect 2 ore; you now have 2 ore.
7 clay-collecting robots collect 7 clay; you now have 14 clay.
5 obsidian-collecting robots collect 5 obsidian; you now have 9 obsidian.
1 geode-cracking robot cracks 1 geode; you now have 2 open geodes.
The new geode-cracking robot is ready; you now have 2 of them.

== Minute 23 ==
Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
2 ore-collecting robots collect 2 ore; you now have 2 ore.
7 clay-collecting robots collect 7 clay; you now have 21 clay.
5 obsidian-collecting robots collect 5 obsidian; you now have 7 obsidian.
2 geode-cracking robots crack 2 geodes; you now have 4 open geodes.
The new geode-cracking robot is ready; you now have 3 of them.

== Minute 24 ==
Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
2 ore-collecting robots collect 2 ore; you now have 2 ore.
7 clay-collecting robots collect 7 clay; you now have 28 clay.
5 obsidian-collecting robots collect 5 obsidian; you now have 5 obsidian.
3 geode-cracking robots crack 3 geodes; you now have 7 open geodes.
The new geode-cracking robot is ready; you now have 4 of them.

== Minute 25 ==
2 ore-collecting robots collect 2 ore; you now have 4 ore.
7 clay-collecting robots collect 7 clay; you now have 35 clay.
5 obsidian-collecting robots collect 5 obsidian; you now have 10 obsidian.
4 geode-cracking robots crack 4 geodes; you now have 11 open geodes.

== Minute 26 ==
Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
2 ore-collecting robots collect 2 ore; you now have 4 ore.
7 clay-collecting robots collect 7 clay; you now have 42 clay.
5 obsidian-collecting robots collect 5 obsidian; you now have 8 obsidian.
4 geode-cracking robots crack 4 geodes; you now have 15 open geodes.
The new geode-cracking robot is ready; you now have 5 of them.

== Minute 27 ==
Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
2 ore-collecting robots collect 2 ore; you now have 4 ore.
7 clay-collecting robots collect 7 clay; you now have 49 clay.
5 obsidian-collecting robots collect 5 obsidian; you now have 6 obsidian.
5 geode-cracking robots crack 5 geodes; you now have 20 open geodes.
The new geode-cracking robot is ready; you now have 6 of them.

== Minute 28 ==
2 ore-collecting robots collect 2 ore; you now have 6 ore.
7 clay-collecting robots collect 7 clay; you now have 56 clay.
5 obsidian-collecting robots collect 5 obsidian; you now have 11 obsidian.
6 geode-cracking robots crack 6 geodes; you now have 26 open geodes.

== Minute 29 ==
Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
2 ore-collecting robots collect 2 ore; you now have 6 ore.
7 clay-collecting robots collect 7 clay; you now have 63 clay.
5 obsidian-collecting robots collect 5 obsidian; you now have 9 obsidian.
6 geode-cracking robots crack 6 geodes; you now have 32 open geodes.
The new geode-cracking robot is ready; you now have 7 of them.

== Minute 30 ==
Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
2 ore-collecting robots collect 2 ore; you now have 6 ore.
7 clay-collecting robots collect 7 clay; you now have 70 clay.
5 obsidian-collecting robots collect 5 obsidian; you now have 7 obsidian.
7 geode-cracking robots crack 7 geodes; you now have 39 open geodes.
The new geode-cracking robot is ready; you now have 8 of them.

== Minute 31 ==
Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
2 ore-collecting robots collect 2 ore; you now have 6 ore.
7 clay-collecting robots collect 7 clay; you now have 77 clay.
5 obsidian-collecting robots collect 5 obsidian; you now have 5 obsidian.
8 geode-cracking robots crack 8 geodes; you now have 47 open geodes.
The new geode-cracking robot is ready; you now have 9 of them.

== Minute 32 ==
2 ore-collecting robots collect 2 ore; you now have 8 ore.
7 clay-collecting robots collect 7 clay; you now have 84 clay.
5 obsidian-collecting robots collect 5 obsidian; you now have 10 obsidian.
9 geode-cracking robots crack 9 geodes; you now have 56 open geodes.
However, blueprint 2 from the example above is still better; using it, the largest number of geodes you could open in 32 minutes is 62.

You no longer have enough blueprints to worry about quality levels. Instead, for each of the first three blueprints, determine the largest number of geodes you could open; then, multiply these three values together.

Don't worry about quality levels; instead, just determine the largest number of geodes you could open using each of the first three blueprints. What do you get if you multiply these numbers together?

*/

static INPUT_FILE: &str = "data/day19/input.txt";

pub fn run() {
    println!("Day19: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day19: End");
}

struct Day {
    part1: bool,
    count_bps: usize,
    bp_produces_ore: Vec<u16>,
    bp_produces_clay: Vec<u16>,
    bp_produces_obs: Vec<u16>,
    bp_produces_geo: Vec<u16>,
    bp_costs_ore: Vec<u16>,
    bp_costs_clay: Vec<u16>,
    bp_costs_obs: Vec<u16>,
}

struct State {
    count_ore: u16,
    count_clay: u16,
    count_obs: u16,
    count_geo: u16,
    count_ore_robots: u16,
    count_clay_robots: u16,
    count_obs_robots: u16,
    count_geo_robots: u16,
    time: u16,
}

impl Day {
    const ORE: usize = 0;
    const CLAY: usize = 1;
    const OBSIDIAN: usize = 2;
    const GEODE: usize = 3;
    const COUNT_ROBOTS: usize = 4;

    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            count_bps: 0,
            bp_produces_ore: Vec::new(),
            bp_produces_clay: Vec::new(),
            bp_produces_obs: Vec::new(),
            bp_produces_geo: Vec::new(),
            bp_costs_ore: Vec::new(),
            bp_costs_clay: Vec::new(),
            bp_costs_obs: Vec::new(),
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.part1(24);
            println!("Day19: Result1 {result1}");
            let expected = 1266;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.part2(32, 3);
            println!("Day19: Result2 {result2}");
            let expected = 5800;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn product_type(product: &str) -> usize {
        if product == "ore" {
            return Day::ORE;
        } else if product == "clay" {
            return Day::CLAY;
        } else if product == "obsidian" {
            return Day::OBSIDIAN;
        } else if product == "geode" {
            return Day::GEODE;
        }
        panic!("Unknown product");
    }

    fn parse(&mut self, lines: &Vec<String>) {
        let mut bp_index = 0;
        for line in lines {
            bp_index += 1;
            // 'Blueprint 1: Each ore robot costs 4 ore.  Each clay robot costs 2 ore.  Each obsidian robot costs 3 ore and 14 clay.  Each geode robot costs 2 ore and 7 obsidian.',
            let toks: Vec<&str> = line.trim().split(":").collect();
            assert!(toks[0].starts_with("Blueprint "));
            let blueprint: usize = toks[0][toks[0].find("t ").unwrap() + 2..]
                .parse()
                .expect("Not a number");
            assert_eq!(blueprint, bp_index);
            let robots: Vec<&str> = toks[1].trim().split(". ").collect();
            assert_eq!(robots.len(), Day::COUNT_ROBOTS);
            for r in 0..Day::COUNT_ROBOTS {
                let rb = robots[r].trim().trim_end_matches(".");
                let toks: Vec<&str> = rb.split(" ").collect();
                let product = toks[1].trim();
                let mut ore_output = 0;
                let mut clay_output = 0;
                let mut obs_output = 0;
                let mut geo_output = 0;
                match Day::product_type(product) {
                    Day::ORE => ore_output = 1,
                    Day::CLAY => clay_output = 1,
                    Day::OBSIDIAN => obs_output = 1,
                    Day::GEODE => geo_output = 1,
                    _ => panic!("Unknown product {product}"),
                }
                self.bp_produces_ore.push(ore_output);
                self.bp_produces_clay.push(clay_output);
                self.bp_produces_obs.push(obs_output);
                self.bp_produces_geo.push(geo_output);

                let mut ore_cost: u16 = 0;
                let mut clay_cost: u16 = 0;
                let mut obs_cost: u16 = 0;
                let cost_count: u16 = toks[4].parse().expect("Not a number");
                let cost_type = Day::product_type(toks[5]);
                match cost_type {
                    Day::ORE => ore_cost = cost_count,
                    Day::CLAY => clay_cost = cost_count,
                    Day::OBSIDIAN => obs_cost = cost_count,
                    _ => panic!("Unknown product {product}"),
                }
                if toks.len() == 9 {
                    let cost_count_two: u16 = toks[7].parse().expect("Not a number");
                    let cost_type_two = Day::product_type(toks[8]);
                    assert_ne!(cost_type, cost_type_two);
                    match cost_type_two {
                        Day::ORE => ore_cost = cost_count_two,
                        Day::CLAY => clay_cost = cost_count_two,
                        Day::OBSIDIAN => obs_cost = cost_count_two,
                        _ => panic!("Unknown product {product}"),
                    }
                }
                self.bp_costs_ore.push(ore_cost);
                self.bp_costs_clay.push(clay_cost);
                self.bp_costs_obs.push(obs_cost);
            }
        }
        self.count_bps = bp_index;
    }

    fn max_geodes(&mut self, minutes: u16, bp: usize) -> usize {
        // Can only make 1 robot per turn :
        // Once we have enough robots to make
        // any item per turn then we don't need any more of that robot
        let r_start = bp * Day::COUNT_ROBOTS;
        let mut max_robots = [u16::MIN; Day::COUNT_ROBOTS];
        for r in 0..Day::COUNT_ROBOTS {
            let robot = r_start + r;
            max_robots[Day::ORE] = max_robots[Day::ORE].max(self.bp_costs_ore[robot]);
            max_robots[Day::CLAY] = max_robots[Day::CLAY].max(self.bp_costs_clay[robot]);
            max_robots[Day::OBSIDIAN] = max_robots[Day::OBSIDIAN].max(self.bp_costs_obs[robot]);
        }
        max_robots[Day::GEODE] = u16::MAX;

        let mut max_count_geo: usize = 0;
        let starting_state: State = State {
            count_ore: 0,
            count_clay: 0,
            count_obs: 0,
            count_geo: 0,
            count_ore_robots: 1,
            count_clay_robots: 0,
            count_obs_robots: 0,
            count_geo_robots: 0,
            time: 0,
        };
        let mut possible_states: VecDeque<State> = VecDeque::new();
        possible_states.push_back(starting_state);

        while !possible_states.is_empty() {
            let this_state = possible_states.pop_front().unwrap();
            let count_ore = this_state.count_ore;
            let count_clay = this_state.count_clay;
            let count_obs = this_state.count_obs;
            let count_geo = this_state.count_geo;
            let count_ore_robots = this_state.count_ore_robots;
            let count_clay_robots = this_state.count_clay_robots;
            let count_obs_robots = this_state.count_obs_robots;
            let count_geo_robots = this_state.count_geo_robots;
            let time = this_state.time;
            // Find the resource/robot which will take the longest time to obtain with current robots
            for r in 0..Day::COUNT_ROBOTS {
                // Don't build more than the maximum number
                if r == Day::ORE {
                    if count_ore_robots >= max_robots[Day::ORE] {
                        continue;
                    }
                } else if r == Day::CLAY {
                    if count_clay_robots >= max_robots[Day::CLAY] {
                        continue;
                    }
                } else if r == Day::OBSIDIAN {
                    if count_obs_robots >= max_robots[Day::OBSIDIAN] {
                        continue;
                    }
                }
                let robot = r_start + r;
                let mut ore_time_to_make = 0;
                let mut clay_time_to_make = 0;
                let mut obs_time_to_make = 0;
                if count_ore < self.bp_costs_ore[robot] {
                    if count_ore_robots > 0 {
                        ore_time_to_make =
                            (self.bp_costs_ore[robot] - count_ore + count_ore_robots - 1)
                                / count_ore_robots;
                    } else {
                        ore_time_to_make = minutes;
                    }
                }
                if count_clay < self.bp_costs_clay[robot] {
                    if count_clay_robots > 0 {
                        clay_time_to_make =
                            (self.bp_costs_clay[robot] - count_clay + count_clay_robots - 1)
                                / count_clay_robots;
                    } else {
                        clay_time_to_make = minutes;
                    }
                }
                if count_obs < self.bp_costs_obs[robot] {
                    if count_obs_robots > 0 {
                        obs_time_to_make =
                            (self.bp_costs_obs[robot] - count_obs + count_obs_robots - 1)
                                / count_obs_robots;
                    } else {
                        obs_time_to_make = minutes;
                    }
                }
                let max_time_to_wait_for_robot =
                    ore_time_to_make.max(clay_time_to_make.max(obs_time_to_make));
                let max_time_to_wait_for_resource = max_time_to_wait_for_robot + 1;
                let max_time_make_resource = time + max_time_to_wait_for_resource;
                // Ignore this resource/robot it will take too long to be built
                if max_time_make_resource >= minutes {
                    continue;
                }

                // Get the resources from the robots minus the cost for building this robot
                let new_count_ore = count_ore + max_time_to_wait_for_resource * count_ore_robots
                    - self.bp_costs_ore[robot];
                let new_count_clay = count_clay + max_time_to_wait_for_resource * count_clay_robots
                    - self.bp_costs_clay[robot];
                let new_count_obs = count_obs + max_time_to_wait_for_resource * count_obs_robots
                    - self.bp_costs_obs[robot];
                let new_count_geo = count_geo + max_time_to_wait_for_resource * count_geo_robots;

                // Build the new robot
                let mut new_count_ore_robots = count_ore_robots;
                let mut new_count_clay_robots = count_clay_robots;
                let mut new_count_obs_robots = count_obs_robots;
                let mut new_count_geo_robots = count_geo_robots;
                match r {
                    Day::ORE => new_count_ore_robots += 1,
                    Day::CLAY => new_count_clay_robots += 1,
                    Day::OBSIDIAN => new_count_obs_robots += 1,
                    Day::GEODE => new_count_geo_robots += 1,
                    _ => panic!("Unknown robot {r}"),
                }

                // Ignore this state if its possible max doesn't beat the current max
                // N time left : build a bot every time which builds an ore
                // 1+2+3+4+5 = triangle numbers
                let time_left = minutes - max_time_make_resource;
                let max_theory_geodes: usize = (new_count_geo
                    + new_count_geo_robots * time_left
                    + ((time_left - 1) * time_left) / 2)
                    as usize;
                if max_theory_geodes < max_count_geo {
                    continue;
                }
                // Add the new state
                let new_state: State = State {
                    count_ore: new_count_ore,
                    count_clay: new_count_clay,
                    count_obs: new_count_obs,
                    count_geo: new_count_geo,
                    count_ore_robots: new_count_ore_robots,
                    count_clay_robots: new_count_clay_robots,
                    count_obs_robots: new_count_obs_robots,
                    count_geo_robots: new_count_geo_robots,
                    time: max_time_make_resource,
                };
                possible_states.push_back(new_state);
            }

            let count_geodes: usize = (count_geo + count_geo_robots * (minutes - time)) as usize;
            max_count_geo = max_count_geo.max(count_geodes);
        }
        return max_count_geo;
    }

    fn part1(&mut self, minutes: u16) -> usize {
        let mut result = 0;
        for bp in 0..self.count_bps {
            let max_count_geo = self.max_geodes(minutes, bp);
            result += (bp + 1) * max_count_geo;
        }
        return result;
    }

    fn part2(&mut self, minutes: u16, max_bps: usize) -> usize {
        let mut result = 1;
        for bp in 0..max_bps {
            let max_count_geo = self.max_geodes(minutes, bp);
            result *= max_count_geo;
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::str_array_to_string_array;

    #[test]
    fn part1() {
        let input = (vec![
"Blueprint 1: Each ore robot costs 4 ore.  Each clay robot costs 2 ore.  Each obsidian robot costs 3 ore and 14 clay.  Each geode robot costs 2 ore and 7 obsidian.",
"Blueprint 2: Each ore robot costs 2 ore.  Each clay robot costs 3 ore.  Each obsidian robot costs 3 ore and 8 clay.  Each geode robot costs 3 ore and 12 obsidian.",],
            33);
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.part1(24), input.1);
    }

    #[test]
    fn part2() {
        let input = (vec![
"Blueprint 1: Each ore robot costs 4 ore.  Each clay robot costs 2 ore.  Each obsidian robot costs 3 ore and 14 clay.  Each geode robot costs 2 ore and 7 obsidian.",
"Blueprint 2: Each ore robot costs 2 ore.  Each clay robot costs 3 ore.  Each obsidian robot costs 3 ore and 8 clay.  Each geode robot costs 3 ore and 12 obsidian.",],
            56 * 62);
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(false);
        day.parse(&lines);
        assert_eq!(day.part2(32, 2), input.1);
    }
}
