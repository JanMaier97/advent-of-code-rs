use std::collections::HashSet;

use itertools::Itertools;

use crate::{print_challange_header, MyResult};

const INPUT: &str = include_str!("input.txt");

pub fn solve() -> MyResult<()> {
    print_challange_header(3);

    println!(
        "The sum of priorities of items found in both compartments is {}",
        calculate_priority_sum(INPUT)?,
    );
    println!(
        "The sum of priorities for each batch group is {}",
        calculate_priority_sum_for_badges(INPUT)?,
    );

    Ok(())
}

fn calculate_priority_sum(input: &str) -> MyResult<u32> {
    let mut priority_sum = 0;
    for line in input.lines() {
        // line only contains a-zA-Z so byte offset is correct
        let (first_compartment, second_compartment) = line.split_at(line.len() / 2);
        let mut first_compartment = first_compartment.chars().collect::<Vec<_>>();

        first_compartment.sort();
        first_compartment.dedup();

        let summed_priorities = first_compartment
            .into_iter()
            .filter(|&c1| second_compartment.contains(|c2| c2 == c1))
            .map(get_priority_for_item)
            .sum::<u32>();

        priority_sum += summed_priorities;
    }

    Ok(priority_sum)
}

fn calculate_priority_sum_for_badges(input: &str) -> MyResult<u32> {
    let mut priority_sum = 0;

    for chunk in &input.lines().chunks(3) {

        let sets = chunk.map(|line| line.chars().collect::<HashSet<_>>()).collect_vec();

        let union = sets.into_iter().reduce(|acc, e| acc.intersection(&e).cloned().collect::<HashSet<_>>()).unwrap();

        println!("Items for current chunk");
       priority_sum += union
           .iter()
           .inspect(|c| print!("{}", c))
           .map(|c| get_priority_for_item(*c))
           .sum::<u32>();
           println!();


        // let chunk = chunk.collect::<Result<Vec<String>, _>>()?;
        // let mut all_items = Vec::new();
        //
        // for line in chunk.iter() {
        //     all_items.extend(line.chars())
        // }
        //
        // all_items.sort();
        // all_items.dedup();
        //
        // if chunk.len() != 3 {
        //     return Err("Found group that does not have 3 people".into());
        // }
        //
        // priority_sum += all_items
        //     .into_iter()
        //     .filter(|&item| chunk[0].contains(|item0| item0 == item))
        //     .filter(|&item| chunk[1].contains(|item1| item1 == item))
        //     .filter(|&item| chunk[2].contains(|item2| item2 == item))
        //     .map(get_priority_for_item)
        //     .sum::<u32>();
    }

    Ok(priority_sum)
}

fn get_priority_for_item(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        item as u32 - 'a' as u32 + 1
    } else {
        item as u32 - 'A' as u32 + 27
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn calculate_priority_sum_example() {
        let result = calculate_priority_sum(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 157);
    }

    #[test]
    fn calculate_priority_sum_for_badge_example() {
        let result = calculate_priority_sum_for_badges(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 70);
    }

    #[test]
    fn calculate_priority_sum_for_badge_solution() {
        let result = calculate_priority_sum_for_badges(INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2413);
    }

    #[test]
    fn calculate_priority_sum_solution() {
        let result = calculate_priority_sum(INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 8394);
    }
}
