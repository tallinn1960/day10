use std::{fs::File, io::Read};

use day10::{p1, p2};

fn main() {
    let mut f = File::open("input.txt").expect("can't open file");
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("can't read file");
    let result = p1(&buf);
    println!("{result}");
    let result = p2(&buf);
    println!("{result}");
}
