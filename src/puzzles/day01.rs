use crate::RunError;

pub fn main(part: u8, data: &str) -> Result<i32, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(&parsed_data),
        2 => part2(&parsed_data),
        _ => Err(RunError::BadPartNum)
    }
}

fn parse_data(data: &str) -> Result<[Vec<i32>; 2], RunError> {
    let lines: Vec<&str> = data[..].split('\n').collect();

    let mut lists: [Vec<i32>; 2] = [vec![], vec![]];

    for line in lines {
        let nums: Vec<&str> = line.split_whitespace().collect();
        lists[0].push(nums[0].parse::<i32>()?);
        lists[1].push(nums[1].parse::<i32>()?);
    }

    Ok(lists)

}

fn part1(values: &[Vec<i32>; 2]) -> Result<i32, RunError> {
    // Sort lists, pair members by rank, determine distance between them.
    // Return sum of those distances.

    todo!();
}

fn part2(values: &[Vec<i32>; 2]) -> Result<i32, RunError> {
    // What's the goal?

    todo!();
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
    const SAMPLE_DATA: [[i32; 6]; 2] = [[3,4,2,1,3,3], [4,3,5,3,9,3]];
    static SAMPLE_GOALS: [i32; 2] = [11, 0];

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
