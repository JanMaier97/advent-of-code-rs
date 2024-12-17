use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

use anyhow::{anyhow, bail, Result};

mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Copy)]
struct Vec2 {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy)]
struct Machine {
    button_a: Vec2,
    button_b: Vec2,
    price: Vec2,
}

fn solve_for_input(input: &str, offset: i64) -> Result<String> {
    let machines = parse_input(input)?;
    let machines = machines
        .iter()
        .map(|m| apply_offset(*m, offset))
        .collect_vec();
    let sum: u64 = machines.iter().map(|m| compute_required_token(*m)).sum();

    Ok(sum.to_string())
}

fn apply_offset(machine: Machine, offset: i64) -> Machine {
    Machine {
        button_a: machine.button_a,
        button_b: machine.button_b,
        price: Vec2 {
            x: machine.price.x + offset,
            y: machine.price.y + offset,
        },
    }
}

fn parse_input(input: &str) -> Result<Vec<Machine>> {
    input
        .split("\r\n\r\n")
        .map(parse_machine)
        .collect::<Result<Vec<_>, _>>()
}

fn parse_machine(block: &str) -> Result<Machine> {
    let lines = block.lines().collect_vec();
    if lines.len() != 3 {
        bail!("Invalid machine block:\n{}", block);
    }

    let machine = Machine {
        button_a: parse_button(lines[0])?,
        button_b: parse_button(lines[1])?,
        price: parse_price(lines[2])?,
    };

    Ok(machine)
}

fn parse_price(line: &str) -> Result<Vec2> {
    static PRICE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap());
    let capture = PRICE_REGEX
        .captures(line)
        .ok_or(anyhow!("Invalid a button input"))?;
    let price = Vec2 {
        x: capture[1].parse::<i64>()?,
        y: capture[2].parse::<i64>()?,
    };

    Ok(price)
}

fn parse_button(line: &str) -> Result<Vec2> {
    static BUTTON_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap());

    let capture = BUTTON_REGEX
        .captures(line)
        .ok_or(anyhow!("Invalid a button input"))?;
    let btn = Vec2 {
        x: capture[1].parse::<i64>()?,
        y: capture[2].parse::<i64>()?,
    };

    Ok(btn)
}
fn compute_required_token(machine: Machine) -> u64 {
    let (token_a, token_b) = find_tokens(machine);

    if are_tokens_correct(token_a, token_b, machine) {
        return (token_a * 3 + token_b).try_into().unwrap();
    }

    0
}

fn find_tokens(machine: Machine) -> (i64, i64) {
    let token_b = compute_b_tokens(machine);
    let token_a = compute_a_tokens(token_b, machine);

    (token_a, token_b)
}

fn are_tokens_correct(token_a: i64, token_b: i64, machine: Machine) -> bool {
    let p = machine.price;
    let a = machine.button_a;
    let b = machine.button_b;

    p.x == token_a * a.x + token_b * b.x && p.y == token_a * a.y + token_b * b.y
}

fn compute_a_tokens(b_token: i64, machine: Machine) -> i64 {
    (machine.price.y - b_token * machine.button_b.y) / machine.button_a.y
}

fn compute_b_tokens(machine: Machine) -> i64 {
    let price = machine.price;
    let a_btn = machine.button_a;
    let b_btn = machine.button_b;
    let upper = price.x * a_btn.y - price.y * a_btn.x;
    let lower = b_btn.x * a_btn.y - b_btn.y * a_btn.x;

    upper / lower
}
