Advent of Code 2024
===================

[Advent of Code](https://adventofcode.com/2024) 2024 in
[Rust](https://www.rust-lang.org/)


Dependencies
------------

 * cargo
 * rustc
 * rustfmt (style)


Use
---

To get the answer for a particular day/part solution when applied to the input
in the given file:

    $ cargo run DAY PART < INPUTFILE


Test
----

To test all day/part solutions against the respective example input/answer
pairs:

    $ cargo test


Style
-----

### Format

Apply format with `rustfmt`:

    $ find . -name '*.rs' -exec rustfmt {} \;
