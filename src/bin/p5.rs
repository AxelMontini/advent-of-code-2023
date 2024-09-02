use std::{collections::VecDeque, fs};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
number = { ASCII_DIGIT+ }
map_name = { ASCII_ALPHA+ }
seeds = { "seeds:" ~ (" " ~ number)+ }
range = { number ~ " " ~ number ~ " " ~ number }
map = { map_name ~ "-to-" ~ map_name ~ " map:" ~ (NEWLINE ~ range)* }
file = { SOI ~ seeds ~ (NEWLINE ~ NEWLINE ~ map)* ~ NEWLINE* ~ EOI }
"#]
struct AlmanacParser;

/// Mapping range, mapping `src..(src+len)` to elements starting at `dst`
#[derive(Clone, Copy, Eq, PartialEq)]
struct Mapping {
    src: usize,
    dst: usize,
    len: usize,
}

fn main() {
    println!("Problem number 5");
    let content = fs::read_to_string("inputs/p5/almanac.txt").expect("reading input file");
    let file = AlmanacParser::parse(Rule::file, &content)
        .expect("parsing input almanac")
        .next()
        .unwrap();

    // the maps are pretty huge, thus representing them in a dumb way (with index-to-value array
    // maps) is not very smart (lots of mem used).
    // I use my own type instead

    // invariant: for all i,j: maps[i].len() == maps[j].len()
    let mut maps: Vec<Vec<_>> = vec![];

    let mut seeds_and_maps = file.into_inner();

    let seeds: Vec<_> = seeds_and_maps
        .next()
        .expect("getting seeds list")
        .into_inner()
        .map(|seed_num| usize::from_str_radix(seed_num.as_str(), 10).unwrap())
        .collect();

    println!("Registered seeds: {seeds:?}");

    // maps seem to be in-order (previous maps' dest is the next maps' src)
    // thus storing them in ordered vector
    for map in seeds_and_maps {
        match map.as_rule() {
            Rule::map => {
                let mut map_children = map.into_inner();
                let from = map_children.next().expect("getting map source").as_str();
                let to = map_children
                    .next()
                    .expect("getting map destination")
                    .as_str();

                println!("{from}->{to} map:");
                let mut ranges = vec![];
                for range in map_children {
                    let [dst, src, len] = range
                        .into_inner()
                        .map(|n| usize::from_str_radix(n.as_str(), 10).unwrap())
                        .collect::<Vec<_>>()
                        .try_into()
                        .expect("wrong range length");

                    println!("\t{src}->{dst}\t(len {len})");

                    ranges.push(Mapping { src, dst, len })
                }
                maps.push(ranges);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    // part 1: lowest location number corresponding to any initial seed number.
    // Idea: obtain by mapping each seed to the next table, until the last.
    let mut tt = seeds.clone();

    for map in &maps {
        for num in tt.iter_mut() {
            *num = map
                .iter()
                .find(|mapping| (mapping.src..(mapping.src + mapping.len)).contains(num))
                .map(|mapping| *num - mapping.src + mapping.dst)
                .unwrap_or(*num);
        }
    }

    println!(
        "Lowest location number corresponding to a valid seed: {}",
        tt.iter().min().unwrap()
    );

    // part 2. Need to change the meaning of `seeds`. Now it contains pairs of (start, len).
    // I basically do the same thing, but I can't do that for each seed number.
    // Instead, I transform **ranges** by mapping them to the next table thingy.
    //

    let mut ranges: Vec<_> = seeds
        .chunks_exact(2)
        .map(|pair| pair[0]..(pair[0] + pair[1]))
        .collect();

    for map in &maps {
        let mut result = vec![];
        for src_range in ranges.iter_mut() {
            let mut leftover = VecDeque::from_iter(Some(src_range.clone()));
            while let Some(num_range) = leftover.pop_front() {
                let result_len = result.len();
                for mapping in map {
                    // Now map the num range to the next table. Note that we may have to split it in UP TO
                    // 3 COMPONENTS for every mapping! Specifically, the situations might be
                    // 1. [     ]    (    )                         not intersecting at all
                    // 2. (    [     ]) or (       [     )    ]     intersecting, only 2 seed ranges are
                    //    produces
                    // 3. (  [  ]   ) intersecting, 3 seed ranges are produced
                    // Intersect at all?
                    if !(num_range.start < (mapping.src + mapping.len)
                        && num_range.end > mapping.src)
                    {
                        continue; // no intesection
                    }

                    // try to split in 3 parts
                    // prefix of num range with no intersection
                    if num_range.start < mapping.src {
                        leftover.push_back(num_range.start..mapping.src);
                    }

                    // num intersecting mapping
                    let intersection_range = (num_range.start.max(mapping.src))
                        ..(num_range.end.min(mapping.src + mapping.len));
                    result.push(
                        (intersection_range.start + mapping.dst - mapping.src)
                            ..(intersection_range.end + mapping.dst - mapping.src),
                    );

                    // suffix of num range with no intersection
                    if num_range.end > mapping.src + mapping.len {
                        leftover.push_back((mapping.src + mapping.len)..num_range.end);
                    }
                }
                // if no mapping applies to this leftover, map it 1:1 (put it in result)
                if result_len == result.len() {
                    result.push(num_range);
                }
            }
        }
        // now result contains all the new items produces by processing the ranges. Replace `ranges`
        // with it.
        ranges = result;
    }

    // please no duplicates... Otherwise algo is wrong, or mappings overlap (yikes!)
    let min_location = ranges.iter().map(|r| r.start).min().unwrap();

    println!("Min location when using seed range representation: {min_location}");
}
