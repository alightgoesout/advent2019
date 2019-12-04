use crate::input::read_lines;
use im_rc::{Vector, HashSet};
use std::cmp::Ordering;
use std::iter::FromIterator;

pub fn execute() {
    let input = read_lines("day3").unwrap();
    let wires: Vector<Wire> = input
        .iter()
        .map({ |l| l.split(",").map(|i| Instruction::from(i)).collect() })
        .map({ |i| instructions_to_wire(&i) })
        .collect();
    println!(
        "3:1 — Closest intersection: {:?}",
        find_closest_intersections(&wires[0], &wires[1])
            .map({ |p| distance_from_central_port(&p) })
            .unwrap()
    );
}

fn find_closest_intersections(w1: &Wire, w2: &Wire) -> Option<Point> {
    compute_intersections(w1, w2)
        .iter()
        .min_by({ |p1, p2| compare_points(p1, p2) })
        .map({ |p| p.clone() })
}

fn compute_intersections(w1: &Wire, w2: &Wire) -> Vector<Point> {
    let w2_points = HashSet::<&Point>::from_iter(w2.iter());
    Vector::from_iter(
        w1.iter()
            .filter({ |p| w2_points.contains(p) })
            .map({ |p| p.clone() }),
    )
}

fn compare_points(p1: &Point, p2: &Point) -> Ordering {
    distance_from_central_port(p1).cmp(&distance_from_central_port(p2))
}

fn distance_from_central_port(point: &Point) -> i32 {
    point.0.abs() + point.1.abs()
}

type Point = (i32, i32);
type Wire = Vector<Point>;

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Instruction {
    direction: Direction,
    amount: i32,
}

impl Direction {
    fn from(s: &str) -> Direction {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unknown direction {}", s),
        }
    }
}

impl Instruction {
    pub fn from(instruction: &str) -> Instruction {
        let (start, end) = instruction.split_at(1);
        Instruction {
            direction: Direction::from(start),
            amount: end.parse::<i32>().unwrap(),
        }
    }
}

fn instructions_to_wire(instructions: &Vector<Instruction>) -> Wire {
    let mut wire = Wire::new();
    let mut p = (0, 0);
    for instruction in instructions {
        match instruction.direction {
            Direction::Right => {
                for x in (p.0 + 1)..(p.0 + instruction.amount + 1) {
                    wire.push_back((x, p.1));
                }
                p = (p.0 + instruction.amount, p.1);
            }
            Direction::Left => {
                for x in ((p.0 - instruction.amount)..p.0).rev() {
                    wire.push_back((x, p.1));
                }
                p = (p.0 - instruction.amount, p.1);
            }
            Direction::Up => {
                for y in (p.1 + 1)..(p.1 + instruction.amount + 1) {
                    wire.push_back((p.0, y));
                }
                p = (p.0, p.1 + instruction.amount);
            }
            Direction::Down => {
                for y in ((p.1 - instruction.amount)..p.1).rev() {
                    wire.push_back((p.0, y));
                }
                p = (p.0, p.1 - instruction.amount);
            }
        }
    }
    wire
}

#[cfg(test)]
mod instructions_to_wire_should {
    use super::*;
    use im_rc::Vector;

    #[test]
    fn create_an_horizontal_wire_when_path_is_r4() {
        let instructions = Vector::from(vec![Instruction {
            direction: Direction::Right,
            amount: 4,
        }]);

        let result = instructions_to_wire(&instructions);

        assert_eq!(Wire::from(vec![(1, 0), (2, 0), (3, 0), (4, 0)]), result);
    }

    #[test]
    fn create_an_horizontal_wire_when_path_is_l4() {
        let instructions = Vector::from(vec![Instruction {
            direction: Direction::Left,
            amount: 4,
        }]);

        let result = instructions_to_wire(&instructions);

        assert_eq!(Wire::from(vec![(-1, 0), (-2, 0), (-3, 0), (-4, 0)]), result);
    }

    #[test]
    fn create_an_vertical_wire_when_path_is_u4() {
        let instructions = Vector::from(vec![Instruction {
            direction: Direction::Up,
            amount: 4,
        }]);

        let result = instructions_to_wire(&instructions);

        assert_eq!(Wire::from(vec![(0, 1), (0, 2), (0, 3), (0, 4)]), result);
    }

    #[test]
    fn create_an_vertical_wire_when_path_is_d4() {
        let instructions = Vector::from(vec![Instruction {
            direction: Direction::Down,
            amount: 4,
        }]);

        let result = instructions_to_wire(&instructions);

        assert_eq!(Wire::from(vec![(0, -1), (0, -2), (0, -3), (0, -4)]), result);
    }

    #[test]
    fn crate_a_wire_from_a_path_with_multiple_instructions() {
        let instructions = Vector::from(vec![
            Instruction::from("R8"),
            Instruction::from("U5"),
            Instruction::from("L5"),
            Instruction::from("D3"),
        ]);

        let result = instructions_to_wire(&instructions);

        assert_eq!(
            Wire::from(vec![
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 0),
                (7, 0),
                (8, 0),
                (8, 1),
                (8, 2),
                (8, 3),
                (8, 4),
                (8, 5),
                (7, 5),
                (6, 5),
                (5, 5),
                (4, 5),
                (3, 5),
                (3, 4),
                (3, 3),
                (3, 2),
            ]),
            result
        );
    }
}
