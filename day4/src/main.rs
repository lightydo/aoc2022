pub static INPUT: &str = include_str!("../input");
pub static TEST_INPUT: &str = include_str!("../input_test");

fn main() {
    println!("part 1 pairs: {}", part_1(INPUT));
    println!("part 2 pairs: {}", part_2(INPUT));
}

pub fn part_1(input: &str) -> i32 {
    let sum = input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(k, v)| (k.split_once('-').unwrap(), v.split_once('-').unwrap()))
        .map(|((a, b), (c, d))| {
            let start = a.parse::<i32>().unwrap();
            let end = b.parse::<i32>().unwrap();
            let range1 = start..=end;
            let start = c.parse::<i32>().unwrap();
            let end = d.parse::<i32>().unwrap();
            let range2 = start..=end;
            (range1, range2)
        })
        .fold(0, |f, (mut range1, mut range2)| {
            if range1.all(|item| range2.contains(&item))
                || range2.all(|item| range1.contains(&item))
            {
                f + 1
            } else {
                f
            }
        });
    sum
}

pub fn part_2(input: &str) -> i32 {
    let sum = input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(k, v)| (k.split_once('-').unwrap(), v.split_once('-').unwrap()))
        .map(|((a, b), (c, d))| {
            let start = a.parse::<i32>().unwrap();
            let end = b.parse::<i32>().unwrap();
            let range1 = start..=end;
            let start = c.parse::<i32>().unwrap();
            let end = d.parse::<i32>().unwrap();
            let range2 = start..=end;
            (range1, range2)
        })
        .fold(0, |f, (mut range1, mut range2)| {
            if range1.any(|item| range2.contains(&item))
                || range2.any(|item| range1.contains(&item))
            {
                f + 1
            } else {
                f
            }
        });
    sum
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(TEST_INPUT), 2);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(TEST_INPUT), 4);
}
