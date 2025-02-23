use advent_of_code_2024_in_rust::grid;
use itertools::{iproduct, izip};
use ndarray::Array2;

const PATTERN_1: [[(usize, usize); 4]; 8] = [
    [(0, 0), (1, 0), (2, 0), (3, 0)], // +x
    [(3, 0), (2, 0), (1, 0), (0, 0)], // -x
    [(0, 0), (0, 1), (0, 2), (0, 3)], // +y
    [(0, 3), (0, 2), (0, 1), (0, 0)], // -y
    [(0, 0), (1, 1), (2, 2), (3, 3)], // +x +y
    [(3, 3), (2, 2), (1, 1), (0, 0)], // -x -y
    [(0, 3), (1, 2), (2, 1), (3, 0)], // +x -y
    [(3, 0), (2, 1), (1, 2), (0, 3)], // -x -y
];
const TARGET_1: &str = "XMAS";

const PATTERN_2: [[(usize, usize); 6]; 4] = [
    [(0, 0), (1, 1), (2, 2), (0, 2), (1, 1), (2, 0)],
    [(0, 0), (1, 1), (2, 2), (2, 0), (1, 1), (0, 2)],
    [(2, 2), (1, 1), (0, 0), (0, 2), (1, 1), (2, 0)],
    [(2, 2), (1, 1), (0, 0), (2, 0), (1, 1), (0, 2)],
];
const TARGET_2: &str = "MASMAS";

fn main() {
    // Parse the input
    let input = include_str!("../../puzzle_inputs/day_04.txt");
    let input: Array2<char> = grid::parse_char_grid(input, |x| x);

    // Solve 04a
    let sol_04a: usize = solve(&input, &PATTERN_1, TARGET_1);
    let correct_sol_04a: usize = 2543;
    println!("* 04a *");
    println!("My solution: {sol_04a}");
    println!("Correct solution: {correct_sol_04a}");
    println!("Equal: {:?}\n", sol_04a.cmp(&correct_sol_04a));

    // Solve 04b
    let sol_04b: usize = solve(&input, &PATTERN_2, TARGET_2);
    let correct_sol_04b: usize = 1930;
    println!("* 04b *");
    println!("My solution: {sol_04b}");
    println!("Correct solution: {correct_sol_04b}");
    println!("Equal: {:?}\n", sol_04b.cmp(&correct_sol_04b));
}

fn solve<T>(input: &Array2<char>, pattern: &[T], target: &str) -> usize
where
    T: AsRef<[(usize, usize)]>,
{
    iproduct!(ndarray::indices(input.dim()), pattern.iter())
        .filter_map(|(idx, offsets)| {
            izip!(target.chars(), offsets.as_ref())
                .all(|(c, offset)| input.get((idx.0 + offset.0, idx.1 + offset.1)) == Some(&c))
                .then_some(())
        })
        .count()
}
