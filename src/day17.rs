use std::cmp::Ordering;
use std::num;

#[derive(Debug, Clone)]
pub struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
pub struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
pub struct TargetArea {
    top_left: Coord,
    bottom_right: Coord,
}
impl TargetArea {
    pub fn new(line: &str) -> Result<TargetArea, num::ParseIntError> {
        // target area: x=20..30, y=-10..-5

        let line = line.replace("target area: ", "");
        let line = line.replace("x=", "");
        let line = line.replace("y=", "");
        // 20..30, 0..-5

        let raw_coords = line.split(", ").collect::<Vec<&str>>();
        let raw_coords = raw_coords
            .iter()
            .map(|z| z.split("..").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        // [[20, 30], [0, -5]]

        let top_left_x = raw_coords[0][0].parse::<i32>()?;
        let top_left_y = raw_coords[1][1].parse::<i32>()?;

        let bottom_right_x = raw_coords[0][1].parse::<i32>()?;
        let bottom_right_y = raw_coords[1][0].parse::<i32>()?;

        let top_left = Coord {
            x: top_left_x,
            y: top_left_y,
        };
        let bottom_right = Coord {
            x: bottom_right_x,
            y: bottom_right_y,
        };
        Ok(TargetArea {
            top_left,
            bottom_right,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Projectile {
    position: Coord,
    velocity: Velocity,
    max_y: i32,
}
impl Projectile {
    pub fn new(position: Coord, velocity: Velocity) -> Projectile {
        Projectile {
            max_y: position.y,
            position,
            velocity,
        }
    }

    fn do_step(&mut self) {
        // Find new position
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        if self.position.y > self.max_y {
            self.max_y = self.position.y;
        }

        // Find new x velocity
        let x_drag_change = match self.velocity.x.cmp(&0) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        };

        self.velocity.x += x_drag_change;

        // Find new y velocity
        let y_gravity_change = -1;
        self.velocity.y += y_gravity_change;
    }

    pub fn does_hit_area_after_fire(&mut self, area: &TargetArea) -> bool {
        loop {
            if self.in_area(area) {
                return true;
            } else if self.cant_hit_area(area) {
                return false;
            }
            self.do_step();
        }
    }
    fn in_area(&self, area: &TargetArea) -> bool {
        if (self.position.x >= area.top_left.x)
            && (self.position.x <= area.bottom_right.x)
            && (self.position.y <= area.top_left.y)
            && (self.position.y >= area.bottom_right.y)
        {
            return true;
        }

        false
    }
    fn cant_hit_area(&self, area: &TargetArea) -> bool {
        #[allow(clippy::if_same_then_else)]
        if (self.position.x > area.bottom_right.x) || (self.position.y < area.bottom_right.y) {
            // Overshot
            return true;
        } else if (self.position.x < area.top_left.x) && (self.velocity.x == 0) {
            // Stopped short
            return true;
        }
        false
    }
}

fn parse_input_lines(raw_input_lines: &[String]) -> Result<TargetArea, num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();
    assert!(input_lines.len() == 1);
    TargetArea::new(input_lines[0])
}

pub fn part_1(target_area: &TargetArea) -> i32 {
    let min_x_velocity = 1;
    let max_x_velocity = target_area.bottom_right.x;

    let min_y_velocity = target_area.bottom_right.y;
    let max_y_velocity = target_area.bottom_right.y.abs();

    for y_velocity in (min_y_velocity..=max_y_velocity).rev() {
        for x_velocity in (min_x_velocity..=max_x_velocity).rev() {
            let mut projectile = Projectile::new(
                Coord { x: 0, y: 0 },
                Velocity {
                    x: x_velocity,
                    y: y_velocity,
                },
            );

            if projectile.does_hit_area_after_fire(target_area) {
                // Started with looping largest Y velocity, so this must be the solution
                return projectile.max_y as i32;
            }
        }
    }

    panic!("No solution!");
}

pub fn part_2(target_area: &TargetArea) -> i32 {
    let min_x_velocity = 1;
    let max_x_velocity = target_area.bottom_right.x;

    let min_y_velocity = target_area.bottom_right.y;
    let max_y_velocity = target_area.bottom_right.y.abs();

    let mut num_diff_velocities = 0;

    for x_velocity in min_x_velocity..=max_x_velocity {
        for y_velocity in min_y_velocity..=max_y_velocity {
            let mut projectile = Projectile::new(
                Coord { x: 0, y: 0 },
                Velocity {
                    x: x_velocity,
                    y: y_velocity,
                },
            );

            if projectile.does_hit_area_after_fire(target_area) {
                num_diff_velocities += 1;
            }
        }
    }

    num_diff_velocities as i32
}

pub fn day17(input_lines: &[String]) -> (u64, u64) {
    let encoded_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error : {} , when trying to parse the input lines", err);
    });
    (part_1(&encoded_data) as u64, part_2(&encoded_data) as u64)
}
