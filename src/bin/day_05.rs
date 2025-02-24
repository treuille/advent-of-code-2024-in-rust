#![allow(unused_imports)]
use advent_of_code_2024_in_rust::parse_regex::parse_lines;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    // Parse the input
    let input = include_str!("../../puzzle_inputs/day_05.txt");
    let (ordering, pages) = input.split_once("\n\n").unwrap();
    let ordering_regex = Regex::new(r"(\d+)\|(\d+)").unwrap();

    //println!("ordering:\n{}", ordering);

    let mut less_than: HashMap<u32, HashSet<u32>> = HashMap::new();
    for (a, b) in parse_lines(ordering_regex, ordering) {
        less_than.entry(a).or_default().insert(b);
    }
    //println!("less_than:\n{:?}", less_than);

    let pages: Vec<Vec<u32>> = pages
        .lines()
        .map(|line| line.split(",").flat_map(|num| num.parse()).collect())
        .collect();
    //println!("pages: {:?}", pages);

    let mut sum: u32 = 0;
    let mut sum_2: u32 = 0;
    let sorted = |page: &[u32]| {
        page.iter()
            .tuple_windows()
            .all(|(a, b)| less_than.get(a).unwrap().contains(b))
    };
    for page in pages {
        if sorted(&page) {
            sum += page[page.len() / 2];
        } else {
            sum_2 += page
                .iter()
                .cloned() // Convert Iterator<&u32> to Iterator<u32>
                .permutations(page.len())
                .find_map(|perm| sorted(&perm).then(|| perm[perm.len() / 2]))
                .unwrap();
        }
    }
    println!("sum: {}", sum);
    println!("sum_2: {}", sum_2);

    //// Solve 04a
    //let sol_04a: usize = solve(&input, &PATTERN_1, TARGET_1);
    //let correct_sol_04a: usize = 2543;
    //println!("* 04a *");
    //println!("My solution: {sol_04a}");
    //println!("Correct solution: {correct_sol_04a}");
    //println!("Equal: {:?}\n", sol_04a.cmp(&correct_sol_04a));
    //
    //// Solve 04b
    //let sol_04b: usize = solve(&input, &PATTERN_2, TARGET_2);
    //let correct_sol_04b: usize = 1930;
    //println!("* 04b *");
    //println!("My solution: {sol_04b}");
    //println!("Correct solution: {correct_sol_04b}");
    //println!("Equal: {:?}\n", sol_04b.cmp(&correct_sol_04b));
}

// Solution 1

//let ordering: Vec<(u32, u32)> = parse_lines(ordering_regex, ordering).collect();
//let pages: Vec<HashMap<u32, usize>> = pages
//    .lines()
//    .map(|line| {
//        line.split(",")
//            .enumerate()
//            .map(|(idx, num)| (num.parse::<u32>().unwrap(), idx))
//            .collect()
//    })
//    .collect();
//
//println!("ordering:\n{:?}", ordering);
//println!("pages:");
//pages.iter().for_each(|page| println!("{:?}", page));
//
//#[allow(clippy::filter_map_bool_then)]
//let sum: u32 = pages
//    .iter()
//    .filter_map(|page| {
//        ordering
//            .iter()
//            .all(|(a, b)| {
//                let Some(a_idx) = page.get(a) else {
//                    return true;
//                };
//                let Some(b_idx) = page.get(b) else {
//                    return true;
//                };
//                a_idx < b_idx
//            })
//            .then(|| {
//                let find_idx = page.len() / 2;
//                let (num, _) = page.iter().find(|(_, idx)| **idx == find_idx).unwrap();
//                println!("page: {:?}", page);
//                println!("num: {:?}", num);
//                *num
//            })
//    })
//    .sum();
//
//println!("sum: {}", sum);
//
