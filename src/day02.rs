fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut reports = Vec::new();
    for line in input.lines() {
        let mut report = Vec::new();
        for word in line.split_whitespace() {
            report.push(word.parse().expect("Could not parse number"))
        }
        reports.push(report)
    }
    return reports;
}

pub fn part1(input: &str) -> i32 {
    let reports = parse(input);
    return reports
        .iter()
        .filter(|r| safe(r))
        .count()
        .try_into()
        .unwrap();
}

pub fn part2(input: &str) -> i32 {
    let reports = parse(input);
    let mut count = 0;
    for report in reports {
        for i in 0..report.len() {
            let mut new = report.clone();
            new.remove(i);
            if safe(&new) {
                count += 1;
                break
            }
        }
    }
    return count
}

fn safe(report: &Vec<i32>) -> bool {
    let mut prev: &i32 = &report[0];
    let mut sign = 0;
    for level in &report[1..] {
        let diff = level - prev;
        if (sign != 0 && sign != diff.signum()) || diff.abs() == 0 || 3 < diff.abs() {
            return false;
        }
        prev = level;
        sign = diff.signum()
    }
    return true;
}
