use std::cmp::Ordering;

pub fn execute() {
    println!(
        "4:1 — Number of valid passwords in range: {:?}",
        count_valid_passwords(271973, 785961)
    );
    println!(
        "4:2 — Number of valid passwords in range: {:?}",
        count_valid_passwords2(271973, 785961)
    );
}

fn count_valid_passwords(start: u32, end: u32) -> u32 {
    (start..end + 1).filter({ |p| is_valid(*p) }).count() as u32
}

fn count_valid_passwords2(start: u32, end: u32) -> u32 {
    (start..end + 1).filter({ |p| is_valid2(*p) }).count() as u32
}

fn is_valid(password: u32) -> bool {
    let mut last_digit = password % 10;
    let mut digits = password / 10;
    let mut double = false;
    while digits > 0 {
        let new_digit = digits % 10;
        match last_digit.cmp(&new_digit) {
            Ordering::Equal => double = true,
            Ordering::Less => return false,
            _ => (),
        }
        digits /= 10;
        last_digit = new_digit;
    }
    double
}

fn is_valid2(password: u32) -> bool {
    let mut last_digit = password % 10;
    let mut digits = password / 10;
    let mut identical_count = 1;
    let mut double = false;
    while digits > 0 {
        let new_digit = digits % 10;
        digits /= 10;
        match last_digit.cmp(&new_digit) {
            Ordering::Equal => identical_count += 1,
            Ordering::Less => return false,
            Ordering::Greater => {
                double = double || identical_count == 2;
                identical_count = 1;
                last_digit = new_digit;
            }
        }
    }
    double || identical_count == 2
}

#[cfg(test)]
mod is_valid_should {
    use super::*;

    #[test]
    fn return_true_for_111111() {
        assert_eq!(is_valid(111111), true);
    }

    #[test]
    fn return_false_for_223450() {
        assert_eq!(is_valid(223450), false);
    }

    #[test]
    fn return_false_for_123789() {
        assert_eq!(is_valid(123789), false);
    }

    #[test]
    fn return_true_for_123455() {
        assert_eq!(is_valid(123455), true);
    }
}

#[cfg(test)]
mod is_valid2_should {
    use super::*;

    #[test]
    fn return_true_for_112233() {
        assert_eq!(is_valid2(112233), true);
    }

    #[test]
    fn return_false_for_123444() {
        assert_eq!(is_valid2(123444), false);
    }

    #[test]
    fn return_true_for_111122() {
        assert_eq!(is_valid2(111122), true);
    }

    #[test]
    fn return_true_for_112345() {
        assert_eq!(is_valid2(112345), true);
    }
}
