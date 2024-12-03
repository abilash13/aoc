fn main() {
    // let input = include_str!("../test.txt");
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}

type Number = i32;

fn check_is_level_safe(are_levels_increasing: bool, a: &Number, b: &Number) -> bool {
    let diff = b - a;
    if diff.abs() < 1 || diff.abs() > 3 {
        false
    } else if diff < 0 && are_levels_increasing {
        false
    } else if diff > 0 && !are_levels_increasing {
        false
    } else {
        true
    }
}

fn check_is_report_safe(levels: &Vec<Number>) -> bool {
    let is_increasing = levels[0] < levels[1];
    for (i, _) in levels.iter().enumerate().skip(1) {
        if !check_is_level_safe(is_increasing, &levels[i - 1], &levels[i]) {
            return false;
        }
    }
    true
}

fn part1(input: &str) {
    let mut num_safe_reports = 0;
    input.lines().for_each(|report| {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|level| {
                level
                    .parse()
                    .unwrap_or_else(|_| panic!("Can't parse {level} as a number"))
            })
            .collect();

        if check_is_report_safe(&levels) {
            num_safe_reports += 1;
        }
    });

    println!("{num_safe_reports}")
}

fn part2(input: &str) {
    let mut num_safe_reports = 0;
    input.lines().for_each(|report| {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|level| {
                level
                    .parse()
                    .unwrap_or_else(|_| panic!("Can't parse {level} as a number"))
            })
            .collect();

        let is_report_safe = check_is_report_safe(&levels);
        if is_report_safe {
            num_safe_reports += 1;
        } else {
            for (i, _) in levels.iter().enumerate() {
                if check_is_report_safe(&remove_element_at_index(&levels, i)) {
                    num_safe_reports += 1;
                    break;
                }
            }
        }
    });

    println!("{num_safe_reports}")
}

fn remove_element_at_index(vec: &Vec<Number>, index: usize) -> Vec<Number> {
    let mut result = Vec::with_capacity(vec.len() - 1);

    for i in 0..index {
        result.push(vec[i].clone());
    }

    for i in index + 1..vec.len() {
        result.push(vec[i].clone());
    }

    result
}
