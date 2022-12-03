fn main() {
    part_1();
}

pub fn part_1() {
    /*
        A = X = Rock
        B = Y = Paper
        C = Z = Scissors

    */
    let sum = include_str!("../input")
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
    println!("f: {}", sum);
}
