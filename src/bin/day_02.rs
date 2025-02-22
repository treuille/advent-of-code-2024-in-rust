use itertools::Itertools;

fn main() {
    // Parse the input
    let input = include_str!("../../puzzle_inputs/day_02.txt");
    let reports: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    // Solve 02a
    let sol_02a: usize = reports
        .iter()
        .filter(|report| safe_part_a(report.iter().cloned()))
        .count();
    let correct_sol_02a: usize = 369;
    println!("* 02a *");
    println!("My solution: {sol_02a}");
    println!("Correct solution: {correct_sol_02a}");
    println!("Equal: {:?}\n", sol_02a.cmp(&correct_sol_02a));

    // Solve 02b
    let sol_02b: usize = reports
        .iter()
        .filter(|report| safe_part_b(report.iter().cloned()))
        .count();
    let correct_sol_02b: usize = 428;
    println!("* 02b *");
    println!("My solution: {sol_02b}");
    println!("Correct solution: {correct_sol_02b}");
    println!("Equal: {:?}\n", sol_02b.cmp(&correct_sol_02b));
}

fn safe_part_a(iter: impl Iterator<Item = usize> + Clone) -> bool {
    let is_safe_increasing = iter
        .clone()
        .tuple_windows()
        .all(|(i, j)| (i < j) && (1..=3).contains(&(j - i)));

    let is_safe_decreasing = iter
        .tuple_windows()
        .all(|(i, j)| (j < i) && (1..=3).contains(&(i - j)));

    is_safe_increasing || is_safe_decreasing
}

fn safe_part_b(iter: impl Iterator<Item = usize> + Clone) -> bool {
    if safe_part_a(iter.clone()) {
        return true;
    }
    (0..iter.clone().count()).any(|skip_idx| {
        safe_part_a(
            iter.clone()
                .enumerate()
                .filter_map(|(idx, val)| (idx != skip_idx).then_some(val)),
        )
    })
}
