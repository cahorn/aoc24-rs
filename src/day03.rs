use regex::Regex;

enum Cmd {
    Mul(i32, i32),
    Do,
    DoNot,
}

fn parse(input: &str) -> Vec<Cmd> {
    let re = Regex::new(r"((?<op>mul|do|don't)\(((?<arg1>\d+),(?<arg2>\d+))?\))").unwrap();
    let mut cmds = Vec::new();
    for caps in re.captures_iter(input) {
        let err = "Failed to parse i32";
        cmds.push(match &caps["op"] {
            "mul" => Cmd::Mul(
                caps["arg1"].parse().expect(err),
                caps["arg2"].parse().expect(err),
            ),
            "do" => Cmd::Do,
            "don't" => Cmd::DoNot,
            _ => panic!("Invalid cmd"),
        })
    }
    return cmds;
}

pub fn part1(input: &str) -> i32 {
    let cmds = parse(input);
    let mut sum = 0;
    for cmd in cmds {
        match cmd {
            Cmd::Mul(arg1, arg2) => sum += arg1 * arg2,
            Cmd::Do => (),
            Cmd::DoNot => (),
        }
    }
    return sum;
}

pub fn part2(input: &str) -> i32 {
    let cmds = parse(input);
    let mut sum = 0;
    let mut doing = true;
    for cmd in cmds {
        match cmd {
            Cmd::Mul(arg1, arg2) => {
                if doing {
                    sum += arg1 * arg2
                }
            }
            Cmd::Do => doing = true,
            Cmd::DoNot => doing = false,
        }
    }
    return sum;
}
