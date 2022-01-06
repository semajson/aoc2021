use std::{collections::btree_set::Intersection, num};
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
            max_y: position.y.clone(),
            position,
            velocity,
        }
    }

    pub fn do_step(&mut self) {
        // Find new position
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        if self.position.y > self.max_y {
            self.max_y = self.position.y;
        }

        // Find new x velocity
        let mut x_drag_change = 0;
        if self.velocity.x > 0 {
            x_drag_change = -1;
        } else if self.velocity.x < 0 {
            x_drag_change = 1;
        }

        self.velocity.x += x_drag_change;

        // Find new y velocity
        let y_gravity_change = -1;
        self.velocity.y += y_gravity_change;
    }

    pub fn does_projectile_hit_area_after_fire(&mut self, area: &TargetArea) -> bool {
        loop {
            if self.in_area(&area) {
                return true;
            } else if self.cant_hit_area(&area) {
                return false;
            }
            self.do_step();
        }
    }
    pub fn in_area(&self, area: &TargetArea) -> bool {
        if (self.position.x >= area.top_left.x)
            && (self.position.x <= area.bottom_right.x)
            && (self.position.y <= area.top_left.y)
            && (self.position.y >= area.bottom_right.y)
        {
            return true;
        }

        false
    }
    pub fn cant_hit_area(&self, area: &TargetArea) -> bool {
        if (self.position.x > area.bottom_right.x) || (self.position.y < area.bottom_right.y) {
            // Overshot
            return true;
        } else if (self.position.x < area.top_left.x) && (self.velocity.x == 0) {
            // Stopped
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
    let mut current_max_y = 0;

    let min_x_velocity = 1;
    let max_x_velocity = target_area.bottom_right.x;

    let min_y_velocity = target_area.bottom_right.y;
    let max_y_velocity = max_x_velocity + 1;

    for x_velocity in min_x_velocity..=max_x_velocity {
        for y_velocity in min_y_velocity..=max_y_velocity {
            let mut projectile = Projectile::new(
                Coord { x: 0, y: 0 },
                Velocity {
                    x: x_velocity,
                    y: y_velocity,
                    // x: 6,
                    // y: -10,
                },
            );

            // println!("{:?}", projectile);
            if projectile.does_projectile_hit_area_after_fire(target_area) {
                if projectile.max_y > current_max_y {
                    current_max_y = projectile.max_y;
                }
            }
        }
    }

    current_max_y as i32
}

pub fn part_2(encoded_data: &TargetArea) -> i32 {
    0
}

pub fn day17(input_lines: &[String]) -> (u64, u64) {
    let encoded_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error : {} , when trying to parse the input lines", err);
    });
    (part_1(&encoded_data) as u64, 0)
}
