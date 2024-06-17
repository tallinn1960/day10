pub mod github;
pub mod github2;

pub fn p1(input: &str) -> u64 {
    let (start_pos, lines) = parse(input);
    let map = Map::new(&lines);
    if let Some(path) = map.find_loop(start_pos) {
        (path.len() / 2) as u64
    } else {
        0
    }
}

fn parse(input: &str) -> ((u32, u32), Vec<Vec<char>>) {
    let mut start_pos = (0, 0);
    let lines = input
        .lines()
        .enumerate()
        .inspect(|(y, chars)| {
            if let Some(pos) = chars.find(|c| c == 'S') {
                start_pos = (pos as u32, *y as u32)
            }
        })
        .map(|(_, l)| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    (start_pos, lines)
}

pub fn p2(input: &str) -> u64 {
    let (start_pos, lines) = parse(input);
    let map = Map::new(&lines);
    if let Some(_path) = map.find_loop(start_pos) {
        // calculate are by shoelace, apply picks theorem
        // for the number of enclosed tiles
        shoelace_with_picks_theorem(_path) as u64
    } else {
        0
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
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
    map: &'a [Vec<char>],
}

impl Map<'_> {
    fn new(map: &[Vec<char>]) -> Map<'_> {
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
            ('|', Direction::South) => {
                loc.north().map(|north| (north.into(), Direction::South))
            }
            ('|', Direction::North) => loc
                .south(self.lower_right.y)
                .map(|north| (north.into(), Direction::North)),
            ('-', Direction::West) => loc
                .east(self.lower_right.x)
                .map(|north| (north.into(), Direction::West)),
            ('-', Direction::East) => {
                loc.west().map(|north| (north.into(), Direction::East))
            }
            ('L', Direction::North) => loc
                .east(self.lower_right.x)
                .map(|north| (north.into(), Direction::West)),
            ('L', Direction::East) => {
                loc.north().map(|north| (north.into(), Direction::South))
            }
            ('J', Direction::North) => {
                loc.west().map(|north| (north.into(), Direction::East))
            }
            ('J', Direction::West) => {
                loc.north().map(|north| (north.into(), Direction::South))
            }
            ('7', Direction::South) => {
                loc.west().map(|north| (north.into(), Direction::East))
            }
            ('7', Direction::West) => loc
                .south(self.lower_right.y)
                .map(|north| (north.into(), Direction::North)),
            ('F', Direction::East) => loc
                .south(self.lower_right.y)
                .map(|north| (north.into(), Direction::North)),
            ('F', Direction::South) => loc
                .east(self.lower_right.x)
                .map(|north| (north.into(), Direction::West)),
            _ => None,
        }
    }

    // given a location, return the char in the map
    fn get(&self, loc: Location) -> char {
        self.map[loc.y as usize][loc.x as usize]
    }

    // given the location, return a list of all positions that are connected to this location
    fn connected_to(&self, loc: &Location) -> Vec<(Location, Direction)> {
        let mut result = Vec::new();
        if let Some(north) = loc.north() {
            match self.get(north) {
                'S' | '|' | 'F' | '7' => {
                    result.push((north.into(), Direction::South));
                }
                _ => {}
            }
        }
        if let Some(south) = loc.south(self.lower_right.y) {
            match self.get(south) {
                'S' | '|' | 'L' | 'J' => {
                    result.push((south.into(), Direction::North));
                }
                _ => {}
            }
        }
        if let Some(west) = loc.west() {
            match self.get(west) {
                'S' | '-' | 'F' | 'L' => {
                    result.push((west.into(), Direction::East));
                }
                _ => {}
            }
        }
        if let Some(east) = loc.east(self.lower_right.x) {
            match self.get(east) {
                'S' | '-' | 'J' | '7' => {
                    result.push((east.into(), Direction::West));
                }
                _ => {}
            }
        }
        result
    }

    // given a map, find the start location, and follow the path leading from there,
    // until you return to the start location, return the found path, if there is any,
    // or None. As every tile has only one next non-visited reachable tile when reached, we find
    // either a loop returning to start or end at the border of the map (which leads to a None result).
    fn find_loop(&self, start: (u32, u32)) -> Option<Vec<Location>> {
        let start = Location {
            x: start.0,
            y: start.1,
        };
        // this assumes that any reachable tile from S is part of the loop
        if let Some((mut current, mut coming_from)) =
            self.connected_to(&start).first()
        {
            let mut path = vec![start, current];
            while let Some((next_loc, direction)) =
                self.next_tile(&current, coming_from)
            {
                path.push(next_loc); // this is for part2
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

fn shoelace_with_picks_theorem(path: Vec<Location>) -> u32 {
    let mut result = 0;
    let n = path.len();
    for i in 0..n - 1 {
        let xi = (path[i].x) as i64;
        let yi = (path[i].y) as i64;
        let x_next = (path[i + 1].x) as i64;
        let y_next = (path[i + 1].y) as i64;
        result += (yi + y_next) * (xi - x_next);
    }
    (((result / 2).abs()) - (path.len() as i64 - 1) / 2 + 1) as u32
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
        let start = Location {
            x: start.0,
            y: start.1,
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
        let start = Location { x: start.0, y: start.1 };
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
        let start = start.into();
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
        let start = start.into();
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
        let start = start.into();
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
        let start = start.into();
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
    fn test_part1() {
        let mut f = File::open("input.txt").expect("can't open file");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("can't read file");
        let result = p1(&buf);
        assert_eq!(result, 6778);
    }
}
