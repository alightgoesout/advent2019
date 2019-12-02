use crate::input::read_lines;

pub fn execute() {
    let input = read_lines("day1").unwrap();
    println!(
        "1:1 — Fuel Requirements Sum: {}",
        fuel_requirement_sum(&input)
    );
    println!(
        "1:2 — Full Fuel Requirements Sum: {}",
        full_fuel_requirement_sum(&input)
    );
}

fn fuel_requirement_sum(input: &Vec<String>) -> u32 {
    input
        .iter()
        .map(|l| l.parse::<u32>().unwrap_or(0))
        .map(|m| fuel_requirement(m))
        .sum()
}

fn fuel_requirement(mass: u32) -> u32 {
    match mass / 3 {
        0..=2 => 0,
        x => x - 2,
    }
}

fn full_fuel_requirement_sum(input: &Vec<String>) -> u32 {
    input
        .iter()
        .map(|l| l.parse::<u32>().unwrap_or(0))
        .map(|m| full_fuel_requirement(m))
        .sum()
}

fn full_fuel_requirement(mass: u32) -> u32 {
    let requirement = fuel_requirement(mass);
    return if requirement == 0 {
        0
    } else {
        requirement + full_fuel_requirement(requirement)
    };
}

#[cfg(test)]
mod fuel_requirement_should {
    use super::fuel_requirement;

    #[test]
    fn return_0_for_a_mass_of_0() {
        let result = fuel_requirement(0);

        assert_eq!(result, 0);
    }

    #[test]
    fn return_2_for_a_mass_of_12() {
        let result = fuel_requirement(12);

        assert_eq!(result, 2);
    }

    #[test]
    fn return_2_for_a_mass_of_14() {
        let result = fuel_requirement(14);

        assert_eq!(result, 2);
    }

    #[test]
    fn return_654_for_a_mass_of_1969() {
        let result = fuel_requirement(1969);

        assert_eq!(result, 654);
    }

    #[test]
    fn return_33583_for_a_mass_of_100756() {
        let result = fuel_requirement(100756);

        assert_eq!(result, 33583);
    }
}

#[cfg(test)]
mod full_fuel_requirement_should {
    use super::full_fuel_requirement;

    #[test]
    fn return_0_for_a_mass_of_0() {
        let result = full_fuel_requirement(0);

        assert_eq!(result, 0);
    }

    #[test]
    fn return_2_for_a_mass_of_12() {
        let result = full_fuel_requirement(14);

        assert_eq!(result, 2);
    }

    #[test]
    fn return_966_for_a_mass_of_1969() {
        let result = full_fuel_requirement(1969);

        assert_eq!(result, 966);
    }

    #[test]
    fn return_50346_for_a_mass_of_100756() {
        let result = full_fuel_requirement(100756);

        assert_eq!(result, 50346);
    }
}
