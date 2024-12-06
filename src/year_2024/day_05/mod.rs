use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::MyResult;

mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

#[derive(Default)]
struct Ordering {
    pages_before: HashSet<Page>,
    pages_after: HashSet<Page>,
}

type PageRules = HashMap<Page, Ordering>;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Ord, PartialOrd)]
struct Page(u32);

struct PrintData {
    rules: PageRules,
    jobs: Vec<PrintJob>,
}

struct PrintJob {
    pages: Vec<Page>,
}

fn parse_input(input: &str) -> MyResult<PrintData> {
    // TODO: find something better
    let parts = input.split("\r\n\r\n").collect_vec();

    if parts.len() != 2 {
        return Err("Invalid input: Expected 2 parts".into());
    }

    let data = PrintData {
        rules: parse_order_rules(parts[0])?,
        jobs: parse_print_jobs(parts[1])?,
    };

    Ok(data)
}

fn parse_print_jobs(input: &str) -> MyResult<Vec<PrintJob>> {
    input
        .lines()
        .map(parse_print_job)
        .collect::<Result<Vec<_>, _>>()
}

fn parse_print_job(line: &str) -> MyResult<PrintJob> {
    let pages = line
        .split(',')
        .map(|p| p.parse::<u32>().map(Page))
        .collect::<Result<Vec<_>, _>>()?;

    let job = PrintJob { pages };
    Ok(job)
}

fn parse_order_rules(input: &str) -> MyResult<PageRules> {
    let mut result: HashMap<Page, Ordering> = HashMap::new();

    for line in input.lines() {
        let pages = line
            .split('|')
            .map(|page| page.parse::<u32>().map(Page))
            .collect::<Result<Vec<_>, _>>()?;

        if pages.len() != 2 {
            return Err("Invalid input: found invalid rule".into());
        }

        result
            .entry(pages[0])
            .or_default()
            .pages_after
            .insert(pages[1]);

        result
            .entry(pages[1])
            .or_default()
            .pages_before
            .insert(pages[0]);
    }

    Ok(result)
}

fn is_job_correct(job: &PrintJob, rules: &PageRules) -> bool {
    for window_size in 1..job.pages.len() {
        for window_start in 0..(job.pages.len() - window_size) {
            let window_end = window_start + window_size;
            let p1 = job.pages[window_start];
            let p2 = job.pages[window_end];
            if !rules[&p1].pages_after.contains(&p2) {
                return false;
            }
        }
    }

    true
}
