use std::{collections::HashMap, io};

fn main() {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                break;
            }
            Ok(_) => (),
            Err(_) => panic!("Failed to read line"),
        }
    }

    let mut list1 = vec![];
    let mut list2 = vec![];
    for line in input.lines() {
        line.splitn(2, ' ')
            .map(|x| {
                x.trim()
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("Failed to parse number {x}"))
            })
            .enumerate()
            .for_each(|x| {
                if x.0 == 0 {
                    list1.push(x.1)
                } else {
                    list2.push(x.1)
                }
            })
    }

    let list2_location_counts = list2
        .iter()
        .fold(HashMap::<usize, usize>::new(), |mut acc, x| {
            *acc.entry(*x).or_insert(0) += 1;
            acc
        });

    let similarity_score = list1.iter().fold(0, |acc, x| {
        acc + x * list2_location_counts.get(x).unwrap_or(&0usize)
    });

    println!("{similarity_score}")
}
