pub fn p1(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<_>>();
    let map = Map::new(&lines);
    if let Some(path) = map.find_loop() {
        (path.len() / 2) as u64
    } else {
        0
    }
}

pub fn p2(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<_>>();
    let map = Map::new(&lines);
    if let Some(_path) = map.find_loop() {
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
    fn north(&self) -> Option<(u32, u32)> {
        if self.y > 0 {
            Some((self.x, self.y.saturating_sub(1)))
        } else {
            None
        }
    }

    fn south(&self, maxy: u32) -> Option<(u32, u32)> {
        if self.y < maxy {
            Some((self.x, self.y.saturating_add(1)))
        } else {
            None
        }
    }

    fn east(&self, maxx: u32) -> Option<(u32, u32)> {
        if self.x < maxx {
            Some((self.x.saturating_add(1), self.y))
        } else {
            None
        }
    }

    fn west(&self) -> Option<(u32, u32)> {
        if self.x > 0 {
            Some((self.x.saturating_sub(1), self.y))
        } else {
            None
        }
    }
}

impl From<(u32, u32)> for Location {
    fn from((x, y): (u32, u32)) -> Self {
        Location { x, y }
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
    map: &'a [&'a str],
}

impl Map<'_> {
    fn new<'a>(map: &'a [&'a str]) -> Map<'a> {
        let lower_right = Location {
            x: (if let Some(&row) = map.first() {
                row.len()
            } else {
                0
            } as u32)
                .saturating_sub(1),
            y: (map.len() as u32).saturating_sub(1),
        };
        Map { lower_right, map }
    }

    // given an x,y map of chars of dimenson 0..MAXX, 0..MAXY find the x,y position of
    // a cell with the char S in it, there is only one
    fn find_start(&self) -> Option<Location> {
        for y in 0..self.map.len() {
            let line = self.map[y];
            if let Some(x) = line.find('S') {
                return Some(Location {
                    x: x as u32,
                    y: y as u32,
                });
            }
        }
        None
    }

    //  The chars are
    // | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.

    #[inline]
    fn next_tile(
        &self,
        loc: &Location,
        coming_from: Direction,
    ) -> Option<(Location, Direction)> {
        match self.get((loc.x, loc.y)) {
            // these brackets/no brackets shenanigans are caused by rust-fmt
            '|' if coming_from == Direction::South => {
                loc.north().map(|north| (north.into(), Direction::South))
            }
            '|' if coming_from == Direction::North => loc
                .south(self.lower_right.y)
                .map(|north| (north.into(), Direction::North)),
            '-' if coming_from == Direction::West => loc
                .east(self.lower_right.x)
                .map(|north| (north.into(), Direction::West)),
            '-' if coming_from == Direction::East => {
                loc.west().map(|north| (north.into(), Direction::East))
            }
            'L' if coming_from == Direction::North => loc
                .east(self.lower_right.x)
                .map(|north| (north.into(), Direction::West)),
            'L' if coming_from == Direction::East => {
                loc.north().map(|north| (north.into(), Direction::South))
            }
            'J' if coming_from == Direction::North => {
                loc.west().map(|north| (north.into(), Direction::East))
            }
            'J' if coming_from == Direction::West => {
                loc.north().map(|north| (north.into(), Direction::South))
            }
            '7' if coming_from == Direction::South => {
                loc.west().map(|north| (north.into(), Direction::East))
            }
            '7' if coming_from == Direction::West => loc
                .south(self.lower_right.y)
                .map(|north| (north.into(), Direction::North)),
            'F' if coming_from == Direction::East => loc
                .south(self.lower_right.y)
                .map(|north| (north.into(), Direction::North)),
            'F' if coming_from == Direction::South => loc
                .east(self.lower_right.x)
                .map(|north| (north.into(), Direction::West)),
            _ => None,
        }
    }

    // given a location, return the char in the map
    #[inline]
    fn get(&self, loc: (u32, u32)) -> char {
        self.map[loc.1 as usize]
            .chars()
            .nth(loc.0 as usize)
            .unwrap_or('.')
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
    fn find_loop(&self) -> Option<Vec<Location>> {
        // protect vs empty map
        if let Some(start) = self.find_start() {
            // protect vs isolated starting point
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
    fn test_find_start() {
        let v = vec!["S..", "...", "..."];
        let map = Map {
            lower_right: Location { x: 2, y: 2 },
            map: &v,
        };
        let start = map.find_start();
        assert_eq!(start, Some(Location { x: 0, y: 0 }));
    }

    #[test]
    fn test_connected_to() {
        let map = Map {
            lower_right: Location { x: 4, y: 4 },
            map: &[".....", ".S-7.", ".|.|.", ".L-J.", "....."],
        };
        let start = Location { x: 1, y: 1 };
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
        let map = Map {
            lower_right: Location { x: 4, y: 4 },
            map: &["-L|F7", "7S-7|", "L|7||", "-L-J|", "L|-JF"],
        };
        let start = Location { x: 1, y: 1 };
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
        let v = vec![".....", ".S-7.", ".|.|.", ".L-J.", "....."];
        let map = Map::new(&v);
        let steps = map.find_loop().unwrap().len() / 2;
        assert_eq!(steps, 4);
    }

    #[test]
    fn test_shoelace() {
        let v = vec![".....", ".S-7.", ".|.|.", ".L-J.", "....."];
        let map = Map::new(&v);
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
        let v = input.lines().collect::<Vec<_>>();
        let map = Map::new(&v);
        let steps = map.find_loop().unwrap();
        let area = shoelace_with_picks_theorem(steps);
        assert_eq!(area, 4);
    }
    #[test]
    fn test_find_loop2() {
        let v = vec!["..F7.", ".FJ|.", "SJ.L7", "|F--J", "LJ..."];
        let map = Map::new(&v);
        let steps = map.find_loop().unwrap().len() / 2;
        assert_eq!(steps, 8);
    }

    #[test]
    fn test_find_loop_empty() {
        let v = Vec::new();
        let map = Map::new(&v);
        let path = map.find_loop();
        assert_eq!(path, None);
    }

    #[test]
    fn test_find_loop_isolated() {
        let v = vec!["...", ".S.", "..."];
        let map = Map::new(&v);
        let path = map.find_loop();
        assert_eq!(path, None);
    }
    #[test]
    fn test_map_wih_no_loop() {
        let v = vec![".7.", "-S-", ".|."];
        let map = Map::new(&v);
        let path = map.find_loop();
        assert_eq!(path, None);
    }

    #[test]
    fn test_map_wih_no_starting_point() {
        let v = vec!["F-7", "|.|", "L-J"];
        let map = Map::new(&v);
        let path = map.find_loop();
        assert_eq!(path, None);
    }

    #[test]
    fn test_next_tile() {
        let v = vec![".....", ".F-7.", ".|.|.", ".L-J.", "....."];
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
