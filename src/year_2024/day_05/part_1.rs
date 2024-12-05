use std::{collections::{HashMap, HashSet}, hash::Hash};

use itertools::Itertools;
use linkme::distributed_slice;
use rayon::{iter::{IntoParallelRefIterator, ParallelIterator}, result};

use crate::{MyResult, SolverMetadata, SOLVERS};


struct Ordering {
    pages_before: HashSet<Page>,
    pages_after: HashSet<Page>
}

impl Default for Ordering {
    fn default() -> Self {
        Self { pages_before: Default::default(), pages_after: Default::default() }
    }
}

type PageRules = HashMap<Page, Ordering>;


#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Ord, PartialOrd)]
struct Page(u32);

struct PrintData {
    rules: PageRules,
    jobs: Vec<PrintJob>
}

struct PrintJob {
    pages: Vec<Page>
}

#[distributed_slice(SOLVERS)]
static SOLVER: SolverMetadata<'static> = SolverMetadata {
    year: 2024,
    day: 5,
    part: 1,
    func: solve,
    input: super::INPUT,
};

fn solve(input: &str) -> MyResult<u32> {
    let data = parse_input(input)?;
    
    // let result = solve_by_page_rank(&data)?;
    let result = solve_by_expanding_order_rules(&data);

    Ok(result)
}

fn solve_by_page_rank(data: &PrintData) -> MyResult<u32> {
    let ranks = compute_ranks(&data.rules)?;

    println!("ranks: {:?}", ranks);

    let result = data.jobs
        .iter()
        .filter(|job| is_print_job_correct(job, &ranks))
        .map(|job| job.pages[job.pages.len()/2].0)
        .sum();

    Ok(result)
}

fn solve_by_expanding_order_rules(data: & PrintData) -> u32 {
    let pages = data.rules.keys().collect_vec();
    let all_order_rules = pages
        .par_iter()
        .map(|p|(**p, find_pages_after(**p, &data.rules)))
        .collect::<HashMap<Page, HashSet<Page>>>();
        
    let result = data.jobs
        .iter()
        // .filter(|job| is_correct2(job, &all_order_rules))
        .filter(|job| is_correct3(job, &data.rules))
        .map(|job| job.pages[job.pages.len()/2].0)
        .sum();

    result
}

fn is_correct3(job: &PrintJob, rules: &PageRules) -> bool {
    for window_size in 1..job.pages.len() {
        for window_start in 0..(job.pages.len()-window_size) {
            let window_end = window_start+window_size;
            let p1 = job.pages[window_start];
            let p2 = job.pages[window_end];
            if !rules[&p1].pages_after.contains(&p2) {
                return false;
            }
        }
    }
    return true;
    // job.pages.windows(2)
    //     .all(|w| all_print_rules[&w[0]].contains(&w[1]))
}

fn is_correct2(job: &PrintJob, all_print_rules: &HashMap<Page, HashSet<Page>>) -> bool {
    for window_size in 1..job.pages.len() {
        for window_start in 0..(job.pages.len()-window_size) {
            let window_end = window_start+window_size;
            let p1 = job.pages[window_start];
            let p2 = job.pages[window_end];
            if !all_print_rules[&p1].contains(&p2) {
                return false;
            }
        }
    }
    return true;
    // job.pages.windows(2)
    //     .all(|w| all_print_rules[&w[0]].contains(&w[1]))
}

fn parse_input(input: &str) -> MyResult<PrintData> {
    // TODO: find something better
    let parts = input.split("\r\n\r\n").collect_vec();

    if parts.len() != 2 {
        return Err("Invalid input: Expected 2 parts".into());
    }

    let data = PrintData {
        rules: parse_order_rules(parts[0])?,
        jobs: parse_print_jobs(parts[1])?
    };
    
    Ok(data)
}

fn parse_print_jobs(input: &str) -> MyResult<Vec<PrintJob>> {
    input.lines()
        .map(parse_print_job)
        .collect::<Result<Vec<_>, _>>()
}

fn parse_print_job(line: &str) -> MyResult<PrintJob> {
    let pages = line
        .split(',')
        .map(|p| p.parse::<u32>().map(|page| Page(page)))
        .collect::<Result<Vec<_>, _>>()?;

    let job = PrintJob { pages };
    Ok(job)
}

fn parse_order_rules(input: &str) -> MyResult<PageRules> {
    let mut result: HashMap<Page, Ordering> = HashMap::new();
    
    for line in input.lines() {
        let pages = line.split('|')
            .map(|page| page.parse::<u32>().map(|p| Page(p)))
            .collect::<Result<Vec<_>,_>>()?;

        if pages.len() != 2 {
            return Err("Invalid input: found invalid rule".into());
        }

        result
            .entry(pages[0])
            .or_default()
            .pages_after.insert(pages[1]);

        result.entry(pages[1])
            .or_default()
            .pages_before.insert(pages[0]);

    }

    Ok(result)
}

fn find_pages_after(page: Page, rules: &PageRules) -> HashSet<Page> {
    let mut pages = rules.get(&page).unwrap().pages_after.clone();

    let mut current_iteration = pages.clone();
    loop {
        let mut next_iteration = HashSet::new();
        for p in current_iteration {
            let pages_to_add = rules.get(&p).unwrap().pages_after.difference(&pages).cloned().collect_vec();
            // let pages_for_next_iteration = pages_to_add.iter()
            //     .flat_map(|p| rules.get(p).unwrap().pages_after.difference(&pages).clone())
            //     .collect::<HashSet<_>>();

            next_iteration.extend(pages_to_add.clone());
        }

        pages.extend(next_iteration.clone());

        if next_iteration.is_empty() {
            break;
        }

        current_iteration = next_iteration;
    }

    pages
}

fn ranks_for_job(job: &PrintJob, rules: &PageRules) -> HashMap<Page, usize> {
    let mut ranks = HashMap::new();
    for page in rules.keys() {
        ranks.insert(*page, 1);
    }


    ranks
}

fn compute_ranks(rules: &PageRules) -> MyResult<HashMap<Page, usize>> {
    let mut ranks = HashMap::new();
    for page in rules.keys() {
        ranks.insert(*page, 1);
    }
    
    let mut current_iteration = rules
        .iter()
        .filter(|(page, ordering)| ordering.pages_before.is_empty())
        .map(|(p, _)| *p)
        .collect::<HashSet<_>>();

    println!("start iteration: {:?}", current_iteration);
    
    loop {
        let mut next_iteration: HashSet<Page> = HashSet::new();
        
        for page in current_iteration.iter() {
            let highest_rank_in_front = rules
                .get(page)
                .unwrap()
                .pages_before
                    .iter()
                    .map(|p| ranks[p]) 
                    .max();


            if let Some(highest_rank_in_front) = highest_rank_in_front {
                let new_rank = highest_rank_in_front + 1;
                if new_rank > ranks[page] {
                    ranks.entry(*page).and_modify(|rank| {*rank = new_rank});
                }
            };

            next_iteration.extend(rules[page].pages_after.iter());
        }
        
        // println!("next iteration: {:?}",next_iteration);
        println!("current ranks: {:?}", ranks);

        current_iteration = next_iteration;
        
        if current_iteration.is_empty() {
            break;
        }
    }




    Ok(ranks)
}


fn is_print_job_correct(job: &PrintJob, ranks: &HashMap<Page, usize>) -> bool {
    let pages_by_rank = job.pages
        .iter()
        .map(|p| ranks.get(p).unwrap())
        .collect_vec();

    pages_by_rank.windows(2)
        .all(|w| w[0] <= w[1])

}

#[cfg(test)]
mod tests {

    #[test]
    fn solve_exampe() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, 143);
    }

    #[test]
    fn solve_input() {
        let result = super::solve(include_str!("input.txt")).unwrap();
        assert_eq!(result, 143);
    }
}