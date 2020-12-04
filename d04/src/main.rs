/* Advent of Code Day 4
 *
 * https://adventofcode.com/2020/day/4
 *
 * Part 1
 * determine if a passport is valid - needs fields
 * byr, iyr, eyr, hgt, hcl, ecl, pid, and cid
 * Passports are in a batch file - see input.txt
 * Treat cid as an optional field.
 *
 * Part 2
 * Determine if passport is valid with more constraints.
 * byr >= 1920 && <= 2002
 * iyr >= 2010 && <= 2020
 * etc.
 *
 * ginglis
 */

use std::collections::HashSet;
use std::io::{self, Read};

fn is_string_numeric(s: String) -> bool {
    for ch in s.chars() {
        if !ch.is_numeric() {
            return false;
        }
    }
    return true;
}

fn is_valid_part2(key: &str, val: &str) -> bool {
    match key {
        "byr" => {
            let yr = val.parse::<i32>().unwrap();
            return yr >= 1920 && yr <= 2002;
        }
        "iyr" => {
            let yr = val.parse::<i32>().unwrap();
            return yr >= 2010 && yr <= 2020;
        }
        "eyr" => {
            let yr = val.parse::<i32>().unwrap();
            return yr >= 2020 && yr <= 2030;
        }
        "hgt" => {
            if val.ends_with("cm") {
                let split: Vec<&str> = val.split("cm").collect();
                let num = split[0].parse::<i32>().unwrap();
                return num >= 150 && num <= 193;
            } else if val.ends_with("in") {
                let split: Vec<&str> = val.split("in").collect();
                let num = split[0].parse::<i32>().unwrap();
                return num >= 59 && num <= 76;
            } else {
                return false;
            }
        }
        "hcl" => {
            if !val.starts_with("#") {
                return false;
            } else {
                for ch in val.split("#").collect::<Vec<&str>>()[0].chars() {
                    if ![
                        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
                        'a', 'b', 'c', 'd', 'e', 'f',
                    ]
                    .contains(&ch) {
                        return false;
                    }
                }
                return true;
            }
        }
        "ecl" => {
            return ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val);
        }
        "pid" => {
            return is_string_numeric(val.to_string()) && val.len() == 9;
        }
        "cid" => {
            return true;
        }
        _ => {
            println!("[error] Unknown field \"{}\"", key);
            return false;
        }
    }
}

fn count_valid_passports(passports: &Vec<&str>, part2: bool) -> u32 {
    let mut count: u32 = 0;

    for passport in passports {
        let mut req_fields: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .into_iter()
            .collect();

        // delimit by newline now
        let info: Vec<&str> = passport.split("\n").collect();
        let mut valid: bool = true;
        for fields in info {
            let space_split: Vec<&str> = fields.split(" ").collect();
            for field in space_split {
                let f: Vec<&str> = field.split(":").collect();

                if req_fields.contains(f[0]) {
                    req_fields.remove(f[0]);
                }

                if !is_valid_part2(f[0], f[1]) && part2 {
                    valid = false;
                }
            }
        }

        if req_fields.len() == 0 && valid {
            count += 1
        }
    }

    count
}

fn main() -> io::Result<()> {
    // Read input into string buffer
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    buffer.pop();

    // Process buffer into list of strings delimited by
    // double newline such that each passport info is one str
    let passports: Vec<&str> = buffer.split("\n\n").collect();

    println!("Part 1: {}", count_valid_passports(&passports, false));
    println!("Part 2: {}", count_valid_passports(&passports, true));

    Ok(())
}
