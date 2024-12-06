use std::collections::{HashMap, HashSet};

fn main() {
    // let input = include_str!("../test.txt");
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}

fn parse(input: &str) -> (HashMap<&str, HashSet<&str>>, Vec<Vec<&str>>) {
    let mut page_ordering_rules: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut updates: Vec<Vec<&str>> = vec![];

    let mut is_in_page_ordering_rules = true;
    input.lines().for_each(|line| {
        if is_in_page_ordering_rules {
            if line.trim() == "" {
                is_in_page_ordering_rules = false;
                return;
            }

            let (page1, page2) = line.split_once("|").unwrap_or_else(|| {
                panic!("Unknown line format while parsing page ordering rules: {line}")
            });

            page_ordering_rules.entry(page2).or_default().insert(page1);
        } else {
            updates.push(line.split(',').collect())
        }
    });

    (page_ordering_rules, updates)
}

fn part1(input: &str) {
    let (page_ordering_rules, updates) = parse(input);

    let sum: usize = updates
        .iter()
        .filter_map(|update| {
            let pages: HashSet<&str> = update.iter().map(|x| *x).collect();
            let mut pages_printed: HashSet<&str> = HashSet::new();
            if update.iter().all(|page| {
                pages_printed.insert(page);
                match page_ordering_rules.get(*page) {
                    None => true,
                    Some(prev_pages) => prev_pages.iter().all(|prev_page| {
                        !pages.contains(prev_page) || pages_printed.contains(prev_page)
                    }),
                }
            }) {
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .map(|page| {
            page.parse::<usize>()
                .unwrap_or_else(|_| panic!("Can't parse page {page} as a number"))
        })
        .sum();
    println!("{sum}");
}

fn part2(input: &str) {
    let (page_ordering_rules, updates) = parse(input);

    let sum: usize = updates
        .iter()
        .filter_map(|update| {
            let pages: HashSet<&str> = update.iter().map(|x| *x).collect();
            let mut pages_printed: HashSet<&str> = HashSet::new();
            if !update.iter().all(|page| {
                pages_printed.insert(page);
                match page_ordering_rules.get(*page) {
                    None => true,
                    Some(prev_pages) => prev_pages.iter().all(|prev_page| {
                        !pages.contains(prev_page) || pages_printed.contains(prev_page)
                    }),
                }
            }) {
                let mut corrected_update: Vec<&str> = vec![];
                let mut pages_printed: HashSet<&str> = HashSet::new();
                update.iter().for_each(|page| {
                    correct_update_page(
                        &page_ordering_rules,
                        page,
                        &pages,
                        &mut corrected_update,
                        &mut pages_printed,
                    )
                });

                Some(corrected_update[update.len() / 2])
            } else {
                None
            }
        })
        .map(|page| {
            page.parse::<usize>()
                .unwrap_or_else(|_| panic!("Can't parse page {page} as a number"))
        })
        .sum();
    println!("{sum}");
}

fn correct_update_page<'a>(
    page_ordering_rules: &HashMap<&str, HashSet<&'a str>>,
    page: &'a str,
    pages: &HashSet<&str>,
    corrected_update: &mut Vec<&'a str>,
    pages_printed: &mut HashSet<&'a str>,
) {
    if pages_printed.contains(page) {
        return;
    }

    if let Some(prev_pages) = page_ordering_rules.get(page) {
        prev_pages.iter().for_each(|prev_page| {
            if !pages.contains(prev_page) || pages_printed.contains(prev_page) {
                return;
            }

            correct_update_page(
                page_ordering_rules,
                prev_page,
                pages,
                corrected_update,
                pages_printed,
            );
        });
    }

    corrected_update.push(page);
    pages_printed.insert(page);
}
