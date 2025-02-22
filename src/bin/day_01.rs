use advent_of_code_2024_in_rust::parse_regex::parse_lines;
use regex::Regex;

fn main() {
    // Parse the input
    let input = include_str!("../../puzzle_inputs/day_01.txt");
    let line_regex = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let (mut list_1, mut list_2): (Vec<usize>, Vec<usize>) =
        parse_lines::<(usize, usize)>(line_regex, input).unzip();
    list_1.sort();
    list_2.sort();

    // Solve 01a
    let sol_01a: usize = list_1
        .iter()
        .zip(list_2.iter())
        .map(|(i, j)| i.abs_diff(*j))
        .sum::<usize>();
    let correct_sol_01a: usize = 2904518;
    println!("* 01a *");
    println!("My solution: {sol_01a}");
    println!("Correct solution: {correct_sol_01a}");
    println!("Equal: {:?}\n", sol_01a.cmp(&correct_sol_01a));

    // Solve 01b
    let sol_01b: usize = list_1
        .iter()
        .map(|i| {
            let n_occurrences = list_2.iter().filter(|j| i == *j).count();
            i * n_occurrences
        })
        .sum();
    let correct_sol_01b: usize = 18650129;
    println!("* 01b *");
    println!("My solution: {sol_01b}");
    println!("Correct solution: {correct_sol_01b}");
    println!("Equal: {:?}\n", sol_01b.cmp(&correct_sol_01b));
}
