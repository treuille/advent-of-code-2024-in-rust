use advent_of_code_2024_in_rust::grid;
use itertools::{iproduct, Itertools};
use ndarray::Array2;

fn main() {
    // Parse the input
    let input = include_str!("../../puzzle_inputs/day_04.txt");
    println!("input:\n{}", input);

    let input: Array2<char> = grid::parse_char_grid(input, |x| x);
    println!("input:\n{:?}", input);
    println!("input.dim(): {:?}", input.dim());
    //let (w, h) = input.dim();

    const DIRECTIONS: [(isize, isize); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    const TARGET: &str = "XMAS";
    let iter = iproduct!(ndarray::indices(input.dim()), DIRECTIONS.iter());
    let mut sum: usize = 0;
    iter.for_each(|(idx, direction)| {
        let (x, y) = (idx.0 as isize, idx.1 as isize);
        //println!("x: {}, y: {}", x, y);
        //println!("direction: {:?}", direction);

        let found = TARGET.chars().enumerate().all(|(i, c)| {
            let (dx, dy) = direction;
            let x = x + i as isize * dx;
            if x < 0 {
                return false;
            }
            let y = y + i as isize * dy;
            if y < 0 {
                return false;
            }
            input.get((x as usize, y as usize)) == Some(&c)
        });
        if found {
            sum += 1;
        }
    });

    println!("sum: {}", sum);
    //
    //DIRECTIONS.iter().

    //let mul_regex = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    //let (mut sol_04a, mut sol_04b): (u32, u32) = (0, 0);
    //let mut mul_enabled = true;
    //for caps in mul_regex.captures_iter(input) {
    //    if &caps[0] == "do()" {
    //        mul_enabled = true;
    //    } else if &caps[0] == "don't()" {
    //        mul_enabled = false;
    //    } else {
    //        let cap_a = &caps[1].parse::<u32>().unwrap();
    //        let cap_b = &caps[2].parse::<u32>().unwrap();
    //        let product = cap_a * cap_b;
    //        sol_04a += product;
    //        if mul_enabled {
    //            sol_04b += product;
    //        }
    //    }
    //}

    //// Solve 04a
    //let correct_sol_04a: u32 = 183669043;
    //println!("* 04a *");
    //println!("My solution: {sol_04a}");
    //println!("Correct solution: {correct_sol_04a}");
    //println!("Equal: {:?}\n", sol_04a.cmp(&correct_sol_04a));
    //
    //// Solve 04b
    //let correct_sol_04b: u32 = 59097164;
    //println!("* 04b *");
    //println!("My solution: {sol_04b}");
    //println!("Correct solution: {correct_sol_04b}");
    //println!("Equal: {:?}\n", sol_04b.cmp(&correct_sol_04b));
}
