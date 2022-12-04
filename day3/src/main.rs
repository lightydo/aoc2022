pub static INPUT: &str = include_str!("../input");
pub static TEST_INPUT: &str = include_str!("../input_test");

use std::collections::HashSet;

fn main() {
    println!("part 1 sum of priorities: {}", part_1(INPUT));
    println!("part 2 sum: {}", part_2(INPUT));
}

pub fn part_1(input: &str) -> i32 {
    let mut sum: i32 = 0;
    let lines = input.lines();
    for line in lines {
        let mut set_right = HashSet::new();
        let mut set_left = HashSet::new();
        let size = line.len();
        for (index, char) in line.chars().enumerate() {
            if index >= size / 2 {
                set_left.insert(char);
            } else {
                set_right.insert(char);
            }
        }

        let intersection = set_right.intersection(&set_left);
        for char in intersection {
            if char.is_ascii_lowercase() {
                sum += (*char as u8 - b'a' + 1) as i32;
            } else {
                sum += (*char as u8 - b'A' + 27) as i32;
            }
        }
    }
    sum
}

pub fn part_2(input: &str) -> i32 {
    let mut sum: i32 = 0;
    let lines = input.lines().collect::<Vec<_>>();
    for group in lines.chunks_exact(3) {
        let set0 = group[0].chars().collect::<HashSet<char>>();
        let set1 = group[1].chars().collect::<HashSet<char>>();
        let set2 = group[2].chars().collect::<HashSet<char>>();

        let inter_sect = set0.intersection(&set1);
        let temp = inter_sect.copied().collect::<HashSet<_>>();
        let inter_sect = temp.intersection(&set2);

        for char in inter_sect.copied() {
            if char.is_ascii_lowercase() {
                sum += (char as u8 - b'a' + 1) as i32;
            } else {
                sum += (char as u8 - b'A' + 27) as i32;
            }
        }
    }
    sum

    // let all_lines = input.lines().collect::<Vec<_>>();
    // all_lines
    //     .chunks_exact(3)
    //     .map(|group| {
    //         let second_set = group[1].chars().collect::<HashSet<char>>();
    //         let third_set = group[2].chars().collect::<HashSet<char>>();
    //         group[0]
    //             .chars()
    //             .find(|c| second_set.contains(c) && third_set.contains(c))
    //             .unwrap()
    //     })
    //     .map(|c| match c {
    //         'a'..='z' => c as i32 - 'a' as i32 + 1,
    //         'A'..='Z' => c as i32 - 'A' as i32 + 27,
    //         _ => {
    //             panic!("Out of range");
    //         }
    //     })
    //     .sum::<i32>()
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(TEST_INPUT), 157);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(TEST_INPUT), 70);
}
