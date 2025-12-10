

use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::{Instant};

mod day_10;

fn read_to_vec<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let lines = br.lines()
        .map(|l| l.expect("Parse error"))
        .collect();

    Ok(lines)
}

fn main() -> Result<(), Error>
{
    let mut now = Instant::now();
    let input : Vec<String> = read_to_vec(File::open("inputs/day_10/input.txt")?)?;
    println!("Read input: {} µs", now.elapsed().as_micros());

    now = Instant::now();
    let r1 = day_10::part_1(&input);
    println!("Part 1: {} ms", now.elapsed().as_micros() as f32 / 1000.0);

    now = Instant::now();
    let r2 = day_10::part_2(&input);
    println!("Part 2: {} ms", now.elapsed().as_micros() as f32/ 1000.0);

    println!("Result 1: {}\nResult 2: {}", r1, r2);

    Ok(())
}