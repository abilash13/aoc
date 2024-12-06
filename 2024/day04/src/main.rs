fn main() {
    // let input = include_str!("../test.txt");
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}

const ALL_DIRECTIONS: [Action; 8] = [
    Action::GoTop,
    Action::GoTopRight,
    Action::GoRight,
    Action::GoBottomRight,
    Action::GoBottom,
    Action::GoBottomLeft,
    Action::GoLeft,
    Action::GoTopLeft,
];

fn part1(input: &str) {
    let word_matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut count = 0;
    word_matrix.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, letter)| {
            if letter.to_owned() == 'X' {
                count += ALL_DIRECTIONS
                    .iter()
                    .filter(|action| is_xmas_1(&word_matrix, (' ', i, j), &action))
                    .count();
            }
        })
    });

    println!("{count}");
}

fn part2(input: &str) {
    let word_matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut count = 0;
    word_matrix.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, letter)| {
            if letter.to_owned() == 'M' || letter.to_owned() == 'S' {
                if is_xmas_2(&word_matrix, (' ', ' ', i, j), &Action::Start) {
                    count += 1;
                }
            }
        })
    });

    println!("{count}");
}

#[derive(Debug)]
enum Action {
    Start,
    GoTop,
    GoTopRight,
    GoRight,
    GoBottomRight,
    GoBottom,
    GoBottomLeft,
    GoLeft,
    GoTopLeft,
    Stop(bool),
}

fn is_xmas_1(word_matrix: &Vec<Vec<char>>, state: (char, usize, usize), action: &Action) -> bool {
    let (prev_letter, i, j) = state;
    let curr_letter = word_matrix[i][j];
    let next_action = match (prev_letter, curr_letter) {
        (' ', 'X') => action,
        ('X', 'M') => action,
        ('M', 'A') => action,
        ('A', 'S') => &Action::Stop(true),
        _ => &Action::Stop(false),
    };

    match next_action {
        Action::Stop(is_found) => *is_found,
        _ => {
            let (old_i, old_j) = (i as i32, j as i32);
            let (new_i, new_j) = match next_action {
                Action::GoTop => (old_i - 1, old_j),
                Action::GoTopRight => (old_i - 1, old_j + 1),
                Action::GoRight => (old_i, old_j + 1),
                Action::GoBottomRight => (old_i + 1, old_j + 1),
                Action::GoBottom => (old_i + 1, old_j),
                Action::GoBottomLeft => (old_i + 1, old_j - 1),
                Action::GoLeft => (old_i, old_j - 1),
                Action::GoTopLeft => (old_i - 1, old_j - 1),
                _ => panic!("Unexpected action {action:?} while getting new_i and new_j"),
            };

            if new_i >= 0
                && new_i < word_matrix.len() as i32
                && new_j >= 0
                && new_j < word_matrix[i].len() as i32
            {
                is_xmas_1(
                    word_matrix,
                    (curr_letter, new_i as usize, new_j as usize),
                    &next_action,
                )
            } else {
                false
            }
        }
    }
}

fn is_xmas_2(
    word_matrix: &Vec<Vec<char>>,
    state: (char, char, usize, usize),
    action: &Action,
) -> bool {
    let (prev_prev_letter, prev_letter, i, j) = state;
    let curr_letter = word_matrix[i][j];
    let next_action = match (prev_prev_letter, prev_letter, action, curr_letter) {
        (' ', ' ', Action::Start, 'M') => Action::GoBottomRight,
        (' ', ' ', Action::Start, 'S') => Action::GoBottomRight,
        (' ', 'M', Action::GoBottomRight, 'A') => Action::GoBottomRight,
        (' ', 'S', Action::GoBottomRight, 'A') => Action::GoBottomRight,
        ('M', 'A', Action::GoBottomRight, 'S') => Action::GoTopLeft,
        ('S', 'A', Action::GoBottomRight, 'M') => Action::GoTopLeft,
        ('A', 'S', Action::GoTopLeft, 'A') => Action::GoTopRight,
        ('A', 'M', Action::GoTopLeft, 'A') => Action::GoTopRight,
        (_, 'A', Action::GoTopRight, 'M') => Action::GoBottomLeft,
        (_, 'A', Action::GoTopRight, 'S') => Action::GoBottomLeft,
        ('A', 'M', Action::GoBottomLeft, 'A') => Action::GoBottomLeft,
        ('A', 'S', Action::GoBottomLeft, 'A') => Action::GoBottomLeft,
        ('M', 'A', Action::GoBottomLeft, 'S') => Action::Stop(true),
        ('S', 'A', Action::GoBottomLeft, 'M') => Action::Stop(true),
        _ => Action::Stop(false),
    };

    match next_action {
        Action::Stop(is_found) => is_found,
        _ => {
            let (old_i, old_j) = (i as i32, j as i32);
            let (new_i, new_j) = match next_action {
                Action::GoTop => (old_i - 1, old_j),
                Action::GoTopRight => (old_i - 1, old_j + 1),
                Action::GoRight => (old_i, old_j + 1),
                Action::GoBottomRight => (old_i + 1, old_j + 1),
                Action::GoBottom => (old_i + 1, old_j),
                Action::GoBottomLeft => (old_i + 1, old_j - 1),
                Action::GoLeft => (old_i, old_j - 1),
                Action::GoTopLeft => (old_i - 1, old_j - 1),
                _ => panic!("Unexpected action {action:?} while getting new_i and new_j"),
            };

            if new_i >= 0
                && new_i < word_matrix.len() as i32
                && new_j >= 0
                && new_j < word_matrix[i].len() as i32
            {
                is_xmas_2(
                    word_matrix,
                    (prev_letter, curr_letter, new_i as usize, new_j as usize),
                    &next_action,
                )
            } else {
                false
            }
        }
    }
}
