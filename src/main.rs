use std::env;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut args = env::args().skip(1);
    let day: usize = args
        .next()
        .expect("No day provided")
        .parse::<usize>()
        .expect("Could not read day") - 1;
    let part: usize = args
        .next()
        .expect("No part provided")
        .parse::<usize>()
        .expect("Could not parse part") - 1;
    println!("{}", aoc24::DAYS[day][part](&input));
    Ok(())
}
