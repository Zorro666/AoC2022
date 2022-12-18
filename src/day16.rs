use crate::file_to_vec;

/*

--- Day 16: Proboscidea Volcanium ---

The sensors have led you to the origin of the distress signal:
yet another handheld device, just like the one the Elves gave you.
However, you don't see any Elves around; instead, the device is surrounded by elephants!
They must have gotten lost in these tunnels, and one of the elephants apparently figured out how to turn on the distress signal.

The ground rumbles again, much stronger this time.
What kind of cave is this, exactly?
You scan the cave with your handheld device; it reports mostly igneous rock, some ash, pockets of pressurized gas, magma... this isn't just a cave, it's a volcano!

You need to get the elephants out of here, quickly.
Your device estimates that you have 30 minutes before the volcano erupts, so you don't have time to go back out the way you came in.

You scan the cave for other options and discover a network of pipes and pressure-release valves.
You aren't sure how such a system got into a volcano, but you don't have time to complain; your device produces a report (your puzzle input) of each valve's flow rate if it were opened (in pressure per minute) and the tunnels you could use to move between the valves.

There's even a valve in the room you and the elephants are currently standing in labeled AA.
You estimate it will take you one minute to open a single valve and one minute to follow any tunnel from one valve to another.

What is the most pressure you could release?

For example, suppose you had the following scan output:

Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II

All of the valves begin closed.

You start at valve AA, but it must be damaged or jammed or something: its flow rate is 0, so there's no point in opening it.

However, you could spend one minute moving to valve BB and another minute opening it; doing so would release pressure during the remaining 28 minutes at a flow rate of 13, a total eventual pressure release of 28 * 13 = 364.
Then, you could spend your third minute moving to valve CC and your fourth minute opening it, providing an additional 26 minutes of eventual pressure release at a flow rate of 2, or 52 total pressure released by valve CC.

Making your way through the tunnels like this, you could probably open many or all of the valves by the time 30 minutes have elapsed.

However, you need to release as much pressure as possible, so you'll need to be methodical.

Instead, consider this approach:

== Minute 1 ==
No valves are open.
You move to valve DD.

== Minute 2 ==
No valves are open.
You open valve DD.

== Minute 3 ==
Valve DD is open, releasing 20 pressure.
You move to valve CC.

== Minute 4 ==
Valve DD is open, releasing 20 pressure.
You move to valve BB.

== Minute 5 ==
Valve DD is open, releasing 20 pressure.
You open valve BB.

== Minute 6 ==
Valves BB and DD are open, releasing 33 pressure.
You move to valve AA.

== Minute 7 ==
Valves BB and DD are open, releasing 33 pressure.
You move to valve II.

== Minute 8 ==
Valves BB and DD are open, releasing 33 pressure.
You move to valve JJ.

== Minute 9 ==
Valves BB and DD are open, releasing 33 pressure.
You open valve JJ.

== Minute 10 ==
Valves BB, DD, and JJ are open, releasing 54 pressure.
You move to valve II.

== Minute 11 ==
Valves BB, DD, and JJ are open, releasing 54 pressure.
You move to valve AA.

== Minute 12 ==
Valves BB, DD, and JJ are open, releasing 54 pressure.
You move to valve DD.

== Minute 13 ==
Valves BB, DD, and JJ are open, releasing 54 pressure.
You move to valve EE.

== Minute 14 ==
Valves BB, DD, and JJ are open, releasing 54 pressure.
You move to valve FF.

== Minute 15 ==
Valves BB, DD, and JJ are open, releasing 54 pressure.
You move to valve GG.

== Minute 16 ==
Valves BB, DD, and JJ are open, releasing 54 pressure.
You move to valve HH.

== Minute 17 ==
Valves BB, DD, and JJ are open, releasing 54 pressure.
You open valve HH.

== Minute 18 ==
Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
You move to valve GG.

== Minute 19 ==
Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
You move to valve FF.

== Minute 20 ==
Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
You move to valve EE.

== Minute 21 ==
Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
You open valve EE.

== Minute 22 ==
Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
You move to valve DD.

== Minute 23 ==
Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
You move to valve CC.

== Minute 24 ==
Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
You open valve CC.

== Minute 25 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

== Minute 26 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

== Minute 27 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

== Minute 28 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

== Minute 29 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

== Minute 30 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

This approach lets you release the most pressure possible in 30 minutes with this valve layout, 1651.

Work out the steps to release the most pressure in 30 minutes.

What is the most pressure you can release?

Your puzzle answer was 1991.

--- Part Two ---

You're worried that even with an optimal approach, the pressure released won't be enough.
What if you got one of the elephants to help you?

It would take you 4 minutes to teach an elephant how to open the right valves in the right order, leaving you with only 26 minutes to actually execute your plan.

Would having two of you working together be better, even if it means having less time? (Assume that you teach the elephant before opening any valves yourself, giving you both the same full 26 minutes.)

In the example above, you could teach the elephant to help you as follows:

== Minute 1 ==
No valves are open.
You move to valve II.
The elephant moves to valve DD.

== Minute 2 ==
No valves are open.
You move to valve JJ.
The elephant opens valve DD.

== Minute 3 ==
Valve DD is open, releasing 20 pressure.
You open valve JJ.
The elephant moves to valve EE.

== Minute 4 ==
Valves DD and JJ are open, releasing 41 pressure.
You move to valve II.
The elephant moves to valve FF.

== Minute 5 ==
Valves DD and JJ are open, releasing 41 pressure.
You move to valve AA.
The elephant moves to valve GG.

== Minute 6 ==
Valves DD and JJ are open, releasing 41 pressure.
You move to valve BB.
The elephant moves to valve HH.

== Minute 7 ==
Valves DD and JJ are open, releasing 41 pressure.
You open valve BB.
The elephant opens valve HH.

== Minute 8 ==
Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
You move to valve CC.
The elephant moves to valve GG.

== Minute 9 ==
Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
You open valve CC.
The elephant moves to valve FF.

== Minute 10 ==
Valves BB, CC, DD, HH, and JJ are open, releasing 78 pressure.
The elephant moves to valve EE.

== Minute 11 ==
Valves BB, CC, DD, HH, and JJ are open, releasing 78 pressure.
The elephant opens valve EE.

(At this point, all valves are open.)

== Minute 12 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

...

== Minute 20 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

...

== Minute 26 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
With the elephant helping, after 26 minutes, the best you could do would release a total of 1707 pressure.

With you and an elephant working together for 26 minutes, what is the most pressure you could release?

*/

static INPUT_FILE: &str = "data/day16/input.txt";

pub fn run() {
    println!("Day16: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day16: End");
}

struct Day {
    part1: bool,
    node_names: Vec<String>,
    node_rates: Vec<usize>,
    node_links: Vec<Vec<usize>>,
    valves: Vec<usize>,
    steps: Vec<Vec<usize>>,
    node_count: usize,
    start_node: usize,
    max_rate: usize,
}

impl Day {
    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            node_names: Vec::new(),
            node_rates: Vec::new(),
            node_links: Vec::new(),
            valves: Vec::new(),
            steps: Vec::new(),
            node_count: 0,
            start_node: usize::MAX,
            max_rate: 0,
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.part1(30);
            println!("Day16: Result1 {result1}");
            let expected = 1991;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.part2(26);
            println!("Day16: Result2 {result2}");
            let expected = 2705;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        let mut link_names: Vec<Vec<String>> = Vec::new();
        for line in lines {
            // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            // Valve HH has flow rate=22; tunnel leads to valve GG
            let name = line[6..8].trim();
            let rate: usize = line
                [line.find("=").expect("No =") + 1..line.find("; tunnel").expect("No ; tunnel")]
                .parse()
                .expect("No a number");
            let node_index = self.node_names.len();
            if name == "AA" {
                assert_eq!(self.start_node, usize::MAX);
                self.start_node = node_index;
            }
            self.node_names.push(name.to_string());
            self.node_rates.push(rate);
            let mut search = "valves ";
            if line.rfind(search).is_none() {
                search = "valve ";
            }
            let toks: Vec<&str> = line[line.rfind(search).expect("No {search}") + search.len()..]
                .split(",")
                .collect();
            let mut links: Vec<String> = Vec::new();
            for l in toks {
                links.push(l.to_string());
            }
            link_names.push(links.to_owned());

            if rate > 0 {
                self.valves.push(node_index);
                self.max_rate += rate;
            }
        }
        self.node_count = self.node_names.len();
        assert!(self.start_node < self.node_count);
        assert_eq!(self.node_count, self.node_rates.len());
        assert_eq!(self.node_count, link_names.len());
        for n in 0..self.node_count {
            let mut node_links: Vec<usize> = Vec::new();
            for l in &link_names[n] {
                let mut link_index = self.node_count;
                let link_name = l.trim();
                for i in 0..self.node_count {
                    if self.node_names[i] == link_name {
                        link_index = i;
                        break;
                    }
                }
                assert_ne!(link_index, self.node_count);
                node_links.push(link_index);
            }
            self.node_links.push(node_links);
        }
        assert_eq!(self.node_count, self.node_links.len());
        // sort valves highest to lowest
        for i in 0..self.valves.len() - 1 {
            for j in i + 1..self.valves.len() {
                let valve_i = self.valves[i];
                let valve_j = self.valves[j];
                if self.node_rates[valve_i] < self.node_rates[valve_j] {
                    self.valves[i] = valve_j;
                    self.valves[j] = valve_i;
                }
            }
        }
    }

    fn print_steps(&self, route: &Vec<usize>) {
        let mut from = route[0];
        let mut count_steps = 0;
        for i in 0..route.len() {
            let to = route[i];
            let steps = self.steps[from][to];
            count_steps += steps;
            from = to;
            if count_steps < 10 {
                print!(" ");
            }
            print!(" {count_steps}");
        }
    }

    fn print_route(&self, route: &Vec<usize>) {
        for v in route {
            let to = *v;
            let name = &self.node_names[to];
            print!(" {name}");
        }
    }

    fn shortest_distance(&self, start: usize, end: usize) -> Vec<usize> {
        let mut min_steps = usize::MAX;
        let mut nodes_to_visit: Vec<usize> = Vec::new();
        let mut visited_nodes_to_visit: Vec<Vec<usize>> = Vec::new();
        let mut steps_to_visit: Vec<usize> = Vec::new();
        let mut shortest_route: Vec<usize> = Vec::new();

        nodes_to_visit.push(start);
        steps_to_visit.push(0);
        visited_nodes_to_visit.push(Vec::new());
        while !nodes_to_visit.is_empty() {
            let this_node = nodes_to_visit.pop().unwrap();
            let visited_nodes = visited_nodes_to_visit.pop().unwrap();
            let mut new_visited_nodes = visited_nodes.to_owned();

            let this_steps = steps_to_visit.pop().unwrap();
            // Reached the end
            if this_node == end {
                if this_steps < min_steps {
                    min_steps = this_steps;
                    shortest_route = visited_nodes.to_owned();
                    shortest_route.push(this_node);
                }
                continue;
            }
            // Route is too long
            if this_steps >= min_steps {
                continue;
            }
            new_visited_nodes.push(this_node);
            let new_steps = this_steps + 1;
            for l in &self.node_links[this_node] {
                let link = *l;
                // No back tracking
                if new_visited_nodes.contains(&link) {
                    continue;
                }
                nodes_to_visit.push(link);
                steps_to_visit.push(new_steps);
                visited_nodes_to_visit.push(new_visited_nodes.to_owned());
            }
        }
        return shortest_route;
    }

    fn compute_steps(&mut self) {
        self.steps.resize(self.node_count, Vec::new());
        for s in 0..self.node_count {
            if s != self.start_node && self.node_rates[s] == 0 {
                continue;
            }
            let start = s;
            self.steps[start].resize(self.node_count, 0);
            for v in &self.valves {
                let end = *v;
                if start == end {
                    continue;
                }
                let route = self.shortest_distance(start, end);
                let min_steps = route.len() - 1;
                self.steps[start][end] = min_steps;
                // let start_name = &self.node_names[start];
                // let end_name = &self.node_names[end];
                // let step_count = self.fsteps[start][end];
                // print!("Node {start_name} -> {end_name} Steps {step_count} :");
                // self.print_route(&route);
                // println!("");
            }
        }
    }

    fn part1(&mut self, minutes: usize) -> usize {
        self.compute_steps();
        let mut max_total_pressure = 0;
        let mut best_route: Vec<usize> = Vec::new();
        let mut nodes_to_visit: Vec<usize> = Vec::new();
        let mut visited_nodes_to_visit: Vec<Vec<usize>> = Vec::new();
        let mut pressure_to_visit: Vec<usize> = Vec::new();
        let mut active_pressure_to_visit: Vec<usize> = Vec::new();
        let mut times_to_visit: Vec<usize> = Vec::new();
        let mut me_node_to_visit: Vec<usize> = Vec::new();
        let mut max_rates_to_visit: Vec<usize> = Vec::new();

        let route = vec![self.start_node];
        for v in &self.valves {
            let valve = *v;
            let rate = self.node_rates[valve];
            nodes_to_visit.push(valve);
            visited_nodes_to_visit.push(route.to_owned());

            times_to_visit.push(self.steps[self.start_node][valve] + 1);
            pressure_to_visit.push(0);
            active_pressure_to_visit.push(rate);
            max_rates_to_visit.push(self.max_rate - rate);
        }
        while !nodes_to_visit.is_empty() {
            let this_node = nodes_to_visit.pop().unwrap();

            let this_time = times_to_visit.pop().unwrap();
            let this_pressure = pressure_to_visit.pop().unwrap();
            let this_active_pressure = active_pressure_to_visit.pop().unwrap();
            let this_max_rate = max_rates_to_visit.pop().unwrap();
            let mut new_visited_nodes = visited_nodes_to_visit.pop().unwrap().to_owned();
            new_visited_nodes.push(this_node);

            let remaining_time = minutes - this_time;
            let max_delta_pressure = remaining_time * this_active_pressure;
            let max_pressure = this_pressure + max_delta_pressure;
            let max_possible_pressure = remaining_time * this_max_rate;
            if max_possible_pressure + max_pressure < max_total_pressure {
                continue;
            }
            if max_pressure > max_total_pressure {
                max_total_pressure = max_pressure;
                best_route = new_visited_nodes.to_owned();
                // print!("New Max {max_total_pressure} Time {new_time} Route:");
                // self.print_route(&new_visited_nodes);
                // print!("New Max {max_total_pressure} Time {new_time} Route:");
                // self.print_steps(&new_visited_nodes);
            }

            for v in &self.valves {
                let end = *v;
                if this_node == end {
                    continue;
                }
                if new_visited_nodes.contains(&end) {
                    continue;
                }
                let steps_to_valve = self.steps[this_node][end];
                let delta_time = steps_to_valve + 1;
                let delta_pressure = delta_time * this_active_pressure;
                let new_pressure = this_pressure + delta_pressure;
                let new_time = this_time + delta_time;
                if new_time >= minutes {
                    continue;
                }
                let rate = self.node_rates[end];
                let new_active_pressure = this_active_pressure + rate;
                let new_max_rate = this_max_rate - rate;
                nodes_to_visit.push(end);
                times_to_visit.push(new_time);
                pressure_to_visit.push(new_pressure);
                active_pressure_to_visit.push(new_active_pressure);
                visited_nodes_to_visit.push(new_visited_nodes.to_owned());
                max_rates_to_visit.push(new_max_rate);
                me_node_to_visit.push(end);
            }
        }
        self.print_route(&best_route);
        println!("");
        self.print_steps(&best_route);
        println!("");
        return max_total_pressure;
    }

    fn part2(&mut self, minutes: usize) -> usize {
        self.compute_steps();
        let mut max_total_pressure = 0;
        let mut best_route: Vec<usize> = Vec::new();
        let mut nodes_to_visit: Vec<usize> = Vec::new();
        let mut visited_nodes_to_visit: Vec<Vec<usize>> = Vec::new();
        let mut max_rates_to_visit: Vec<usize> = Vec::new();
        let mut el_node_to_visit: Vec<usize> = Vec::new();
        let mut el_times_to_visit: Vec<usize> = Vec::new();
        let mut el_pressure_to_visit: Vec<usize> = Vec::new();
        let mut el_active_pressure_to_visit: Vec<usize> = Vec::new();
        let mut me_node_to_visit: Vec<usize> = Vec::new();
        let mut me_times_to_visit: Vec<usize> = Vec::new();
        let mut me_pressure_to_visit: Vec<usize> = Vec::new();
        let mut me_active_pressure_to_visit: Vec<usize> = Vec::new();

        let route = vec![self.start_node];
        for v in &self.valves {
            let valve = *v;
            let rate = self.node_rates[valve];
            nodes_to_visit.push(valve);
            visited_nodes_to_visit.push(route.to_owned());
            max_rates_to_visit.push(self.max_rate - rate);

            el_node_to_visit.push(valve);
            el_times_to_visit.push(self.steps[self.start_node][valve] + 1);
            el_pressure_to_visit.push(0);
            el_active_pressure_to_visit.push(rate);

            me_node_to_visit.push(self.start_node);
            me_times_to_visit.push(0);
            me_pressure_to_visit.push(0);
            me_active_pressure_to_visit.push(0);
        }

        while !nodes_to_visit.is_empty() {
            let this_node = nodes_to_visit.pop().unwrap();
            let mut new_visited_nodes = visited_nodes_to_visit.pop().unwrap().to_owned();
            new_visited_nodes.push(this_node);
            let this_max_rate = max_rates_to_visit.pop().unwrap();

            let el_node = el_node_to_visit.pop().unwrap();
            let el_time = el_times_to_visit.pop().unwrap();
            let el_pressure = el_pressure_to_visit.pop().unwrap();
            let el_active_pressure = el_active_pressure_to_visit.pop().unwrap();

            let me_node = me_node_to_visit.pop().unwrap();
            let me_time = me_times_to_visit.pop().unwrap();
            let me_pressure = me_pressure_to_visit.pop().unwrap();
            let me_active_pressure = me_active_pressure_to_visit.pop().unwrap();

            let el_remaining_time;
            if minutes > el_time {
                el_remaining_time = minutes - el_time;
            } else {
                el_remaining_time = 0;
            }
            let el_max_delta_pressure = el_remaining_time * el_active_pressure;
            let el_max_pressure = el_pressure + el_max_delta_pressure;

            let me_remaining_time;
            if minutes > me_time {
                me_remaining_time = minutes - me_time;
            } else {
                me_remaining_time = 0;
            }
            let me_max_delta_pressure = me_remaining_time * me_active_pressure;
            let me_max_pressure = me_pressure + me_max_delta_pressure;

            let max_pressure = el_max_pressure + me_max_pressure;
            if max_pressure > max_total_pressure {
                max_total_pressure = max_pressure;
                best_route = new_visited_nodes.to_owned();
                // println!("New Max {max_total_pressure}");
                // print!("New Max {max_total_pressure} Time {new_time} Route:");
                // self.print_route(&new_visited_nodes);
                // print!("New Max {max_total_pressure} Time {new_time} Route:");
                // self.print_steps(&new_visited_nodes);
            }

            if me_remaining_time <= 1 && el_remaining_time <= 1 {
                continue;
            }
            let remaining_time = el_remaining_time.max(me_remaining_time);
            let max_possible_pressure = remaining_time * this_max_rate + max_pressure;
            if max_possible_pressure < max_total_pressure {
                // println!("Ignoring route Max {max_total_pressure} Possible {max_possible_pressure}");
                continue;
            }

            for v in &self.valves {
                let end = *v;
                if this_node == end {
                    continue;
                }
                if new_visited_nodes.contains(&end) {
                    continue;
                }
                let rate = self.node_rates[end];
                let el_time_to_valve = self.steps[el_node][end] + 1;
                let me_time_to_valve = self.steps[me_node][end] + 1;
                let move_elephant = el_time + el_time_to_valve < me_time + me_time_to_valve;

                let mut el_delta_time = 0;
                let mut el_delta_active_pressure = 0;
                let mut el_new_node = el_node;

                let mut me_delta_time = 0;
                let mut me_delta_active_pressure = 0;
                let mut me_new_node = me_node;
                if move_elephant {
                    el_delta_time = el_time_to_valve;
                    el_delta_active_pressure = rate;
                    el_new_node = end;
                } else {
                    me_delta_time = me_time_to_valve;
                    me_delta_active_pressure = rate;
                    me_new_node = end;
                }
                let el_delta_pressure = el_delta_time * el_active_pressure;
                let el_new_pressure = el_pressure + el_delta_pressure;
                let el_new_time = el_time + el_delta_time;
                if el_new_time >= minutes && move_elephant {
                    assert_eq!(me_delta_time, 0);
                    continue;
                }
                let me_delta_pressure = me_delta_time * me_active_pressure;
                let me_new_pressure = me_pressure + me_delta_pressure;
                let me_new_time = me_time + me_delta_time;
                if me_new_time >= minutes && !move_elephant {
                    assert_eq!(el_delta_time, 0);
                    continue;
                }
                let el_new_active_pressure = el_active_pressure + el_delta_active_pressure;
                let me_new_active_pressure = me_active_pressure + me_delta_active_pressure;

                nodes_to_visit.push(end);

                el_node_to_visit.push(el_new_node);
                el_times_to_visit.push(el_new_time);
                el_pressure_to_visit.push(el_new_pressure);
                el_active_pressure_to_visit.push(el_new_active_pressure);

                me_node_to_visit.push(me_new_node);
                me_times_to_visit.push(me_new_time);
                me_pressure_to_visit.push(me_new_pressure);
                me_active_pressure_to_visit.push(me_new_active_pressure);

                visited_nodes_to_visit.push(new_visited_nodes.to_owned());
                let new_max_rate = this_max_rate - rate;
                max_rates_to_visit.push(new_max_rate);
            }
        }
        self.print_route(&best_route);
        println!("");
        self.print_steps(&best_route);
        println!("");
        return max_total_pressure;
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
                "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
                "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
                "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
                "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
                "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
                "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
                "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
                "Valve HH has flow rate=22; tunnel leads to valve GG",
                "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
                "Valve JJ has flow rate=21; tunnel leads to valve II",
            ],
            1651,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.part1(30), input.1);
    }

    #[test]
    fn part2() {
        let input = (
            vec![
                "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
                "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
                "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
                "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
                "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
                "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
                "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
                "Valve HH has flow rate=22; tunnel leads to valve GG",
                "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
                "Valve JJ has flow rate=21; tunnel leads to valve II",
            ],
            1707,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(false);
        day.parse(&lines);
        assert_eq!(day.part2(26), input.1);
    }
}
