use linkme::distributed_slice;

use crate::{MyResult, SolverMetadata, SOLVERS};

struct Record {
    levels: Vec<u32>
}

#[derive(PartialEq)]
enum Safety {
    Save,
    Unsave
}

#[distributed_slice(SOLVERS)]
static SOLVER: SolverMetadata<'static> = SolverMetadata {year: 2024, day: 2, part: 1, func: solve, input: include_str!("input.txt") };

fn solve(input: &str) -> MyResult<u32> {
    let records = parse_input(input)?;
    
    let count = records
        .into_iter()
        .map(|r| determine_record_safety(r))
        .filter(|safety| *safety == Safety::Save)
        .count();

    Ok(count as u32)
}

fn parse_input(input: &str) -> MyResult<Vec<Record>> {
    let mut records = Vec::new();
    for line in input.lines() {
        let levels = line.split(" ")
            .map(|n| n.parse::<u32>())
            .collect::<Result<Vec<_>,_>>()?;
            
        records.push(Record { levels });
    }

    Ok(records)
}

fn determine_record_safety(record: Record) -> Safety {
    let is_increasing = record.levels[0] < record.levels[1];
    for window in record.levels.windows(2) {
        let x = window[0];
        let y = window[1];

        let difference = x.abs_diff(y);
        if difference < 1 || difference > 3 {
            return Safety::Unsave;
        }

        if (is_increasing && x >= y) || (!is_increasing && x <= y) {
            return Safety::Unsave
        } 
    }

    return Safety::Save
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn solve_example_part_1() {
        let result = super::solve(EXAMPLE).unwrap();
        assert_eq!(result, 2);
    }
}