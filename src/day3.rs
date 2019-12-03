use im_rc::{OrdMap, OrdSet, Vector};

pub fn execute() {}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct PathInstruction {
    direction: Direction,
    amount: u32,
}

struct Path {
    instructions: Vector<PathInstruction>,
}

#[derive(Clone, PartialEq, Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(PartialEq, Debug)]
struct Wire {
    points: OrdMap<u32, OrdSet<u32>>,
}

impl Wire {
    pub fn new() -> Wire {
        Wire {
            points: OrdMap::new(),
        }
    }

    pub fn update(&self, x: u32, y: u32) -> Wire {
        Wire {
            points: self.points.alter(
                { |o| o.or_else({ || Some(OrdSet::new()) }).map(|p| p.update(y)) },
                x,
            ),
        }
    }
}

impl Wire {
    pub fn from(origin: Point, path: Path) -> Wire {
        let mut wire = Wire::new();
        for instruction in path.instructions {
            match instruction.direction {
                Direction::Right => {
                    for x in origin.x..(origin.x + instruction.amount + 1) {
                        wire = wire.update(x, origin.y);
                    }
                }
                Direction::Left => {
                    for x in (origin.x - instruction.amount)..(origin.x + 1) {
                        wire = wire.update(x, origin.y);
                    }
                }
                Direction::Up => {
                    for y in origin.y..(origin.y + instruction.amount + 1) {
                        wire = wire.update(origin.x, y);
                    }
                }
                Direction::Down => {
                    for y in (origin.y - instruction.amount)..(origin.y + 1) {
                        wire = wire.update(origin.x, y);
                    }
                }
            }
        }

        wire
    }
}

#[cfg(test)]
mod wire_from_origin_path_should {
    use super::{Direction, Path, PathInstruction, Point, Wire};
    use im_rc::{OrdMap, OrdSet};

    #[test]
    fn create_an_horizontal_wire_when_path_is_r4() {
        let origin = Point { x: 1, y: 1 };
        let path = Path {
            instructions: im_rc::vector![PathInstruction {
                direction: Direction::Right,
                amount: 4
            }],
        };

        let result = Wire::from(origin, path);

        assert_eq!(
            Wire {
                points: OrdMap::from(vec![
                    (1u32, OrdSet::from(vec![1u32])),
                    (2u32, OrdSet::from(vec![1u32])),
                    (3u32, OrdSet::from(vec![1u32])),
                    (4u32, OrdSet::from(vec![1u32])),
                    (5u32, OrdSet::from(vec![1u32])),
                ])
            },
            result
        );
    }

    #[test]
    fn create_an_horizontal_wire_when_path_is_l4() {
        let origin = Point { x: 4, y: 1 };
        let path = Path {
            instructions: im_rc::vector![PathInstruction {
                direction: Direction::Left,
                amount: 4
            }],
        };

        let result = Wire::from(origin, path);

        assert_eq!(
            Wire {
                points: OrdMap::from(vec![
                    (0u32, OrdSet::from(vec![1u32])),
                    (1u32, OrdSet::from(vec![1u32])),
                    (2u32, OrdSet::from(vec![1u32])),
                    (3u32, OrdSet::from(vec![1u32])),
                    (4u32, OrdSet::from(vec![1u32])),
                ])
            },
            result
        );
    }

    #[test]
    fn create_an_vertical_wire_when_path_is_u4() {
        let origin = Point { x: 1, y: 1 };
        let path = Path {
            instructions: im_rc::vector![PathInstruction {
                direction: Direction::Up,
                amount: 4
            }],
        };

        let result = Wire::from(origin, path);

        assert_eq!(
            Wire {
                points: OrdMap::from(vec![(
                    1u32,
                    OrdSet::from(vec![1u32, 2u32, 3u32, 4u32, 5u32])
                ),])
            },
            result
        );
    }

    #[test]
    fn create_an_vertical_wire_when_path_is_d4() {
        let origin = Point { x: 1, y: 4 };
        let path = Path {
            instructions: im_rc::vector![PathInstruction {
                direction: Direction::Down,
                amount: 4
            }],
        };

        let result = Wire::from(origin, path);

        assert_eq!(
            Wire {
                points: OrdMap::from(vec![(
                    1u32,
                    OrdSet::from(vec![0u32, 1u32, 2u32, 3u32, 4u32])
                ),])
            },
            result
        );
    }
}
