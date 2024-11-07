use std::{collections::HashMap, fs};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum State<'n> {
    Accept,
    Reject,
    Other(&'n str),
}

/// Conditions for values of `Part`. `GtX(1)` means "x is greater than 1"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    GtX(u16),
    LtX(u16),
    GtA(u16),
    LtA(u16),
    GtS(u16),
    LtS(u16),
    GtM(u16),
    LtM(u16),
    True,
}

impl Condition {
    pub fn matches(self, p: Part) -> bool {
        match self {
            Condition::GtX(v) => p.x > v,
            Condition::LtX(v) => p.x < v,
            Condition::GtA(v) => p.a > v,
            Condition::LtA(v) => p.a < v,
            Condition::GtS(v) => p.s > v,
            Condition::LtS(v) => p.s < v,
            Condition::GtM(v) => p.m > v,
            Condition::LtM(v) => p.m < v,
            Condition::True => true,
        }
    }
}

#[derive(Debug, Clone)]
struct Rule<'s> {
    cond: Condition,
    next: State<'s>,
}

#[derive(Debug, Clone, Default)]
struct Workflow<'s> {
    name: &'s str,
    rules: Vec<Rule<'s>>,
}

#[derive(Debug, Clone, Default, Copy)]
struct Part {
    x: u16,
    a: u16,
    m: u16,
    s: u16,
}

fn parse_line(s: &str) -> Result<Workflow, Part> {
    let (rule_name, content) = s.split_once('{').unwrap();
    let (content, _) = content.split_once('}').unwrap();

    if rule_name.is_empty() {
        Err(parse_part(content))
    } else {
        Ok(parse_workflow(rule_name, content))
    }
}

fn parse_part(c: &str) -> Part {
    let set_kv = |mut part: Part, kv: &str| {
        let (k, v) = kv.split_once('=').unwrap();
        let v = v.parse().unwrap();
        match k {
            "x" => part.x = v,
            "a" => part.a = v,
            "m" => part.m = v,
            "s" => part.s = v,
            _ => unreachable!(),
        }
        part
    };
    c.split(',').fold(Part::default(), set_kv)
}

fn parse_rule<'s>(rule: &'s str) -> Rule<'s> {
    use Condition::*;
    match rule.split_once(':') {
        Some((cond, next)) => {
            let next = match next {
                "A" => State::Accept,
                "R" => State::Reject,
                o => State::Other(o),
            };

            let is_gt = cond.contains('>');
            let (n, v) = cond.split_once(['>', '<']).unwrap();
            let v = v.parse().unwrap();

            let cond = match (is_gt, n) {
                (true, "a") => GtA(v),
                (true, "s") => GtS(v),
                (true, "m") => GtM(v),
                (true, "x") => GtX(v),
                (false, "a") => LtA(v),
                (false, "s") => LtS(v),
                (false, "m") => LtM(v),
                (false, "x") => LtX(v),
                _ => unreachable!("impossible condition obtained"),
            };
            Rule { next, cond }
        }
        None => {
            let next = match rule {
                "A" => State::Accept,
                "R" => State::Reject,
                o => State::Other(o),
            };
            Rule {
                next,
                cond: Condition::True,
            }
        }
    }
}

fn parse_workflow<'s>(name: &'s str, rules: &'s str) -> Workflow<'s> {
    let rules = rules.split(',').map(parse_rule).collect();

    Workflow { name, rules }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("inputs/p19/input.txt")?;
    let (workflows, parts): (HashMap<_, _>, Vec<_>) = content
        .lines()
        .filter(|s| !s.is_empty())
        .map(parse_line)
        .map(|v| v.map(|w| (w.name, w)))
        .partition_result();

    let mut part1 = 0u64;

    for p in parts {
        let mut w = workflows
            .get("in")
            .expect("\"in\" workflow must be present in input"); // has to be there
        'part: loop {
            for rule in &w.rules {
                if rule.cond.matches(p) {
                    match rule.next {
                        State::Accept => part1 += p.a as u64 + p.x as u64 + p.s as u64 + p.m as u64,
                        State::Reject => (),
                        State::Other(next_name) => {
                            w = workflows.get(next_name).unwrap();
                            continue 'part;
                        }
                    }
                    break 'part;
                }
            }
        }
    }

    println!("[PART1] Sum of ratings: {part1}");

    Ok(())
}
