use crate::RunError;

pub fn main(part: u8, data: &str) -> Result<usize, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(&parsed_data),
        2 => part2(&parsed_data),
        _ => Err(RunError::BadPartNum)
    }
}

fn parse_data(data: &str) -> Result<Vec<usize>, RunError> {
    let lines: Vec<&str> = data[..].split('\n').collect();

    todo!();
}

fn part1(values: &[usize]) -> Result<usize, RunError> {
    // What's the goal?

    todo!();
}

fn part2(values: &[usize]) -> Result<usize, RunError> {
    // What's the goal?

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str ="";
    static SAMPLE_DATA: &'static [i32] = &[];
    static SAMPLE_GOALS: [usize; 2] = [0, 0];

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_data(&SAMPLE_INPUT).unwrap(),
            SAMPLE_DATA);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&SAMPLE_DATA).unwrap(),
            SAMPLE_GOALS[0]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&SAMPLE_DATA).unwrap(),
            SAMPLE_GOALS[1]);
    }
}
