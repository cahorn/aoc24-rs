use paste::paste;

macro_rules! tests {
    ($($day:ident: $input:expr, $part1:expr, $part2:expr),*) => {
        $(
            use aoc24::$day;

            paste! {
                const [<$day:upper _INPUT>]: &str = include_str!($input);

                #[test]
                fn [<$day _part1>]() {
                    assert_eq!($day::part1([<$day:upper _INPUT>]), $part1)
                }

                #[test]
                fn [<$day _part2>]() {
                    assert_eq!($day::part2([<$day:upper _INPUT>]), $part2)
                }
            }

        )*
    }
}

tests! {
    day01: "day01.txt", 11, 31,
    day02: "day02.txt", 2, 4,
    day03: "day03.txt", 161, 48
}
