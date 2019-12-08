use crate::input::read_lines;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn execute() {
    let lines = read_lines("day6").unwrap();
    let orbit_map: MapNode = build_orbit_map(lines.iter().map(|l| split_in_two(l)).collect());
    println!("6:1 — Number of orbits: {}", orbit_map.nb_orbits());
    println!("6:2 — Distance with Santa: {}", orbit_map.distance("SAN", "YOU"));
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

    fn find(&self, value: &str) -> Option<Vec<&str>> {
        match self.value.cmp(value) {
            Ordering::Equal => Some(Vec::new()),
            _ => self
                .children
                .iter()
                .flat_map(|c| c)
                .flat_map(|n| n.find(value))
                .next()
                .map(|mut v| {
                    v.insert(0, self.value);
                    v
                }),
        }
    }

    fn distance(&self, n1: &str, n2: &str) -> usize {
        let v1 = self.find(n1).unwrap_or_else(|| Vec::new());
        let v2 = self.find(n2).unwrap_or_else(|| Vec::new());
        let (v1, v2) = remove_prefix(v1, v2);
        v1.len() + v2.len()
    }
}

fn remove_prefix<'a>(mut v1: Vec<&'a str>, mut v2: Vec<&'a str>) -> (Vec<&'a str>, Vec<&'a str>) {
    while let (Some(i), Some(j)) = (v1.get(0), v2.get(0)) {
        if i == j {
            v1.remove(0);
            v2.remove(0);
        } else {
            break;
        }
    }
    (v1, v2)
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
        let orbit_map = build_orbit_map(vec![
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

#[cfg(test)]
mod map_node_find_should {
    use super::*;

    fn orbit_map<'a>() -> MapNode<'a> {
        build_orbit_map(vec![
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
            ("K", "YOU"),
            ("I", "SAN"),
        ])
    }

    #[test]
    fn return_none_when_value_is_not_found() {
        let orbit_map = orbit_map();

        let result = orbit_map.find("N");

        assert_eq!(result, None);
    }

    #[test]
    fn return_an_empty_vector_when_value_is_com() {
        let orbit_map = orbit_map();

        let result = orbit_map.find("COM");

        assert_eq!(result, Some(Vec::new()));
    }

    #[test]
    fn return_com_when_value_is_b() {
        let orbit_map = orbit_map();

        let result = orbit_map.find("B");

        assert_eq!(result, Some(vec!["COM"]));
    }

    #[test]
    fn return_com_b_c_d_e_j_k_when_value_is_you() {
        let orbit_map = orbit_map();

        let result = orbit_map.find("YOU");

        assert_eq!(result, Some(vec!["COM", "B", "C", "D", "E", "J", "K"]));
    }

    #[test]
    fn return_com_b_c_d_i_when_value_is_san() {
        let orbit_map = orbit_map();

        let result = orbit_map.find("SAN");

        assert_eq!(result, Some(vec!["COM", "B", "C", "D", "I"]));
    }
}

#[cfg(test)]
mod remove_prefix_should {
    use super::*;

    #[test]
    fn return_initial_vectors_when_there_is_no_common_prefix() {
        let v1 = vec!["A", "B"];
        let v2 = vec!["C", "D"];

        let result = remove_prefix(v1, v2);

        assert_eq!(result, (vec!["A", "B"], vec!["C", "D"]));
    }

    #[test]
    fn return_vectors_without_common_prefix_when_it_has_a_length_of_one() {
        let v1 = vec!["A", "B"];
        let v2 = vec!["A", "C"];

        let result = remove_prefix(v1, v2);

        assert_eq!(result, (vec!["B"], vec!["C"]));
    }
}
