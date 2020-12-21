/* Advent of Code Day 6
 *
 * Time to play catchup. 12/21/20
 *
 * https://adventofcode.com/2020/day/6
 *
 * Parse customs declarations forms
 *
 * ginglis
 */

use std::io::{self, Read};
use std::collections::{HashSet, HashMap};

// Return number of "yes" responses to survey
// Not a very cool rust soln but it works
fn part1(responses: &Vec<&str>) -> u32 {

    let mut count: u32 = 0;

    for r in responses {
        let mut h = HashSet::new();
        let group = r.split("\n").collect::<Vec<&str>>();

        for response in group {
            for ch in response.chars() {
                h.insert(ch);
            }
        }

        count += h.len() as u32;
    }

    count
}

// Return number of Q's to which each group member responded "yes"
// Not a very cool rust soln but it works
fn part2(responses: &Vec<&str>) -> u32 {
    let mut count: u32 = 0;

    for r in responses {
        let mut m = HashMap::new();
        let group = r.split("\n").collect::<Vec<&str>>();
        let len = group.len();

        for response in group {
            for ch in response.chars() {
                let ch_ct = m.entry(ch).or_insert(0);
                *ch_ct += 1;
            }
        }

        // go thru map, update count with the amount of values == len(group)
        for ch_ct in m {
            if ch_ct.1 == len {
                count += 1
            }
        }
    }

    count

}

fn main() -> io::Result<()> {
    // Read input into string buffer
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    buffer.pop();

    let survey_responses: Vec<&str> = buffer.split("\n\n").collect();

    println!("Part 1: {}", part1(&survey_responses));
    println!("Part 2: {}", part2(&survey_responses));

    Ok(())
}
