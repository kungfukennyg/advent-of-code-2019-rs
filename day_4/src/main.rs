use std::mem::MaybeUninit;
use std::collections::HashMap;

fn main() {
    let res = test_in_range(std::fs::read_to_string("./input.txt").unwrap().as_str());
    println!("{:?}", res);
}

fn test_in_range(input: &str) -> (u32, u32) {
    let (min, max) = parse_input(input);

    let (mut count_one, mut count_two) = (0, 0);
    for num in min..max {
        let num_str = num.to_string();
        let num_str = num_str.as_str();
        if !digits_increasing(num_str) {
            continue;
        }

        if has_adjacent_digits(num_str) {
            count_one += 1;
        }
        if has_adjacent_digits_part_two(num_str) {
            count_two += 1;
        }
    }

    (count_one, count_two)
}

fn has_adjacent_digits(num: &str) -> bool {
    let mut prev = 'a';
    for d in num.chars() {
        if d == prev {
            return true;
        }

        prev = d;
    }

    false
}

fn has_adjacent_digits_part_two(num: &str) -> bool {
    let mut prev = 'a';
    let mut counts: HashMap<char, u32> = HashMap::new();
    num.chars().for_each(|c| {
        *counts.entry(c).or_insert(0) += 1
    });

    return counts.iter()
        .map(|e| e.1.clone())
        .filter(|c| *c == 2)
        .count() > 0
}

fn digits_increasing(num: &str) -> bool {
    let mut min = std::u32::MIN;
    for d in num.chars() {
        let num = d.to_digit(10).unwrap();
        if num < min {
            return false;
        }

        min = num;
    }

    true
}

fn parse_input(input: &str) -> (u32, u32) {
    let parts: Vec<&str> = input.split("-").collect();
    let low = parts[0].parse::<u32>().unwrap();
    let high = parts[1].parse::<u32>().unwrap();

    (low, high)
}

#[test]
fn test_has_adjacent_digits_true() {
    assert_eq!(has_adjacent_digits("122345"), true);
}

#[test]
fn test_has_adjacent_digits_false() {
    assert_eq!(has_adjacent_digits("12345"), false);
}


#[test]
fn test_has_adjacent_digits_part_two_true() {
    assert_eq!(has_adjacent_digits_part_two("111122"), true);
    assert_eq!(has_adjacent_digits_part_two("112233"), true);
}

#[test]
fn test_has_adjacent_digits_part_two_false() {
    assert_eq!(has_adjacent_digits_part_two("123444"), false);
}

#[test]
fn test_digits_increasing_true() {
    assert_eq!(digits_increasing("12345"), true);
}

#[test]
fn test_digits_increasing_false() {
    assert_eq!(digits_increasing("12435"), false);
}