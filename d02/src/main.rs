/* Advent of Code Day 2
 *
 * Check if passwords are valid
 *
 * 1-3 a: abcde indicates at least 1 'a' must be present
 * and at most 3 'a's can be present. Return the total number of
 * valid passwords.
 *
 * Part 2: 1-3 indicates that exactly one of position 1 and position
 * 3 in the password must be 'a'. Return count of valid passwords.
 *
 * ginglis
 */

use std::io::{self, Read};

fn check_pword(min: u32, max: u32, chr: char, pword: String) -> bool {
    let mut num = 0;
    for c in pword.chars() {
        if c == chr {
            num += 1;
        }

    }
    return num >= min && num <= max;
}

fn count_valid(input: String) -> u32 {
    // Process buffer into list of strings delimited by newline
    let passwords: Vec<&str> = input.split("\n").collect();

    let mut total: u32 = 0;

    // Example password input: 1-3 a: abc123
    for password in passwords {
        let input = password.split("-").collect::<Vec<&str>>();

        let min = input[0].parse::<u32>().unwrap();
        let max = input[1].split(" ").collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
        let max_vec = input[1].split(" ").collect::<Vec<&str>>();
        let chr = max_vec[1].split(":").collect::<Vec<&str>>()[0];
        let pword = max_vec[2];

        let c: Vec<char> = chr.chars().collect();

        if check_pword(min, max, c[0], pword.to_string()) {
            total += 1;
        }
    }

    return total;
}

fn part_two(input: String) -> u32 {
    let passwords: Vec<&str> = input.split("\n").collect();

    let mut total: u32 = 0;

    for password in passwords {
        let input = password.split("-").collect::<Vec<&str>>();

        let begin = input[0].parse::<usize>().unwrap();
        let end = input[1].split(" ").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
        let max_vec = input[1].split(" ").collect::<Vec<&str>>();
        let chr = max_vec[1].split(":").collect::<Vec<&str>>()[0];
        let pword = max_vec[2];

        let c: Vec<char> = chr.chars().collect();

        if pword.as_bytes()[begin-1] as char == c[0] && pword.as_bytes()[end-1] as char != c[0] {
            total += 1;
        } else if pword.as_bytes()[begin-1] as char != c[0] && pword.as_bytes()[end-1] as char == c[0] {
            total += 1;
        }
    }

    return total;
}

fn main() -> io::Result<()> {
    // Read input into string buffer
    let mut buffer = String::new();

    // Propagate result with ?
    io::stdin().read_to_string(&mut buffer)?;
    // Newline still exists for some reason. remove with pop()
    buffer.pop();

    let s: String = buffer.clone();

    println!("\nPart 1: {}", count_valid(buffer));
    println!("Part 2: {}", part_two(s));

    Ok(())
}
