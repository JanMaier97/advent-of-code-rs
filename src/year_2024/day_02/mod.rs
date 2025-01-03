use anyhow::Result;

mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone)]
struct Record {
    levels: Vec<u32>,
}

#[derive(PartialEq)]
enum Safety {
    Save,
    Unsave,
}

fn parse_input(input: &str) -> Result<Vec<Record>> {
    let mut records = Vec::new();
    for line in input.lines() {
        let levels = line
            .split(' ')
            .map(|n| n.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;

        records.push(Record { levels });
    }

    Ok(records)
}

fn count_save_records(records: &[Record], safety_strategy: fn(&Record) -> Safety) -> u32 {
    let count = records
        .iter()
        .map(safety_strategy)
        .filter(|safety| *safety == Safety::Save)
        .count();

    count as u32
}

fn parse_and_count(input: &str, safety_strategy: fn(&Record) -> Safety) -> Result<String> {
    let records = parse_input(input)?;
    let count = count_save_records(&records, safety_strategy);

    Ok(count.to_string())
}

fn all_levels_safe(record: &Record) -> Safety {
    let diffs = record
        .levels
        .windows(2)
        .map(|w| w[1] as i32 - w[0] as i32)
        .collect::<Vec<_>>();

    let contains_invalid_difference = diffs
        .iter()
        .map(|level: &i32| level.abs())
        .any(|level| !(1..=3).contains(&level));

    if contains_invalid_difference {
        return Safety::Unsave;
    }

    let contains_direction_change = diffs.windows(2).any(|w| w[0].signum() != w[1].signum());

    if contains_direction_change {
        Safety::Unsave
    } else {
        Safety::Save
    }
}
