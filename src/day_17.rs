use crate::array_2d::Coordinate;

pub fn part_1(input: &str) -> isize {
    // One time tick after reaching 0 in y, we get to position (-v_y - 1), since it's reverse of how we went up in the start
        // Since we want to maximize y_max = (v_y^2 + v_y) / 2, which is increasing, we simply find highest v_y s.t. (-v_y - 1) >= bottom, i.e., v_y = (-bottom - 1)
    // v_x is then one that gets "stuck" on any x level within
    let area = Area2D::from_input(input);
    let best_v_y = -area.lower_right.y() - 1;
    (best_v_y.pow(2) + best_v_y) / 2
}

pub fn part_2(input: &str) -> u128 {
    0
}

struct Area2D {
    upper_left: Coordinate,
    lower_right: Coordinate,
}

#[allow(dead_code)]
impl Area2D {
    pub fn point_within(&self, point: Coordinate) -> bool {
        self.upper_left.x() < point.x() && point.x() < self.lower_right.x() && point.y() < self.upper_left.y() && self.lower_right.y() < point.y()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_examples_work() {
        assert_eq!(part_1("target area: x=20..30, y=-10..-5"), 45);
    }

    #[test]
    fn aoc_examples_work() {
        assert_eq!(part_1("target area: x=257..286, y=-101..-57"), 5050);
    }
}