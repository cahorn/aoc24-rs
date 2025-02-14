use regex::Regex;
use std::collections::HashMap;
use std::iter;

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for (_, [l, r]) in re.captures_iter(input).map(|c| c.extract()) {
        let err = "Failed to parse i32";
        left.push(l.parse().expect(err));
        right.push(r.parse().expect(err))
    }
    return (left, right)
}

pub fn part1(input: &str) -> i32 {
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();

    let mut sum = 0;
    for (l, r) in iter::zip(left, right) {
        sum += i32::abs(l - r)
    }
    return sum;
}

pub fn part2(input: &str) -> i32 {
    let (left, right) = parse(input);
    let counts = occurrences(&right);

    let mut sum = 0;
    for l in left {
        sum += l * counts.get(&l).unwrap_or(&0)
    }
    return sum;
}

fn occurrences(items: &Vec<i32>) -> HashMap<i32, i32> {
    let mut counts = HashMap::new();
    for i in items {
        counts.insert(*i, *counts.get(&i).unwrap_or(&0) + 1);
    }
    return counts;
}
