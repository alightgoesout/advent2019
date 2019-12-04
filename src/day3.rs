use crate::input::read_lines;
use im_rc::{HashMap, HashSet, Vector};
use std::cmp::Ordering;
use std::iter::FromIterator;

pub fn execute() {
    let input = read_lines("day3").unwrap();
    let wires: Vector<Wire> = input.iter().map({ |l| str_to_wire(l) }).collect();
    let w1 = &wires[0];
    let w2 = &wires[1];
    let intersections = compute_intersections(w1, w2);
    println!(
        "3:1 — Closest intersection distance: {:?}",
        find_closest_intersection(&intersections)
            .map({ |p| distance_from_central_port(&p) })
            .unwrap()
    );
    println!(
        "3:2 — Lowest steps to an intersection: {:?}",
        compute_lowest_steps_to_intersection(w1, w2, &intersections).unwrap()
    );
}

fn find_closest_intersection(intersections: &HashSet<Point>) -> Option<Point> {
    intersections
        .iter()
        .min_by({ |p1, p2| compare_points(p1, p2) })
        .map({ |p| p.clone() })
}

fn compute_lowest_steps_to_intersection(
    w1: &Wire,
    w2: &Wire,
    intersections: &HashSet<Point>,
) -> Option<usize> {
    let w1_steps = compute_steps_to_intersection(w1, intersections);
    let w2_steps = compute_steps_to_intersection(w2, intersections);
    w1_steps
        .iter()
        .map({ |(p, s)| w2_steps.get(p).unwrap() + s })
        .min()
}

fn compute_steps_to_intersection(
    wire: &Wire,
    intersections: &HashSet<Point>,
) -> HashMap<Point, usize> {
    let mut result = HashMap::new();
    wire.iter()
        .enumerate()
        .filter({ |(_, p)| intersections.contains(p) })
        .for_each({
            |(i, p)| {
                if !result.contains_key(p) {
                    result.insert(p.clone(), i + 1);
                }
            }
        });
    result
}

fn compute_intersections(w1: &Wire, w2: &Wire) -> HashSet<Point> {
    let w2_points = HashSet::<&Point>::from_iter(w2.iter());
    HashSet::from_iter(
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

fn str_to_wire(instructions: &str) -> Wire {
    instructions_to_wire(
        &instructions
            .split(",")
            .map(|i| Instruction::from(i))
            .collect(),
    )
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

#[cfg(test)]
mod compute_steps_to_intersection_should {
    use super::*;

    #[test]
    fn return_15_and_20_for_the_first_example() {
        let w1 = str_to_wire("R8,U5,L5,D3");
        let w2 = str_to_wire("U7,R6,D4,L4");
        let intersections = compute_intersections(&w1, &w2);

        let result = compute_steps_to_intersection(&w1, &intersections);

        assert_eq!(HashMap::from(vec![((6, 5), 15), ((3, 3), 20)]), result);
    }
}

#[cfg(test)]
mod compute_lowest_steps_to_intersection_should {
    use super::*;
    use im_rc::Vector;

    #[test]
    fn return_30_for_the_first_example() {
        let w1 = str_to_wire("R8,U5,L5,D3");
        let w2 = str_to_wire("U7,R6,D4,L4");
        let intersections = compute_intersections(&w1, &w2);

        let result = compute_lowest_steps_to_intersection(&w1, &w2, &intersections);

        assert_eq!(Some(30), result);
    }

    #[test]
    fn return_610_for_the_second_example() {
        let w1 = str_to_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let w2 = str_to_wire("U62,R66,U55,R34,D71,R55,D58,R83");
        let intersections = compute_intersections(&w1, &w2);

        let result = compute_lowest_steps_to_intersection(&w1, &w2, &intersections);

        assert_eq!(Some(610), result);
    }

    #[test]
    fn return_410_for_the_third_example() {
        let w1 = instructions_to_wire(&Vector::from(vec![
            Instruction::from("R98"),
            Instruction::from("U47"),
            Instruction::from("R26"),
            Instruction::from("D63"),
            Instruction::from("R33"),
            Instruction::from("U87"),
            Instruction::from("L62"),
            Instruction::from("D20"),
            Instruction::from("R33"),
            Instruction::from("U53"),
            Instruction::from("R51"),
        ]));
        let w2 = instructions_to_wire(&Vector::from(vec![
            Instruction::from("U98"),
            Instruction::from("R91"),
            Instruction::from("D20"),
            Instruction::from("R16"),
            Instruction::from("D67"),
            Instruction::from("R40"),
            Instruction::from("U7"),
            Instruction::from("R15"),
            Instruction::from("U6"),
            Instruction::from("R7"),
        ]));
        let intersections = compute_intersections(&w1, &w2);

        let result = compute_lowest_steps_to_intersection(&w1, &w2, &intersections);

        assert_eq!(Some(410), result);
    }
}
