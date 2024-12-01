use std::{collections::HashMap, hash::Hash};

use crate::RunError;

pub fn main(part: u8, data: &str) -> Result<usize, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(&parsed_data),
        2 => part2(&parsed_data),
        _ => Err(RunError::BadPartNum)
    }
}

fn parse_data(data: &str) -> Result<[Vec<usize>; 2], RunError> {
    let lines: Vec<&str> = data[..].split('\n').collect();

    let mut lists: [Vec<usize>; 2] = [vec![], vec![]];

    for line in lines {
        let nums: Vec<&str> = line.split_whitespace().collect();
        lists[0].push(nums[0].parse::<usize>()?);
        lists[1].push(nums[1].parse::<usize>()?);
    }

    Ok(lists)

}

fn part1(values: &[Vec<usize>; 2]) -> Result<usize, RunError> {
    // Sort lists, pair members by rank, determine distance between them.
    // Return sum of those distances.

    let mut lists = values.clone();

    lists[0].sort();
    lists[1].sort();

    let mut diff_sum: usize = 0;

    for i in 0..lists[0].len() {
        diff_sum += (lists[0][i] as i32- lists[1][i] as i32).abs() as usize
    }

    Ok(diff_sum)

}

fn part2(values: &[Vec<usize>; 2]) -> Result<usize, RunError> {
    // Count occurrences of each left list number in the right list.
    // Multiply left list numbers by theat count.
    // Sum those products

    let mut counts: HashMap<usize, usize> = HashMap::new();
    let mut total: usize = 0;

    for &num in &values[0] {
        let value: usize = *counts.entry(num).or_insert(values[1].iter().filter(|&x| *x == num).count());

        total += num * value;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str ="3   4
4   3
2   5
1   3
3   9
3   3";
    const SAMPLE_DATA: [[usize; 6]; 2] = [[3,4,2,1,3,3], [4,3,5,3,9,3]];
    static SAMPLE_GOALS: [usize; 2] = [11, 31];

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_data(&SAMPLE_INPUT).unwrap(),
            SAMPLE_DATA);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&[SAMPLE_DATA[0].to_vec(), SAMPLE_DATA[1].to_vec()]).unwrap(),
            SAMPLE_GOALS[0]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&[SAMPLE_DATA[0].to_vec(), SAMPLE_DATA[1].to_vec()]).unwrap(),
            SAMPLE_GOALS[1]);
    }
}
