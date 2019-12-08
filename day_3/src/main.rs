use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let input = parse_input(input);

    let (a, b) = (&input[0], &input[1]);
    let (a, b) = (snake_wire(a), snake_wire(b));
    let result = part_one(&a, &b);
    let result = part_two(&a, &b, result.1);
}

fn parse_input(input: String) -> Vec<Vec<String>> {
    input.lines()
        .map(|v| v.split(",")
            .map(|s| s.to_owned()).collect())
        .collect()
}

fn snake_wire(wire: &Vec<String>) -> HashSet<Point> {
    let mut current_loc = CENTER.clone();
    let mut pos: HashSet<Point> = HashSet::new();

    for part in wire.iter() {
        let op = &part[0..1];
        let num = (&part[1..]).parse::<i64>().unwrap();
        let offset = get_offset(op);

        for _ in 0..num {
            current_loc.add(offset.0, offset.1);
            current_loc.len += 1;
            pos.insert(current_loc.clone());
        }
    }

    pos
}

fn get_offset(direction: &str) -> (i64, i64) {
    match direction {
        "U" => (1, 0),
        "D" => (-1, 0),
        "L" => (0, -1),
        "R" => (0, 1),
        _ => panic!("fuck")
    }
}

#[derive(Debug, Eq, Clone)]
struct Point {
    x: i64,
    y: i64,
    len: i64
}

const CENTER: Point = Point { x: 0, y: 0, len: 0 };

impl Point {
    fn manhattan_dist(&self, b: &Point) -> i64 {
        (self.x - b.x).abs() + (self.y - b.y).abs()
    }

    fn add(&mut self, x: i64, y: i64) {
        self.x += x;
        self.y += y;
    }

    fn is_center(&self) -> bool {
        return self.x == CENTER.x && self.y == CENTER.y
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn part_one(a: &HashSet<Point>, b: &HashSet<Point>) -> (i64, HashSet<Point>) {
    let intersection = a.intersection(b);

    let min = intersection
        .clone()
        .filter(|p| !p.is_center())
        .map(|p| p.manhattan_dist(&CENTER))
        .min();

    (min.unwrap(), intersection.map(|p| p.clone()).collect())
}

fn part_two(a: &HashSet<Point>, b: &HashSet<Point>, intersection: HashSet<Point>) -> i64 {
    let mut min = std::i64::MAX;
    let a: HashMap<&Point, i64> = a.iter().map(|p| (p, p.len))
        .collect();
    let b: HashMap<&Point, i64> = b.iter().map(|p| (p, p.len))
        .collect();

    for p in intersection.iter() {
        let a_len = a.get(p).unwrap();
        let b_len = b.get(p).unwrap();
        let total = a_len + b_len;
        println!();
        if total < min {
            min = total;
        }
    }

    min
}

#[test]
fn input_one() {
    let input = parse_input("R8,U5,L5,D3\nU7,R6,D4,L4".to_owned());
    let (a, b) = (&input[0], &input[1]);
    let (a, b) = (snake_wire(a), snake_wire(b));
    let res = part_one(&a, &b);

    assert_eq!(res.0, 6);
    assert_eq!(part_two(&a, &b, res.1), 30);
}