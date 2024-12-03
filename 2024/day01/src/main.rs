use std::collections::HashMap;

fn main() {
    // let input = include_str!("../test.txt");
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
            let mut parts = line.splitn(2, ' ').map(|number| {
                number
                    .trim()
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("Can't parse number {number}"))
            });

            let first = parts.next().expect("Missing first number");
            let second = parts.next().expect("Missing second number");

            (first, second)
        })
        .unzip()
}

fn part1(input: &str) {
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();

    let total_distance: usize = left
        .into_iter()
        .zip(right.into_iter())
        .map(|x| x.0.abs_diff(x.1))
        .sum();

    println!("{total_distance}")
}

fn part2(input: &str) {
    let (left, right) = parse(input);

    let right_counts = right
        .iter()
        .fold(HashMap::<usize, usize>::new(), |mut acc, loc| {
            *acc.entry(*loc).or_insert(0) += 1;
            acc
        });

    let similarity_score = left.iter().fold(0, |acc, loc| {
        acc + loc * right_counts.get(loc).unwrap_or(&0usize)
    });

    println!("{similarity_score}")
}
