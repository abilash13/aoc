use regex::Regex;

fn main() {
    // let input = include_str!("../test.txt");
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}

type Number = i32;

fn part1(input: &str) {
    let re = Regex::new(r"mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\)").unwrap();

    let sum: Number = re
        .captures_iter(input)
        .map(|cap| {
            (
                cap.name("num1")
                    .unwrap()
                    .as_str()
                    .parse::<Number>()
                    .unwrap(),
                cap.name("num2")
                    .unwrap()
                    .as_str()
                    .parse::<Number>()
                    .unwrap(),
            )
        })
        .map(|num| num.0 * num.1)
        .sum();
    println!("{sum}");
}

fn part2(input: &str) {
    let re = Regex::new(r"(?:mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\))|(?<cmd>don't|do)").unwrap();

    let mut is_mul_enabled = true;
    let sum: Number = re
        .captures_iter(input)
        .filter_map(|cap| {
            if cap.name("cmd").is_some() {
                match cap.name("cmd").unwrap().as_str() {
                    "don't" => is_mul_enabled = false,
                    _ => is_mul_enabled = true,
                }
                None
            } else {
                if is_mul_enabled {
                    Some((
                        cap.name("num1")
                            .unwrap()
                            .as_str()
                            .parse::<Number>()
                            .unwrap(),
                        cap.name("num2")
                            .unwrap()
                            .as_str()
                            .parse::<Number>()
                            .unwrap(),
                    ))
                } else {
                    None
                }
            }
        })
        .map(|num| num.0 * num.1)
        .sum();
    println!("{sum}");
}
