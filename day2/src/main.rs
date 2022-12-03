pub static INPUT: &str = include_str!("../input");
pub static TEST_INPUT: &str = include_str!("../input_test");

fn main() {
    part_1(INPUT);
    part_2(INPUT);
}

pub fn part_1(input: &str) -> i32 {
    /*
        A = X = Rock
        B = Y = Paper
        C = Z = Scissors

    */
    let sum = input
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold(0, |f, (k, v)| match (k, v) {
            ("A", "X") => f + 1 + 3,
            ("A", "Y") => f + 2 + 6,
            ("A", "Z") => f + 3 + 0,
            ("B", "X") => f + 1 + 0,
            ("B", "Y") => f + 2 + 3,
            ("B", "Z") => f + 3 + 6,
            ("C", "X") => f + 1 + 6,
            ("C", "Y") => f + 2 + 0,
            ("C", "Z") => f + 3 + 3,
            _ => unreachable!(),
        });
    println!("part 1 sum: {}", sum);
    sum
}

pub fn part_2(input: &str) -> i32 {
    /*
        A = Rock
        B = Paper
        C = Scissors

        X = Lose
        Y = Draw
        Z = Win


    */
    let sum = input
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold(0, |f, (k, v)| match (k, v) {
            ("A", "X") => f + 3 + 0,
            ("A", "Y") => f + 1 + 3,
            ("A", "Z") => f + 2 + 6,
            ("B", "X") => f + 1 + 0,
            ("B", "Y") => f + 2 + 3,
            ("B", "Z") => f + 3 + 6,
            ("C", "X") => f + 2 + 0,
            ("C", "Y") => f + 3 + 3,
            ("C", "Z") => f + 1 + 6,
            _ => unreachable!(),
        });
    println!("part 2 sum: {}", sum);
    sum
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(TEST_INPUT), 15);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(TEST_INPUT), 12);
}
