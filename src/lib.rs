pub mod github;

pub fn p1(input: &str) -> u64 {
    let (start_pos, lines) = parse(input);
    let map = Map::new(&lines);
    if let Some(path) = map.find_loop(start_pos) {
        (path.len() / 2) as u64
    } else {
        0
    }
}

pub fn p2(input: &str) -> u64 {
    let (start_pos, lines) = parse(input);
    let map = Map::new(&lines);
    if let Some(path) = map.find_loop(start_pos) {
        // calculate area by shoelace, apply picks theorem
        // for the number of enclosed tiles
        shoelace_with_picks_theorem(path) as u64
    } else {
        0
    }
}

/// parse the input into a starting point and a grid of bytes
fn parse(input: &str) -> (Location, Vec<Vec<u8>>) {
    let mut start_pos = Location::default();
    let lines = input
        .lines()
        .enumerate()
        .inspect(|&(y, chars)| {
            if let Some(pos) = chars.find(|c| c == 'S') {
                start_pos = Location {
                    x: pos as u32,
                    y: y as u32,
                };
            }
        })
        .map(|(_, l)| l.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    (start_pos, lines)
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
struct Location {
    x: u32,
    y: u32,
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

    fn south(&self, maxy: u32) -> Option<Location> {
        if self.y < maxy {
            Some(Location {
                x: self.x,
                y: self.y.saturating_add(1),
            })
        } else {
            None
        }
    }

    fn east(&self, maxx: u32) -> Option<Location> {
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
    map: &'a [Vec<u8>],
}

impl Map<'_> {
    fn new(map: &[Vec<u8>]) -> Map<'_> {
        let lower_right = Location {
            x: (if let Some(row) = map.first() {
                row.len()
            } else {
                0
            } as u32)
                .saturating_sub(1),
            y: (map.len() as u32).saturating_sub(1),
        };
        Map { lower_right, map }
    }

    //  The chars are
    // | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.

    fn next_tile(
        &self,
        loc: &Location,
        coming_from: Direction,
    ) -> Option<(Location, Direction)> {
        match (self.get(*loc), coming_from) {
            // these brackets/no brackets shenanigans are caused by rust-fmt
            (b'|', Direction::South) => {
                loc.north().map(|north| (north, Direction::South))
            }
            (b'|', Direction::North) => loc
                .south(self.lower_right.y)
                .map(|north| (north, Direction::North)),
            (b'-', Direction::West) => loc
                .east(self.lower_right.x)
                .map(|north| (north, Direction::West)),
            (b'-', Direction::East) => {
                loc.west().map(|north| (north, Direction::East))
            }
            (b'L', Direction::North) => loc
                .east(self.lower_right.x)
                .map(|north| (north, Direction::West)),
            (b'L', Direction::East) => {
                loc.north().map(|north| (north, Direction::South))
            }
            (b'J', Direction::North) => {
                loc.west().map(|north| (north, Direction::East))
            }
            (b'J', Direction::West) => {
                loc.north().map(|north| (north, Direction::South))
            }
            (b'7', Direction::South) => {
                loc.west().map(|north| (north, Direction::East))
            }
            (b'7', Direction::West) => loc
                .south(self.lower_right.y)
                .map(|north| (north, Direction::North)),
            (b'F', Direction::East) => loc
                .south(self.lower_right.y)
                .map(|north| (north, Direction::North)),
            (b'F', Direction::South) => loc
                .east(self.lower_right.x)
                .map(|north| (north, Direction::West)),
            _ => None,
        }
    }

    // given a location, return the char in the map
    fn get(&self, loc: Location) -> u8 {
        self.map[loc.y as usize][loc.x as usize]
    }

    // given the location, return a list of all positions that are connected to this location
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

    /// given a map, find the start location, and follow the path leading from there,
    /// until you return to the start location, return the found path, if there is any,
    /// or None. As every tile has only one next non-visited reachable tile when reached, we find
    /// either a loop returning to start or end at the border of the map (which leads to a None result).
    fn find_loop(&self, start: Location) -> Option<Vec<Location>> {
        // this assumes that any reachable tile from S is part of the loop
        if let Some((mut current, mut coming_from)) =
            self.connected_to(&start).first()
        {
            let mut path = vec![start, current];
            while let Some((next_loc, direction)) =
                self.next_tile(&current, coming_from)
            {
                // this is for part2 - the funny thing is, this is faster than counting steps
                path.push(next_loc);
                if next_loc == start {
                    return Some(path);
                }
                current = next_loc;
                coming_from = direction;
            }
        }

        None
    }
}

/// this assumes that the last point in the list is
/// the same as the first point in the list
fn shoelace_with_picks_theorem(path: Vec<Location>) -> u32 {
    let n = path.len();
    // shoelace for area
    let area: i64 = (0..n - 1)
        .fold(0, |acc, i| {
            let xi = (path[i].x) as i64;
            let yi = (path[i].y) as i64;
            let x_next = (path[i + 1].x) as i64;
            let y_next = (path[i + 1].y) as i64;
            acc + (yi + y_next) * (xi - x_next)
        })
        .abs()
        / 2;
    // Pick's theorem
    (area - (path.len() as i64 - 1) / 2 + 1) as u32
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
        let (start, v) = parse(input);
        let map = Map::new(&v);
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
        let (start, v) = parse(input);
        let map = Map {
            lower_right: Location { x: 4, y: 4 },
            map: &v,
        };

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
        let (start, v) = parse(input);
        let map = Map {
            lower_right: Location { x: 4, y: 4 },
            map: &v,
        };
        let start = start;
        let steps = map.find_loop(start).unwrap().len() / 2;
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
        let (start, v) = parse(input);
        let map = Map {
            lower_right: Location { x: 4, y: 4 },
            map: &v,
        };
        let start = start;
        let steps = map.find_loop(start).unwrap();
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
        let (start, v) = parse(input);
        let map = Map::new(&v);
        let start = start;
        let steps = map.find_loop(start).unwrap();
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
        let (start, v) = parse(input);
        let map = Map {
            lower_right: Location { x: 4, y: 4 },
            map: &v,
        };
        let start = start;
        let steps = map.find_loop(start).unwrap().len() / 2;
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
        let (_, v) = parse(input);
        let map = Map::new(&v);
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
