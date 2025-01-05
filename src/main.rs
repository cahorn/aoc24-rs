use aoc24::day01;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("{}", day01::part1(&input));
    Ok(())
}
