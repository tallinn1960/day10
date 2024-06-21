extern crate link_cplusplus;

#[link(name = "day10cpp", kind = "static")]
extern "C" {
    pub fn solve_ffi(bytes: *const u8, size: usize) -> u64;
}
#[link(name = "day10cpp", kind = "static")]
extern "C" {
    pub fn run_p1(bytes: *const u8, size: usize) -> u64;
}

#[cfg(test)]
pub fn p1_cpp(input: &str) -> u64 {
    unsafe {run_p1(input.as_ptr(), input.len())}
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use crate::ffi::run_p1;

    #[test]
    fn test_cpp_sample() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
        let result = unsafe {run_p1(input.as_ptr(), input.len())};
        assert_eq!(result, 80);
    }

    #[test]
    fn test_cpp_part1() {
        let mut f = File::open("input.txt").expect("can't open file");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("can't read file");
        let result = unsafe {run_p1(buf.as_ptr(), buf.len())};
        assert_eq!(result, 6778);
    }

}
