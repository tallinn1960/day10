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
        // apply shoelace algorithm on path
        todo!("apply shoelace algorithm on path")
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

    // given certain chars as pipes connecting a Location to its neighbors
    // given a map and a location, return a vector of the neighbors. The chars are
    // | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.

    fn neighbors(&self, loc: &Location) -> Vec<Location> {
        let mut result = Vec::new();
        match self.get((loc.x, loc.y)) {
            '|' => {
                if let Some(north) = loc.north() {
                    result.push(Location {
                        x: north.0,
                        y: north.1,
                    });
                }
                if let Some(south) = loc.south(self.lower_right.y) {
                    result.push(Location {
                        x: south.0,
                        y: south.1,
                    });
                }
            }
            '-' => {
                if let Some(east) = loc.east(self.lower_right.x) {
                    result.push(Location {
                        x: east.0,
                        y: east.1,
                    });
                }
                if let Some(west) = loc.west() {
                    result.push(Location {
                        x: west.0,
                        y: west.1,
                    });
                }
            }
            'L' => {
                if let Some(north) = loc.north() {
                    result.push(Location {
                        x: north.0,
                        y: north.1,
                    });
                }
                if let Some(east) = loc.east(self.lower_right.x) {
                    result.push(Location {
                        x: east.0,
                        y: east.1,
                    });
                }
            }
            'J' => {
                if let Some(north) = loc.north() {
                    result.push(Location {
                        x: north.0,
                        y: north.1,
                    });
                }
                if let Some(west) = loc.west() {
                    result.push(Location {
                        x: west.0,
                        y: west.1,
                    });
                }
            }
            '7' => {
                if let Some(south) = loc.south(self.lower_right.y) {
                    result.push(Location {
                        x: south.0,
                        y: south.1,
                    });
                }
                if let Some(west) = loc.west() {
                    result.push(Location {
                        x: west.0,
                        y: west.1,
                    });
                }
            }
            'F' => {
                if let Some(south) = loc.south(self.lower_right.y) {
                    result.push(Location {
                        x: south.0,
                        y: south.1,
                    });
                }
                if let Some(east) = loc.east(self.lower_right.x) {
                    result.push(Location {
                        x: east.0,
                        y: east.1,
                    });
                }
            }
            _ => {}
        }
        result
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
    fn connected_to(&self, loc: &Location) -> Vec<Location> {
        let mut result: Vec<Location> = Vec::new();
        if let Some(north) = loc.north() {
            match self.get(north) {
                'S' | '|' | 'F' | '7' => {
                    result.push(north.into());
                }
                _ => {}
            }
        }
        if let Some(south) = loc.south(self.lower_right.y) {
            match self.get(south) {
                'S' | '|' | 'L' | 'J' => {
                    result.push(south.into());
                }
                _ => {}
            }
        }
        if let Some(west) = loc.west() {
            match self.get(west) {
                'S' | '-' | 'F' | 'L' => {
                    result.push(west.into());
                }
                _ => {}
            }
        }
        if let Some(east) = loc.east(self.lower_right.x) {
            match self.get(east) {
                'S' | '-' | 'J' | '7' => {
                    result.push(east.into());
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
            let mut last_visited = start;
            // protect vs isolated starting point
            if let Some(current) = self.connected_to(&start).first() {
                let mut current = *current;
                let mut path = vec![start, current];
                while let Some(next_loc) = self
                    .neighbors(&current)
                    .into_iter()
                    .find(|&loc| loc != last_visited)
                {
                    path.push(next_loc); // this is for part2
                    if next_loc == start {
                        return Some(path);
                    }
                    last_visited = current;
                    current = next_loc;
                }
            }
        }
        None
    }
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
    fn test_neighbors() {
        let map = Map {
            lower_right: Location { x: 2, y: 2 },
            map: &["...", ".J.", "..."],
        };
        let start = Location { x: 1, y: 1 };
        let neighbors = map.neighbors(&start);
        assert_eq!(neighbors.len(), 2);
        let expected = vec![Location { x: 1, y: 0 }, Location { x: 0, y: 1 }];
        assert_eq!(neighbors, expected);
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
        let expected = vec![Location { x: 1, y: 2 }, Location { x: 2, y: 1 }];
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
        let expected = vec![Location { x: 1, y: 2 }, Location { x: 2, y: 1 }];
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
    fn test_part1() {
        let mut f = File::open("input.txt").expect("can't open file");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("can't read file");
        let result = p1(&buf);
        assert_eq!(result, 6778);
    }
}
