use crate::RunError;

pub fn main(part: u8, data: &str) -> Result<usize, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(&parsed_data),
        2 => part2(&parsed_data),
        _ => Err(RunError::BadPartNum)
    }
}

fn parse_data(data: &str) -> Result<Vec<Vec<i32>>, RunError> {
    let lines: Vec<&str> = data[..].split('\n').collect();

    let mut reports: Vec<Vec<i32>> = vec![];

    for line in lines {
        let nums: Vec<&str> = line.split(' ').collect();
        let mut report: Vec<i32> = vec![];

        for num in nums {
            report.push(num.parse::<i32>()?);
        }
        reports.push(report);
    }

    Ok(reports)
}

fn part1(reports: &Vec<Vec<i32>>) -> Result<usize, RunError> {
    // Determine whether reports (lines) are gradually increasing or decreasing.
    // Changes in direction or gaps greater than 3 are unsafe.
    // Return count of safe reports.

    let mut safe_count = 0;

    for report in reports {
        let increasing = report[1] > report[0];
        let mut safe = true;

        for i in 0..report.len() - 1 {
            if increasing && report[i + 1] < report[i] ||
                !increasing && report[i + 1] > report[i] {
                safe = false;
                break;
            }

            let diff = (report[i + 1] - report[i]).abs();
            if diff < 1 || diff > 3 {
                safe = false;
                break;
            }
        }

        if !safe {
            continue;
        }

        safe_count += 1;
    }

    Ok(safe_count)
}

fn part2(values: &Vec<Vec<i32>>) -> Result<usize, RunError> {
    // What's the goal?

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str ="7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    fn sample_data() -> Vec<Vec<i32>> {
        vec![
            [7,6,4,2,1].to_vec(),
            [1,2,7,8,9].to_vec(),
            [9,7,6,2,1].to_vec(),
            [1,3,2,4,5].to_vec(),
            [8,6,4,4,1].to_vec(),
            [1,3,6,7,9].to_vec()
            ]
    }
    static SAMPLE_GOALS: [usize; 2] = [2, 0];

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_data(&SAMPLE_INPUT).unwrap(),
            sample_data());
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&sample_data()).unwrap(),
            SAMPLE_GOALS[0]);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(
    //         part2(&SAMPLE_DATA).unwrap(),
    //         SAMPLE_GOALS[1]);
   // }
}
