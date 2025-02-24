use advent_of_code_2024_in_rust::parse_regex::parse_lines;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    // Parse the input
    let input = include_str!("../../puzzle_inputs/day_05.txt");
    let (ordering, pages) = input.split_once("\n\n").unwrap();
    let ordering_regex = Regex::new(r"(\d+)\|(\d+)").unwrap();

    println!("ordering:\n{}", ordering);

    let ordering: Vec<(u32, u32)> = parse_lines(ordering_regex, ordering).collect();
    let pages: Vec<Vec<u32>> = pages
        .lines()
        .map(|line| {
            line.split(",")
                // TODO: get rid of the walrus notation
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    println!("ordering:\n{:?}", ordering);
    println!("pages:");
    pages.iter().for_each(|page| println!("{:?}", page));

    // Compute a partial ordering on ordering
    #[allow(unused_mut, unused_variables)]
    let mut total_order: Vec<u32> = Vec::new();
    let mut is_greater_than: HashMap<u32, Vec<u32>> = ordering
        .iter()
        .flat_map(|(a, b)| [*a, *b])
        .map(|num| (num, Vec::new()))
        .collect();

    // Make sure that greater_than contains all the numbers we want
    for (a, b) in ordering.iter() {
        assert!(
            is_greater_than.contains_key(a),
            "Ordering doesn't contain {}",
            a
        );
        assert!(
            is_greater_than.contains_key(b),
            "Ordering doesn't contain {}",
            b
        );
    }
    println!(
        "is_greater_than len contains all ({}) keys",
        is_greater_than.len()
    );

    println!("is_greater_than (before): {:?}", is_greater_than);
    ordering
        .iter()
        .for_each(|(a, b)| is_greater_than.get_mut(b).unwrap().push(*a));
    println!("is_greater_than (after): {:?}", is_greater_than);
    println!("ordering len {}", ordering.len());
    println!("is_greater_than len {}", is_greater_than.len());

    for (num, greater_then) in &is_greater_than {
        println!(
            "num: {}, greater_then: ({})",
            num,
            greater_then.len(),
            //"num: {}, greater_then: ({}) {:?}",
            //num,
            //greater_then.len(),
            //greater_then
        );
    }

    println!(
        "62 -> ({}) {:?}",
        is_greater_than.get(&62).unwrap().len(),
        is_greater_than.get(&62).unwrap()
    );
    let sixty_twos: Vec<u32> = ordering
        .iter()
        .filter_map(|(a, b)| (*b == 62).then_some(*a))
        .collect();
    println!("sixty_twos: {:?}", sixty_twos);

    let sorted_vec = |x: &[u32]| x.iter().copied().sorted().collect();
    let sixty_twos_a: Vec<u32> = sorted_vec(&is_greater_than[&62]);
    let sixty_twos_b: Vec<u32> = sorted_vec(&sixty_twos);
    println!("sixty_twos_a: ({}) {:?}", sixty_twos_a.len(), sixty_twos_a);
    println!("sixty_twos_b: ({}) {:?}", sixty_twos_b.len(), sixty_twos_b);
    assert_eq!(
        sixty_twos_a, sixty_twos_b,
        "The two sixty_twos are not equal"
    );
    println!("The two sixty_twos are equal");
    unimplemented!();

    //while let Some(num) = is_greater_than
    //    .iter()
    //    .flat_map(|(num, v)| v.is_empty().then_some(*num))
    //    .next()
    //{
    //    total_order.push(num);
    //    is_greater_than.iter_mut().for_each(|(_, v)| {
    //        v.retain(|&x| x != num);
    //    });
    //}
    //
    //println!("total_order: {:?}", total_order);

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
