use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .unwrap();
    let parsed = contents.lines()
        .map(|s| s.trim())
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    part_one(&parsed);
    part_two(&parsed);
}

fn part_one(input: &Vec<u32>) {
    let sum: u32 = input.iter()
        .map(|mass| calculate_fuel(mass))
        .sum();

    println!("{}", sum);
}

fn part_two(input: &Vec<u32>) {
    let mut total_fuel = 0;
    for m in input.iter() {
        let mut fuel = calculate_fuel(m);
        total_fuel += fuel;
        while fuel > 0 {
            let sub = calculate_fuel(&fuel);
            total_fuel += sub;
            fuel = sub;
        }
    }

    println!("{}", total_fuel)
}

fn calculate_fuel(x: &u32) -> u32 {
    let tmp = x / 3;
    return if tmp < 2 {
        0
    } else {
        tmp - 2
    }
}
