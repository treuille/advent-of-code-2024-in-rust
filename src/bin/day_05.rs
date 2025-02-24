use advent_of_code_2024_in_rust::parse_regex::parse_lines;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    // Parse the input
    let input = include_str!("../../puzzle_inputs/day_05.txt");
    let (ordering, pages) = input.split_once("\n\n").unwrap();
    let ordering_regex = Regex::new(r"(\d+)\|(\d+)").unwrap();

    // Parse the ordering into the compare(..) function
    let mut less_than: HashMap<u32, HashSet<u32>> = HashMap::new();
    for (a, b) in parse_lines(ordering_regex, ordering) {
        less_than.entry(a).or_default().insert(b);
    }
    let compare = |a: &u32, b: &u32| match (a, b) {
        (a, b) if a == b => std::cmp::Ordering::Equal,
        (a, b) if less_than.get(a).unwrap().contains(b) => std::cmp::Ordering::Less,
        (a, b) if less_than.get(b).unwrap().contains(a) => std::cmp::Ordering::Greater,
        _ => panic!("Incomparable values: {a}, {b}"),
    };

    // Parse the pages
    let pages: Vec<Vec<u32>> = pages
        .lines()
        .map(|line| line.split(",").flat_map(|num| num.parse()).collect())
        .collect();

    // Solve the puzzles
    let (mut sol_05a, mut sol_05b) = (0, 0);
    for mut page in pages {
        if page.is_sorted_by(|a, b| compare(a, b) == std::cmp::Ordering::Less) {
            sol_05a += page[page.len() / 2];
        } else {
            page.sort_unstable_by(compare);
            sol_05b += page[page.len() / 2];
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
