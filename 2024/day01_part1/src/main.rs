use std::io;

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
                    .unwrap_or_else(|_| panic!("Can't parse number {x}"))
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
    list1.sort();
    list2.sort();

    let total_distance = list1
        .iter()
        .zip(list2.iter())
        .map(|x| x.0.abs_diff(*x.1))
        .sum::<usize>();

    println!("{total_distance}")
}
