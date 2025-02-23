use regex::Regex;

fn main() {
    // Parse the input
    let input = include_str!("../../puzzle_inputs/day_04.txt");
    println!("input:\n{}", input);

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
