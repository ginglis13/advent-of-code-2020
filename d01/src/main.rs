/* Day 1 - AoC
 *
 * Two- and Three- Sum Problems
 *
 * ginglis
 */

use std::collections::HashMap;
use std::io::{self, Read};


fn two_sum(buffer: String, mut map: HashMap<u32, u32>) -> u32 {
    // Process buffer into list of strings delimited by newline and
    // convert to u32
    let numbers: Vec<u32> = buffer.split("\n").map(|x| x.parse::<u32>().unwrap()).collect();

    // into iter means it is on longer available after this. could change
    // to just .iter() if we want to borrow
    for num in numbers.into_iter() {
        // Check if difference is already in map. If so, return
        // num * map[2020-num]
        if map.contains_key(&(num)) {
            return (2020-num) * num;
        } else {
            // Otherwise insert
            map.insert(2020-num, num);
        }
    }

    return 0;
}


fn three_sum(buffer: String) -> u32 {
    // Process buffer into list of strings delimited by newline and
    // convert to u32
    let mut numbers: Vec<u32> = buffer.split("\n").map(|x| x.parse::<u32>().unwrap()).collect();

    numbers.sort();

    let mut res: u32 = 0;

    for i in 0..numbers.len() - 2 {

        // use left and right pointers to traverse vec
        let mut left: usize = i + 1;
        let mut right: usize = numbers.len() - 1;

        while left < right {
            // get sum of ints at each pointer, compare with 2020
            let s: u32 = numbers[i] + numbers[left] + numbers[right];

            if s < 2020 {
                left += 1;
            } else if s > 2020 {
                right -= 1;
            } else {
                res = numbers[i] * numbers[left] * numbers[right];
                return res;
            }
        }
    }

    return res;
}


fn main() -> io::Result<()> {
    // Read input into string buffer
    let mut buffer = String::new();

    // Propagate result with ?
    io::stdin().read_to_string(&mut buffer)?;
    // Newline still exists for some reason. remove with pop()
    buffer.pop();

    let s: String = buffer.clone();
    let map = HashMap::new();

    println!("two-sum:    {}", two_sum(buffer, map));
    println!("three-sum: {}", three_sum(s));

    Ok(())
}
