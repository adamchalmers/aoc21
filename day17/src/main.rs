use std::{cmp::Ordering, fmt, ops::AddAssign};

use itertools::Itertools;

type Scale = i32;
fn main() {
    let target = Box {
        top_left: Point { x: 282, y: -45 },
        bottom_right: Point { x: 314, y: -80 },
    };
    println!("Q1: {:?}", trick_shot(target, 100));
    println!("Q2: {}", all_velocities_that_hit(target, 1000));
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Debug)]
struct Point {
    x: Scale,
    y: Scale,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

/// Note the orientation of the plane:
///   y
///   ↑
/// (0,0) → x
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Box {
    top_left: Point,
    bottom_right: Point,
}

impl fmt::Display for Box {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.top_left, self.bottom_right)
    }
}

/// 2D velocity with an X and Y component.
/// {x:0,y:10} would fire the probe straight up
/// {x:10,y:-1} would fire the probe forward at a slight downward angle.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
struct Velocity {
    x: Scale,
    y: Scale,
}

impl Velocity {
    /// Does a rocket launched with the given velocity hit the target?
    /// If no, return None.
    /// If so, return the highest y-position the rocket reaches.
    fn path_collides(&self, target: Box) -> Option<Scale> {
        let mut position = Point::default();
        let mut max_y = position.y;
        let mut velocity = *self;
        loop {
            if position.within(target) {
                return Some(max_y);
            }
            if position.has_passed(target) {
                return None;
            }
            position += velocity;
            velocity.degrade();
            if position.y > max_y {
                max_y = position.y;
            }
        }
    }

    /// Velocities degrade over time, due to gravity and drag.
    fn degrade(&mut self) {
        // Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it
        // decreases by 1 if it is greater than 0,
        // increases by 1 if it is less than 0,
        match self.x.cmp(&0) {
            Ordering::Greater => self.x -= 1,
            Ordering::Less => self.x += 1,
            _ => {}
        }
        // Due to gravity, the probe's y velocity decreases by 1.
        self.y -= 1;
    }
}

fn all_velocities_that_hit(target: Box, bound: u16) -> usize {
    (-(bound as Scale)..bound as Scale)
        .cartesian_product(-(bound as Scale)..bound as Scale)
        .map(|(x, y)| Velocity { x, y })
        .filter(|v| v.path_collides(target).is_some())
        .count()
}

/// Find the velocity which reaches the highest y-position and still passes through the target.
/// What is that y position?
fn trick_shot(target: Box, bound: u8) -> Option<Scale> {
    (-(bound as Scale)..bound as Scale)
        .cartesian_product(-(bound as Scale)..bound as Scale)
        .map(|(x, y)| Velocity { x, y })
        .filter_map(|v| v.path_collides(target))
        .max()
}

/// Adding a velocity to a point represents moving the point along the velocity.
impl AddAssign<Velocity> for Point {
    fn add_assign(&mut self, rhs: Velocity) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Point {
    fn within(&self, target: Box) -> bool {
        self.x >= target.top_left.x
            && self.x <= target.bottom_right.x
            && self.y <= target.top_left.y
            && self.y >= target.bottom_right.y
    }

    fn has_passed(&self, target: Box) -> bool {
        self.y < target.bottom_right.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_TARGET: Box = Box {
        top_left: Point { x: 20, y: -5 },
        bottom_right: Point { x: 30, y: -10 },
    };

    #[test]
    fn test_path_collides() {
        for (velocity, expect_collision) in [
            (Velocity { x: 7, y: 2 }, true),
            (Velocity { x: 6, y: 3 }, true),
            (Velocity { x: 9, y: 0 }, true),
            (Velocity { x: 17, y: -4 }, false),
        ] {
            assert_eq!(
                velocity.path_collides(EXAMPLE_TARGET).is_some(),
                expect_collision,
                "{:?} should{} hit",
                velocity,
                if expect_collision { "" } else { " not" }
            )
        }
    }

    #[test]
    fn test_within() {
        for (point, expect_contains) in [
            (Point { x: 0, y: 0 }, false),
            (Point { x: 25, y: -7 }, true),
        ] {
            assert_eq!(
                point.within(EXAMPLE_TARGET),
                expect_contains,
                "{} should{} be within target",
                point,
                if expect_contains { "" } else { " not" }
            )
        }
    }

    #[test]
    fn test_trick_shot() {
        assert_eq!(trick_shot(EXAMPLE_TARGET, 10), Some(45));
    }

    #[test]
    fn test_all_velocities_that_hit() {
        let actual = all_velocities_that_hit(EXAMPLE_TARGET, 1000);
        assert_eq!(actual, 112);
    }
}
