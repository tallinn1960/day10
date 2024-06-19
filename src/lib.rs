use memchr::memchr;
extern crate link_cplusplus;

pub mod github;

pub mod ffi {
    #[link(name = "day10cpp", kind = "static")]
    extern "C" {
        pub fn solve_ffi(bytes: *const u8, size: usize) -> u64;
    }
}
pub fn p1(input: &str) -> usize {
    let map = parse(input);
    if let Some(path) = map.find_loop() {
        path.len() / 2
    } else {
        0
    }
}

pub fn p2(input: &str) -> usize {
    let map = parse(input);
    if let Some(path) = map.find_loop() {
        // Calculate the number of tiles enclosed
        // by the path.
        shoelace_with_picks_theorem(path)
    } else {
        0
    }
}

/// parse the input into a Map with a starting point and a grid
fn parse(input: &str) -> Map {
    let mut start_pos = Location::default();
    let lines = input
        .lines()
        .map(|l| l.as_bytes())
        .enumerate()
        .inspect(|&(y, chars)| {
            if let Some(x) = memchr(b'S', chars) {
                start_pos = Location { x, y };
            }
        })
        .map(|(_, l)| l)
        .collect::<Vec<_>>();
    Map::new(start_pos, lines)
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

impl Location {
    fn north(&self) -> Option<Location> {
        if self.y > 0 {
            Some(Location {
                x: self.x,
                y: self.y.saturating_sub(1),
            })
        } else {
            None
        }
    }

    fn south(&self, maxy: usize) -> Option<Location> {
        if self.y < maxy {
            Some(Location {
                x: self.x,
                y: self.y.saturating_add(1),
            })
        } else {
            None
        }
    }

    fn east(&self, maxx: usize) -> Option<Location> {
        if self.x < maxx {
            Some(Location {
                x: self.x.saturating_add(1),
                y: self.y,
            })
        } else {
            None
        }
    }

    fn west(&self) -> Option<Location> {
        if self.x > 0 {
            Some(Location {
                x: self.x.saturating_sub(1),
                y: self.y,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Map<'a> {
    lower_right: Location,
    map: Vec<&'a [u8]>,
    starting_pos: Location,
}

impl Map<'_> {
    fn new(starting_pos: Location, map: Vec<&[u8]>) -> Map {
        let lower_right = Location {
            x: if let Some(&row) = map.first() {
                row.len()
            } else {
                0
            }
            .saturating_sub(1),
            y: map.len().saturating_sub(1),
        };
        Map {
            lower_right,
            map,
            starting_pos,
        }
    }

    /// Find the next tile to go to from a Location,
    /// coming from a direction not to return to.
    /// Return None if the next tile to go is not connected
    /// (we hit a wall then), or is beyond the border of the map.
    fn next_tile(
        &self,
        loc: &Location,
        coming_from: Direction,
    ) -> Option<(Location, Direction)> {
        match (self.get(*loc), coming_from) {
            // These brackets/no brackets shenanigans are caused by rust-fmt.
            // Note that collapsing branches hurts performance.
            (b'|', Direction::South) => {
                loc.north().map(|north| (north, Direction::South))
            }
            (b'|', Direction::North) => loc
                .south(self.lower_right.y)
                .map(|south| (south, Direction::North)),
            (b'-', Direction::West) => loc
                .east(self.lower_right.x)
                .map(|east| (east, Direction::West)),
            (b'-', Direction::East) => {
                loc.west().map(|west| (west, Direction::East))
            }
            (b'L', Direction::North) => loc
                .east(self.lower_right.x)
                .map(|east| (east, Direction::West)),
            (b'L', Direction::East) => {
                loc.north().map(|north| (north, Direction::South))
            }
            (b'J', Direction::North) => {
                loc.west().map(|west| (west, Direction::East))
            }
            (b'J', Direction::West) => {
                loc.north().map(|north| (north, Direction::South))
            }
            (b'7', Direction::South) => {
                loc.west().map(|west| (west, Direction::East))
            }
            (b'7', Direction::West) => loc
                .south(self.lower_right.y)
                .map(|south| (south, Direction::North)),
            (b'F', Direction::East) => loc
                .south(self.lower_right.y)
                .map(|south| (south, Direction::North)),
            (b'F', Direction::South) => loc
                .east(self.lower_right.x)
                .map(|east| (east, Direction::West)),
            // this should not happen
            _ => unreachable!("you cannot be here!"),
        }
        .filter(|v| match v.1 { // check if we ran against a wall
            Direction::North => {
                memchr(self.get(v.0), &[b'S', b'|', b'L', b'J']).is_some()
            }
            Direction::South => {
                memchr(self.get(v.0), &[b'S', b'|', b'7', b'F']).is_some()
            }
            Direction::East => {
                memchr(self.get(v.0), &[b'S', b'-', b'L', b'F']).is_some()
            }
            Direction::West => {
                memchr(self.get(v.0), &[b'S', b'-', b'7', b'J']).is_some()
            }
        })
    }

    /// Given a location, return the char in the map.
    /// Note that this function does no bound checking.
    fn get(&self, loc: Location) -> u8 {
        unsafe { *(*self.map.get_unchecked(loc.y)).get_unchecked(loc.x) }
    }

    /// Given the location, return a list of all positions that are connected to this location.
    fn connected_to(&self, loc: &Location) -> Vec<(Location, Direction)> {
        let mut result = Vec::new();
        if let Some(north) = loc.north() {
            match self.get(north) {
                b'S' | b'|' | b'F' | b'7' => {
                    result.push((north, Direction::South));
                }
                _ => {}
            }
        }
        if let Some(south) = loc.south(self.lower_right.y) {
            match self.get(south) {
                b'S' | b'|' | b'L' | b'J' => {
                    result.push((south, Direction::North));
                }
                _ => {}
            }
        }
        if let Some(west) = loc.west() {
            match self.get(west) {
                b'S' | b'-' | b'F' | b'L' => {
                    result.push((west, Direction::East));
                }
                _ => {}
            }
        }
        if let Some(east) = loc.east(self.lower_right.x) {
            match self.get(east) {
                b'S' | b'-' | b'J' | b'7' => {
                    result.push((east, Direction::West));
                }
                _ => {}
            }
        }
        result
    }

    /// Find the loop that returns to the starting point, if there is any.
    /// The returned sequence of locations include the starting point as
    /// the first and last location in the list. It is a closed polygon, but with
    /// all integer coordinates in the list, not just the edges.
    fn find_loop(&self) -> Option<Vec<Location>> {
        let mut path_starts = self.connected_to(&self.starting_pos).into_iter();
        // try all tiles connected to S for a loop (not all connected tiles to S may be part of a loop)
        while let Some((mut current, mut coming_from)) = path_starts.next() {
            let mut path = vec![self.starting_pos, current];
            // follow the path until we reach S again or bump into a wall or
            // the border of the map
            while let Some((next_loc, direction)) =
                self.next_tile(&current, coming_from)
            {
                // this is for part2 - the funny thing is, this is faster
                // than counting steps
                path.push(next_loc);
                if next_loc == self.starting_pos {
                    return Some(path);
                }
                current = next_loc;
                coming_from = direction;
            }
        }

        None
    }
}

/// Computes the number of enclosed tiles of the given path.
/// This expects that the last point in the list is
/// the same as the first point in the list, like in the return
/// value of map.find_loop(). This function may panic with
/// integer overflow, depending on the range of x and y coordinates
/// in the path. So keep your map size reasonable.
fn shoelace_with_picks_theorem(path: Vec<Location>) -> usize {
    let n = path.len();
    // shoelace for area
    // we need to switch to isize arithmetic as area may
    // become negative
    let area = (0..n - 1)
        .fold(0, |acc, i| {
            // avoid bound checking is safe here
            let xi = unsafe { path.get_unchecked(i).x } as isize;
            let yi = unsafe { path.get_unchecked(i).y } as isize;
            let x_next = unsafe { path.get_unchecked(i + 1).x } as isize;
            let y_next = unsafe { path.get_unchecked(i + 1).y } as isize;
            acc + (yi + y_next) * (xi - x_next)
        })
        .abs() as usize
        / 2;
    // Pick's theorem
    area - (n - 1) / 2 + 1
}

#[cfg(test)]
mod tests {

    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn test_connected_to() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....        
";
        let map = parse(input);
        let start = map.starting_pos;
        let connected = map.connected_to(&start);
        assert_eq!(connected.len(), 2);
        let expected = vec![
            (Location { x: 1, y: 2 }, Direction::North),
            (Location { x: 2, y: 1 }, Direction::West),
        ];
        assert_eq!(connected, expected);
    }

    #[test]
    fn test_connected_to2() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";
        let map = parse(input);
        let start = map.starting_pos;

        let connected = map.connected_to(&start);
        assert_eq!(connected.len(), 2);
        let expected = vec![
            (Location { x: 1, y: 2 }, Direction::North),
            (Location { x: 2, y: 1 }, Direction::West),
        ];
        assert_eq!(connected, expected);
    }

    #[test]
    fn test_find_loop() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....        
";
        let map = parse(input);
        let steps = map.find_loop().unwrap().len() / 2;
        assert_eq!(steps, 4);
    }

    #[test]
    fn test_shoelace() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....        
";
        let map = parse(input);
        let steps = map.find_loop().unwrap();
        let area = shoelace_with_picks_theorem(steps);
        assert_eq!(area, 1);
    }

    #[test]
    fn test_shoelace2() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let map = parse(input);
        let steps = map.find_loop().unwrap();
        let area = shoelace_with_picks_theorem(steps);
        assert_eq!(area, 4);
    }
    #[test]
    fn test_find_loop2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
        let map = parse(input);
        let steps = map.find_loop().unwrap().len() / 2;
        assert_eq!(steps, 8);
    }

    #[test]
    fn test_next_tile() {
        let input = ".....
.F-7.
.|.|.
.L-J.
.....        
";
        let map = parse(input);
        let result = map
            .next_tile(&Location { x: 1, y: 1 }, Direction::South)
            .unwrap();
        assert_eq!(result, (Location { x: 2, y: 1 }, Direction::West));
        let result = map
            .next_tile(&Location { x: 1, y: 1 }, Direction::East)
            .unwrap();
        assert_eq!(result, (Location { x: 1, y: 2 }, Direction::North));
        let result = map
            .next_tile(&Location { x: 2, y: 1 }, Direction::West)
            .unwrap();
        assert_eq!(result, (Location { x: 3, y: 1 }, Direction::West));
        let result = map
            .next_tile(&Location { x: 2, y: 1 }, Direction::East)
            .unwrap();
        assert_eq!(result, (Location { x: 1, y: 1 }, Direction::East));
        let result = map
            .next_tile(&Location { x: 3, y: 1 }, Direction::West)
            .unwrap();
        assert_eq!(result, (Location { x: 3, y: 2 }, Direction::North));
        let result = map
            .next_tile(&Location { x: 3, y: 1 }, Direction::South)
            .unwrap();
        assert_eq!(result, (Location { x: 2, y: 1 }, Direction::East));
        let result = map
            .next_tile(&Location { x: 1, y: 2 }, Direction::North)
            .unwrap();
        assert_eq!(result, (Location { x: 1, y: 3 }, Direction::North));
        let result = map
            .next_tile(&Location { x: 1, y: 2 }, Direction::South)
            .unwrap();
        assert_eq!(result, (Location { x: 1, y: 1 }, Direction::South));
        let result = map
            .next_tile(&Location { x: 1, y: 3 }, Direction::North)
            .unwrap();
        assert_eq!(result, (Location { x: 2, y: 3 }, Direction::West));
        let result = map
            .next_tile(&Location { x: 1, y: 3 }, Direction::East)
            .unwrap();
        assert_eq!(result, (Location { x: 1, y: 2 }, Direction::South));
        let result = map
            .next_tile(&Location { x: 3, y: 3 }, Direction::North)
            .unwrap();
        assert_eq!(result, (Location { x: 2, y: 3 }, Direction::East));
        let result = map
            .next_tile(&Location { x: 3, y: 3 }, Direction::West)
            .unwrap();
        assert_eq!(result, (Location { x: 3, y: 2 }, Direction::South));
    }

    #[test]
    fn test_p2_sample() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let result = p2(input);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_part1() {
        let mut f = File::open("input.txt").expect("can't open file");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("can't read file");
        let result = p1(&buf);
        assert_eq!(result, 6778);
    }

    #[test]
    fn test_part2() {
        let mut f = File::open("input.txt").expect("can't open file");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("can't read file");
        let result = p2(&buf);
        assert_eq!(result, 433);
    }
}
