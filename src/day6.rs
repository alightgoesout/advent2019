use crate::input::read_lines;
use std::collections::HashMap;

pub fn execute() {
    let lines = read_lines("day6").unwrap();
    let orbit_map: MapNode = build_orbit_map(lines.iter().map(|l| split_in_two(l)).collect());
    println!("6:1 — Number of orbits: {}", orbit_map.nb_orbits());
}

fn split_in_two(s: &String) -> (&str, &str) {
    let parts: Vec<&str> = s.split(")").collect();
    match parts[..] {
        [orbited, orbiting] => (orbited, orbiting),
        _ => unreachable!(),
    }
}

fn build_orbit_map<'a>(lines: Vec<(&'a str, &'a str)>) -> MapNode<'a> {
    let mut orbits: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in lines {
        orbits
            .entry(line.0)
            .or_insert_with(|| Vec::new())
            .push(line.1);
    }

    MapNode::new("COM", build_children("COM", &orbits))
}

fn build_children<'a>(
    object: &'a str,
    orbits: &HashMap<&'a str, Vec<&'a str>>,
) -> Option<Vec<MapNode<'a>>> {
    orbits
        .get(object)
        .map(|c| {
            Some(
                c.iter()
                    .map(|o| MapNode::new(o, build_children(o, orbits)))
                    .collect(),
            )
        })
        .unwrap_or_default()
}

struct MapNode<'a> {
    value: &'a str,
    children: Option<Vec<MapNode<'a>>>,
}

impl<'a> MapNode<'a> {
    fn new(value: &'a str, children: Option<Vec<MapNode<'a>>>) -> Self {
        MapNode { value, children }
    }

    fn nb_unique_orbits(&self) -> usize {
        self.children
            .iter()
            .flat_map(|c| c)
            .map(|c| c.nb_unique_orbits() + 1)
            .sum()
    }

    fn nb_orbits(&self) -> usize {
        self.children
            .iter()
            .flat_map(|c| c)
            .map(|c| c.nb_orbits() + c.nb_unique_orbits() + 1)
            .sum()
    }
}

#[cfg(test)]
mod map_node_nb_unique_orbits_should {
    use super::*;

    #[test]
    fn return_0_when_map_is_empty() {
        let map = MapNode::new("COM", None);

        let result = map.nb_unique_orbits();

        assert_eq!(result, 0);
    }

    #[test]
    fn return_1_when_com_has_one_direct_orbit() {
        let map = MapNode::new("COM", Some(vec![MapNode::new("A", None)]));

        let result = map.nb_unique_orbits();

        assert_eq!(result, 1);
    }

    #[test]
    fn return_2_when_com_has_two_direct_orbit() {
        let map = MapNode::new(
            "COM",
            Some(vec![MapNode::new("A", None), MapNode::new("B", None)]),
        );

        let result = map.nb_unique_orbits();

        assert_eq!(result, 2);
    }

    #[test]
    fn return_2_when_com_has_one_direct_orbit_and_one_indirect_orbit() {
        let map = MapNode::new(
            "COM",
            Some(vec![MapNode::new("A", Some(vec![MapNode::new("B", None)]))]),
        );

        let result = map.nb_unique_orbits();

        assert_eq!(result, 2);
    }

    #[test]
    fn return_42_for_the_example() {
        let orbit_map: MapNode = build_orbit_map(vec![
            ("COM", "B"),
            ("B", "C"),
            ("C", "D"),
            ("D", "E"),
            ("E", "F"),
            ("B", "G"),
            ("G", "H"),
            ("D", "I"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L"),
        ]);

        let result = orbit_map.nb_unique_orbits();

        assert_eq!(result, 11);
    }
}

#[cfg(test)]
mod map_node_nb_orbits_should {
    use super::*;

    #[test]
    fn return_3_when_com_has_one_direct_orbit_and_one_indirect_orbit() {
        let map = MapNode::new(
            "COM",
            Some(vec![MapNode::new("A", Some(vec![MapNode::new("B", None)]))]),
        );

        let result = map.nb_orbits();

        assert_eq!(result, 3);
    }

    #[test]
    fn return_42_for_the_example() {
        let orbit_map: MapNode = build_orbit_map(vec![
            ("COM", "B"),
            ("B", "C"),
            ("C", "D"),
            ("D", "E"),
            ("E", "F"),
            ("B", "G"),
            ("G", "H"),
            ("D", "I"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L"),
        ]);

        let result = orbit_map.nb_orbits();

        assert_eq!(result, 42);
    }
}
