/* Advent of Code Day 5
 *
 * https://adventofcode.com/2020/day/5
 *
 * Part 1
 * Find row and column of seat on plane using binary partitioning
 * First 7 letters are (F)ront or (B)ack, last 3 are (L)eft or (R)ight.
 * 128 rows (0-127) and 8 columns (0-7)
 *
 * ginglis
 */

use std::io::{self, Read};

fn find_row(pass: &str) -> u32 {
    let mut start: u32 = 0;
    let mut end: u32 = 127;

    for ch in pass.chars() {

        if ch == 'F' {
            end -= (end - start) / 2 + 1;
        } else if ch == 'B' {
            start += (end - start) / 2 + 1;
        }

    }


    return if start < end {start} else {end}
}

fn find_column(pass: &str) -> u32 {
    let mut start: u32 = 0;
    let mut end: u32 = 7;

    for ch in pass.chars() {

        if ch == 'L' {
            end -= (end - start) / 2 + 1;
        } else if ch == 'R' {
            start += (end - start) / 2 + 1;
        }

    }


    return if start < end {start} else {end}
}

fn part1(boarding_passes: &Vec<&str>) -> u32 {

    let mut max: u32 = 0;

    for pass in boarding_passes {
        let seat_id: u32 = find_row(pass) * 8 + find_column(pass);
        if seat_id > max {
            max = seat_id;
        }
    }

    max
}

fn part2(boarding_passes: &Vec<&str>) -> u32 {
    let mut seat_ids = boarding_passes.iter()
        .map(|pass| 8 * find_row(pass) + find_column(pass))
        .collect::<Vec<u32>>();

    // Windows function separates vector into windows of size
    seat_ids.sort();
    seat_ids.windows(2).find(|w| w[0] + 2 == w[1])
        .map(|w| w[0] + 1)
        .unwrap_or(0)
}

fn main() -> io::Result<()> {
    // Read input into string buffer
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    buffer.pop();

    let boarding_passes: Vec<&str> = buffer.split("\n").collect();

    println!("Part 1: {}", part1(&boarding_passes));
    println!("Part 2: {}", part2(&boarding_passes));

    Ok(())
}
