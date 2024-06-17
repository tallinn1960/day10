use hashbrown::HashSet;

fn find_loop(
    graph: &[Vec<[bool; 4]>],
    start: (usize, usize),
) -> Option<HashSet<(usize, usize)>> {
    let (mut r, mut c) = start;
    let mut d = graph[r][c].iter().position(|&d| d).unwrap(); // start in the first direction found
    let mut seen = HashSet::new();
    loop {
        if !seen.insert((r, c)) {
            // r,c is the starting position again, break
            break Some(seen);
        }
        // d is the direction we've taken
        // came_from is the direction we should not take
        // to continue on the path
        let came_from = match d {
            // we went up, so do not go down again
            0 => {
                r -= 1;
                2
            }
            // we went right, so do not go left again
            1 => {
                c += 1;
                3
            }
            // we went down, so do not go up again
            2 => {
                r += 1;
                0
            }
            // we went left, so do not go right again
            3 => {
                c -= 1;
                1
            }
            _ => unreachable!(),
        };
        // is the next tile connected to our position?
        // if not, we bumped into a wall and the path
        // taken ends without reaching the start position
        // again, so it is not a valid path
        if !graph[r][c][came_from] {
            break None;
        }
        // find the next direction to go
        d = (0..4).find(|&i| i != came_from && graph[r][c][i]).unwrap();
    }
}

fn connections(tile: u8) -> [bool; 4] {
    match tile {
        //      [   up, right,  down,  left]
        b'|' => [true, false, true, false],
        b'-' => [false, true, false, true],
        b'L' => [true, true, false, false],
        b'J' => [true, false, false, true],
        b'7' => [false, false, true, true],
        b'F' => [false, true, true, false],
        _ => [false, false, false, false],
    }
}

pub fn p1(input: &str) -> u64 {
    let pipe_loop = fun_name(input);
    (pipe_loop.len() / 2) as u64
}

fn fun_name(input: &str) -> HashSet<(usize, usize)> {
    let mut start = (0, 0);
    // parse the map into a grid with a set of up, down, left, right flags indicating which exits are
    // possible from the tile, find the start tile along the way
    let graph = fun_name1(input, &mut start);
    // try each possible pipe symbol in turn and try to find the loop
    fun_name2(graph, start)
}

fn fun_name2(mut graph: Vec<Vec<[bool; 4]>>, start: (usize, usize)) -> HashSet<(usize, usize)> {
    let pipe_loop = "J|-L7F"
        .bytes()
        .find_map(|start_tile| {
            graph[start.0][start.1] = connections(start_tile);
            find_loop(&graph, start)
        })
        .unwrap();
    pipe_loop
}

fn fun_name1(input: &str, start: &mut (usize, usize)) -> Vec<Vec<[bool; 4]>> {
    let graph = input
        .split('\n')
        .enumerate()
        .map(|(r, line)| {
            line.bytes()
                .enumerate()
                .map(|(c, tile)| {
                    if tile == b'S' {
                        *start = (r, c);
                    }
                    connections(tile)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    graph
}

pub fn p2(input: &str) -> u64 {
    let mut start = (0, 0);
    // parse the map into a grid with a set of up, down, left, right flags indicating which exits are
    // possible from the tile, find the start tile along the way
    let mut graph = fun_name1(input, &mut start);
    // try each possible pipe symbol in turn and try to find the loop
    let pipe_loop = "J|-L7F".bytes().find_map(|start_tile| {
        graph[start.0][start.1] = connections(start_tile);
        find_loop(&graph, start)
      }).unwrap();
    let mut p2 = 0;
    for r in 0..graph.len() {
      let mut inside = false;
      for c in 0..graph[0].len() {
        if !pipe_loop.contains(&(r,c)) {
          p2 += inside as usize;
        } else if graph[r][c][0] {
          inside = !inside;
        }
      }
    }
    p2 as u64
}