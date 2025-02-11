pub mod day01;
pub mod day02;

pub const DAYS: [[fn(&str) -> i32; 2]; 2] =
    [[day01::part1, day01::part2], [day02::part1, day02::part2]];
