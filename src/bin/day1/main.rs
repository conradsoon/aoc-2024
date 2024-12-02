use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn solve() -> io::Result<i64> {
    let file = File::open("src/inputs/day01.txt")?;
    let reader = BufReader::new(file);
    let mut x_v: Vec<i64> = vec![];
    let mut y_v: Vec<i64> = vec![];

    for line in reader.lines() {
        let _line = line?;
        let tokens: Vec<i64> = _line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        x_v.push(tokens[0]);
        y_v.push(tokens[1]);
    }

    x_v.sort();
    y_v.sort();
    let mut ans: i64 = 0;

    for i in 0..x_v.len() {
        ans += (y_v[i] - x_v[i]).abs();
    }

    Ok(ans)
}

fn main() -> io::Result<()> {
    println!("{}", solve()?);
    Ok(())
}
