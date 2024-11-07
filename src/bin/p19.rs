use std::{collections::HashMap, fs, ops::RangeInclusive};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum State<'n> {
    Accept,
    Reject,
    Other(&'n str),
}

/// By default it's 1..=4000 for each
#[derive(Debug, Clone)]
struct PartRange {
    m: RangeInclusive<u16>,
    s: RangeInclusive<u16>,
    x: RangeInclusive<u16>,
    a: RangeInclusive<u16>,
}

impl Default for PartRange {
    fn default() -> Self {
        // possible value range for each
        let range = 1..=4000;
        Self {
            m: range.clone(),
            s: range.clone(),
            x: range.clone(),
            a: range,
        }
    }
}

impl PartRange {
    const EMPTY: Self = Self {
        m: 1..=0,
        a: 1..=0,
        x: 1..=0,
        s: 1..=0,
    };

    pub fn is_empty(&self) -> bool {
        self.m.is_empty() || self.x.is_empty() || self.a.is_empty() || self.s.is_empty()
    }
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

    /// Applies the condition to the range. Returns the range of values that satisfy the condition
    /// and the range of values that don't
    pub fn split_part_range(self, r: PartRange) -> (PartRange, PartRange) {
        // return gt v and le v
        let split_range_gt = |r: RangeInclusive<u16>, v: u16| ((v + 1)..=*r.end(), *r.start()..=v);
        let split_range_lt = |r: RangeInclusive<u16>, v: u16| (*r.start()..=(v - 1), v..=*r.end());

        match self {
            Condition::GtX(v) => {
                let (xa, xr) = split_range_gt(r.clone().x, v);
                (PartRange { x: xa, ..r.clone() }, PartRange { x: xr, ..r })
            }
            Condition::LtX(v) => {
                let (xa, xr) = split_range_lt(r.clone().x, v);
                (PartRange { x: xa, ..r.clone() }, PartRange { x: xr, ..r })
            }
            Condition::GtA(v) => {
                let (aa, ar) = split_range_gt(r.clone().a, v);
                (PartRange { a: aa, ..r.clone() }, PartRange { a: ar, ..r })
            }
            Condition::LtA(v) => {
                let (aa, ar) = split_range_lt(r.clone().a, v);
                (PartRange { a: aa, ..r.clone() }, PartRange { a: ar, ..r })
            }
            Condition::GtS(v) => {
                let (sa, sr) = split_range_gt(r.clone().s, v);
                (PartRange { s: sa, ..r.clone() }, PartRange { s: sr, ..r })
            }
            Condition::LtS(v) => {
                let (sa, sr) = split_range_lt(r.clone().s, v);
                (PartRange { s: sa, ..r.clone() }, PartRange { s: sr, ..r })
            }
            Condition::GtM(v) => {
                let (ma, mr) = split_range_gt(r.clone().m, v);
                (PartRange { m: ma, ..r.clone() }, PartRange { m: mr, ..r })
            }
            Condition::LtM(v) => {
                let (ma, mr) = split_range_lt(r.clone().m, v);
                (PartRange { m: ma, ..r.clone() }, PartRange { m: mr, ..r })
            }
            Condition::True => (r, PartRange::EMPTY),
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

fn parse_rule(rule: &str) -> Rule {
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
        // Walk the workflow graph (is it a tree?)
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

    // now part2 is pretty huge. We still need to walk the graph, but this time we need to
    // take every possible path to an accepting leaf. The objective is to find how many
    // combinations are accepted in total. We do this by gathering all the possible values when we
    // reach an accept leaf (not sure if a graph or a tree, so I will consider the case where
    // multiple paths end at the same leaf).

    let root = workflows.get("in").unwrap();
    let pr = PartRange::default();
    let arrangements = walk_graph(&workflows, pr, root);

    println!("[PART2] Possible arrangements: {arrangements}");

    Ok(())
}

fn walk_graph(ws: &HashMap<&str, Workflow>, pr: PartRange, w: &Workflow<'_>) -> u64 {
    let arrangements = |pr: &PartRange| {
        (pr.a.end() + 1 - pr.a.start()) as u64
            * (pr.s.end() + 1 - pr.s.start()) as u64
            * (pr.m.end() + 1 - pr.m.start()) as u64
            * (pr.x.end() + 1 - pr.x.start()) as u64
    };

    let mut possible = 0;
    w.rules.iter().try_fold(pr, |acc, rule| {
        let (a, acc) = rule.cond.split_part_range(acc);
        match rule.next {
            State::Accept => {
                possible += arrangements(&a);
            }
            State::Reject => {}
            State::Other(next_name) => {
                let w = ws.get(next_name).unwrap();
                possible += walk_graph(ws, a, w);
            }
        };

        (!acc.is_empty()).then_some(acc)
    });
    possible
}
