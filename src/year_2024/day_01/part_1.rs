use crate::MyResult;

struct ParsedLocation {
    list1: Vec<u32>,
    list2: Vec<u32>
}

pub fn solve(input: &str) -> MyResult<u32> {
    let mut locations = parse_input(input)?;
    
    locations.list1.sort();
    locations.list2.sort();

    
    let result = locations.list1
        .into_iter()
        .zip(locations.list2)
        .map(|(loc1, loc2)| loc1.abs_diff(loc2))
        .sum() ;

    Ok(result)
}

fn parse_input(input: &str) -> MyResult<ParsedLocation> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        let split = line
            .split("   ")
            .map(|elem| elem.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;

        if split.len() != 2 {
            return Err(format!("Invalid input at line {}", idx + 1).into());
        }

        list1.push(split[0]);
        list2.push(split[1]);
    }
    
    Ok(ParsedLocation {list1, list2})
}

#[cfg(test)]
mod tests {
    use crate::year_2024::day_01::part_1::solve;
    use crate::year_2024::day_01::INPUT;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn test_part_one_example() {
        let result = solve(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 11);
    }

    #[test]
    fn test_part_one_input() {
        let result = solve(INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2057374);
    }
}
