use aoc24::day01;

const DAY01_INPUT: &str = include_str!("day01.txt");

#[test]
fn day01_part1() {
    assert_eq!(day01::part1(&DAY01_INPUT), 11)
}

#[test]
fn day01_part2() {
    assert_eq!(day01::part2(&DAY01_INPUT), 31)
}
