use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut left = input
        .lines()
        .map(|line| line.split_whitespace().next().unwrap())
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    left.sort();
    let mut right = input
        .lines()
        .map(|line| line.split_whitespace().last().unwrap())
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    right.sort();

    let distances = left.into_iter().zip(right).map(|(a, b)| (a - b).abs()).collect::<Vec<_>>();

    Some(distances.iter().sum::<i64>() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut occurances: HashMap<i64, i64> = HashMap::new();
    input.lines()
        .map(|line| line.split_whitespace().last().unwrap())
        .map(|s| s.parse::<i64>().unwrap())
        .for_each(|s| *occurances.entry(s).or_insert(0) += 1);

    let result = input.lines()
        .map(|line| line.split_whitespace().next().unwrap())
        .map(|s| s.parse::<i64>().unwrap())
        .fold(0, |acc, x| acc + x * occurances.get(&x).unwrap_or(&0));
    
    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
