/* Advent of Code Day 3
 *
 * https://adventofcode.com/2020/day/3
 *
 * input is grid with open squares (.) and trees (#).
 * grid repeats infitely to the right.
 *
 * Part 1
 *
 * start in top left open square and reach the bottom of
 * the grid, count all trees you would encounter by moving
 * right 3, down 1 consecutively.
 *
 * Part 2
 *
 * Use a couple different slopes and return the product of
 * number of trees encountered for each slope
 *
 * ginglis
 */

use std::io::{self, Read};

fn ski_slope(grid: &Vec<&str>, slope_x: usize, slope_y: usize) -> u32 {
    let length = grid.len();
    let width = grid[0].len();
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut trees: u32 = 0;

    while y < length {
        // cannot index &str -> convert to array of chars
        let row: Vec<char> = grid[y].chars().collect();
        let square = row[x % width];

        if square == '#' {
            trees += 1;
        }

        y += slope_y;
        x += slope_x;
    }

    trees
}

fn main() -> io::Result<()> {
    // Read input into string buffer
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    buffer.pop();

    let grid: Vec<&str> = buffer.split("\n").collect();

    println!("Part 1: {}", ski_slope(&grid, 3, 1));

    println!("Part 2: {}", ski_slope(&grid, 1, 1)
        * ski_slope(&grid, 3, 1) * ski_slope(&grid, 5, 1)
        * ski_slope(&grid, 7, 1) * ski_slope(&grid, 1, 2));

    Ok(())
}
