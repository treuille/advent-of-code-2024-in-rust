use regex::Regex;

fn main() {
    // Parse the input
    let input = include_str!("../../puzzle_inputs/day_03.txt");

    let mul_regex = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let (mut sol_03a, mut sol_03b): (u32, u32) = (0, 0);
    let mut mul_enabled = true;
    for caps in mul_regex.captures_iter(input) {
        if &caps[0] == "do()" {
            mul_enabled = true;
        } else if &caps[0] == "don't()" {
            mul_enabled = false;
        } else {
            let cap_a = &caps[1].parse::<u32>().unwrap();
            let cap_b = &caps[2].parse::<u32>().unwrap();
            let product = cap_a * cap_b;
            sol_03a += product;
            if mul_enabled {
                sol_03b += product;
            }
        }
    }

    // Solve 03a
    let correct_sol_03a: u32 = 183669043;
    println!("* 03a *");
    println!("My solution: {sol_03a}");
    println!("Correct solution: {correct_sol_03a}");
    println!("Equal: {:?}\n", sol_03a.cmp(&correct_sol_03a));

    // Solve 03b
    let correct_sol_03b: u32 = 59097164;
    println!("* 03b *");
    println!("My solution: {sol_03b}");
    println!("Correct solution: {correct_sol_03b}");
    println!("Equal: {:?}\n", sol_03b.cmp(&correct_sol_03b));
}
