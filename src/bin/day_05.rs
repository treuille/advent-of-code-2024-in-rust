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

    // Parse the ordering into the is_less_than function
    let mut less_than: HashMap<u32, HashSet<u32>> = HashMap::new();
    for (a, b) in parse_lines(ordering_regex.clone(), ordering) {
        less_than.entry(a).or_default().insert(b);
    }
    let is_less_than = |a: &u32, b: &u32| less_than.get(a).unwrap().contains(b);

    // Parse the pages
    let pages: Vec<Vec<u32>> = pages
        .lines()
        .map(|line| line.split(",").flat_map(|num| num.parse()).collect())
        .collect();

    // Solve the puzzles
    let mut sol_05a: u32 = 0;
    let mut sol_05b: u32 = 0;
    let sorted = |page: &[u32]| page.iter().tuple_windows().all(|(a, b)| is_less_than(a, b));
    for page in pages {
        if sorted(&page) {
            sol_05a += page[page.len() / 2];
        } else {
            let sorted_page = sort(&page, is_less_than);
            assert!(sorted(&sorted_page));
            sol_05b += sorted_page[sorted_page.len() / 2];
        }
    }

    // Solve 05a
    let correct_sol_05a: u32 = 6505;
    println!("* 05a *");
    println!("My solution: {sol_05a}");
    println!("Correct solution: {correct_sol_05a}");
    println!("Equal: {:?}\n", sol_05a.cmp(&correct_sol_05a));

    // Solve 05b
    let correct_sol_05b: u32 = 6897;
    println!("* 05b *");
    println!("My solution: {sol_05b}");
    println!("Correct solution: {correct_sol_05b}");
    println!("Equal: {:?}\n", sol_05b.cmp(&correct_sol_05b));
}

fn sort(page: &[u32], is_less_than: impl Fn(&u32, &u32) -> bool + Copy) -> Vec<u32> {
    if page.len() <= 1 {
        return page.to_vec();
    }
    let split_idx = page.len() / 2;
    let (left, right) = page.split_at(split_idx);
    let left = sort(left, is_less_than);
    let right = sort(right, is_less_than);
    left.into_iter().merge_by(right, is_less_than).collect()
}
