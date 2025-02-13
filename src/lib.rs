pub mod day01;
pub mod day02;
pub mod day03;

pub const DAYS: [[fn(&str) -> i32; 2]; 3] = [
    [day01::part1, day01::part2],
    [day02::part1, day02::part2],
    [day03::part1, day03::part2],
];
