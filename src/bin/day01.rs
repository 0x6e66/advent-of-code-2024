use advent_of_code_2024::INPUT_PATH;

fn main() {
    let lines = std::fs::read_to_string(INPUT_PATH).unwrap();

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = lines
        .lines()
        .filter_map(|line| line.split_once("   "))
        .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
        .collect();

    left.sort();
    right.sort();

    part1(&left, &right);
    part2(&left, &right);
}

fn part1(left: &[i32], right: &[i32]) {
    let sum: u32 = left.iter().zip(right).map(|(l, r)| l.abs_diff(*r)).sum();

    println!("Part 1: {}", sum);
}

fn part2(left: &[i32], right: &[i32]) {
    let sum: i32 = left
        .iter()
        .map(|l| right.iter().filter(|x| *x == l).count() as i32 * l)
        .sum();

    println!("Part 2: {}", sum);
}
