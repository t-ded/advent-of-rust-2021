use std::cmp::max;
use std::collections::HashMap;
use crate::array_2d::Coordinate;

pub fn part_1(input: &str) -> isize {
    // One time tick after reaching 0 in y, we get to position (-v_y - 1), since it's reverse of how we went up in the start
        // Since we want to maximize y_max = (v_y^2 + v_y) / 2, which is increasing, we simply find highest v_y s.t. (-v_y - 1) >= bottom, i.e., v_y = (-bottom - 1)
    // v_x is then one that gets "stuck" on any x level within
    let area = Area2D::from_input(input);
    let best_v_y = -area.lower_right.y() - 1;
    (best_v_y.pow(2) + best_v_y) / 2
}

pub fn part_2(input: &str) -> isize {
    let area = Area2D::from_input(input);

    // Must not overshoot in one step
    let max_vx = area.lower_right.x();

    // Must not stall the x coordinate of the probe before reaching the left edge of the target
    let min_vx = ((-1.0 + f64::sqrt(1.0 + 8.0 * area.upper_left.x() as f64)) / 2.0).ceil() as isize;

    // Must not "overshoot" during (i.e., use same v_y as in pt1)
    let max_vy = -area.lower_right.y() - 1;

    // Must not "overshoot" in one step
    let min_vy = area.lower_right.y();

    let mut num_results = 0;

    // Memo version - slower
    // let mut probe = Probe::from_area(&area);
    // for vx in min_vx..=max_vx {
    //     for vy in min_vy..=max_vy {
    //         num_results += probe.will_hit_target(0, 0, vx, vy);
    //     }
    // }

    // Naive version - faster
    for vx in min_vx..=max_vx {
        for vy in min_vy..=max_vy {
            num_results += simulate(0, 0, vx, vy, &area);
        }
    }

    num_results
}

struct Area2D {
    upper_left: Coordinate,
    lower_right: Coordinate,
}

#[allow(dead_code)]
impl Area2D {

    pub fn coordinates_within(&self, x: isize, y: isize) -> bool {
        self.upper_left.x() <= x && x <= self.lower_right.x() && y <= self.upper_left.y() && self.lower_right.y() <= y
    }

    pub fn from_input(input_str: &str) -> Self {
        let input = input_str.trim_start_matches("target area: ").trim_end_matches("\n");
        let parts: Vec<&str> = input.split(", ").collect();
        let x_range: Vec<isize> = parts[0].trim_start_matches("x=").split("..").map(|s| s.parse().unwrap()).collect();
        let y_range: Vec<isize> = parts[1].trim_start_matches("y=").split("..").map(|s| s.parse().unwrap()).collect();
        Area2D {
            upper_left: Coordinate::from_coordinates(x_range[0], y_range[1]),
            lower_right: Coordinate::from_coordinates(x_range[1], y_range[0]),
        }
    }
}

fn simulate(mut x: isize, mut y: isize, mut vx: isize, mut vy: isize, target_area: &Area2D) -> isize{
    while !((x > target_area.lower_right.x()) || (y < target_area.lower_right.y() && vy <= 0) || (x < target_area.upper_left.x() && vx == 0)) {
        if target_area.coordinates_within(x, y) { return 1 }
        x += vx;
        y += vy;
        vx = max(0, vx - 1);
        vy -= 1;
    }
    0
}

#[allow(dead_code)]
struct Probe<'a> {
    target_area: &'a Area2D,
    memo: HashMap<[isize; 4], isize>,  // x, y, vx, vy
}

#[allow(dead_code)]
impl<'a> Probe<'a> {

    pub fn from_area(area: &'a Area2D) -> Self {
        Probe {
            target_area: area,
            memo: HashMap::new(),
        }
    }

    pub fn will_hit_target(&mut self, x: isize, y: isize, vx: isize, vy: isize) -> isize {
        if let Some(&memo_value) = self.memo.get(&[x, y, vx, vy]) {
            return memo_value
        }

        if self.target_area.coordinates_within(x, y) {
            return 1
        }

        if (x > self.target_area.lower_right.x()) || (y < self.target_area.lower_right.y() && vy <= 0) || (x < self.target_area.upper_left.x() && vx == 0) {
            self.memo.insert([x, y, vx, vy], 0);
            return 0
        }

        let res = self.will_hit_target(x + vx, y + vy, max(0, vx - 1), vy - 1);
        self.memo.insert([x, y, vx, vy], res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_examples_work() {
        assert_eq!(part_1("target area: x=20..30, y=-10..-5"), 45);
        assert_eq!(part_2("target area: x=20..30, y=-10..-5"), 112);
    }

    #[test]
    fn pt_1_works() {
        assert_eq!(part_1("target area: x=257..286, y=-101..-57"), 5_050);
    }

    #[test]
    fn pt_2_works() {
        assert_eq!(part_2("target area: x=257..286, y=-101..-57"), 2_223);
    }
}