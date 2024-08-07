// The Swift Programming Language
// https://docs.swift.org/swift-book
import Foundation

struct Location: Equatable {
    let x: Int
    let y: Int
    func south(_ endOfWorld: Int) -> Location? {
        return y < endOfWorld ? Location(x: x, y: y + 1) : nil
    }
    func north() -> Location? {
        return y > 0 ? Location(x: x, y: y - 1) : nil
    }
    func east(_ endOfWorld: Int) -> Location? {
        return x < endOfWorld ? Location(x: x + 1, y: y) : nil
    }
    func west() -> Location? {
        return x > 0 ? Location(x: x - 1, y: y) : nil
    }
}

enum Direction {
    case north
    case east
    case south
    case west
}

enum Tile: UInt8 {  // values are ASCII values for the map symbols
    case pipe = 124
    case dash = 45
    case L = 76
    case J = 74
    case seven = 55
    case F = 70
    case S = 83
    case dot = 46
}

struct Map {
    private let lines: Data
    private let startLocation: Location
    private let width: Int
    private let height: Int

    private func get(_ location: Location) -> Tile? {
        return Tile(rawValue: lines[location.y * (width + 1) + location.x])
    }

    /// Given a location and the direction we came from to get there, return the next location
    /// the pipe on that tile leads us to, if there is any.
    func nextLocation(iAmHere: Location, andCameFrom: Direction) -> (Location, Direction)? {
        guard let tile = get(iAmHere) else {
            return nil
        }
        let result: (Location, Direction)? =
            switch (tile, andCameFrom) {
            case (.pipe, .north), (.seven, .west), (.F, .east):
                iAmHere.south(height).map { ($0, .north) }
            case (.pipe, .south), (.L, .east), (.J, .west):
                iAmHere.north().map { ($0, .south) }
            case (.dash, .east), (.J, .north), (.seven, .south):
                iAmHere.west().map { ($0, .east) }
            case (.dash, .west), (.L, .north), (.F, .south):
                iAmHere.east(width).map { ($0, .west) }
            default:
                nil
            }

        // Check that we did not enter an unreachable tile.
        // The starting point is always reachable.
        return result.flatMap { (next, direction) in
            return switch direction {
            case .north where [.S, .pipe, .L, .J].contains(get(next)),
                .east where [.S, .dash, .L, .F].contains(get(next)),
                .south where [.S, .pipe, .seven, .F].contains(get(next)),
                .west where [.S, .dash, .seven, .J].contains(get(next)):
                (next, direction)
            default:
                nil
            }
        }
    }

    /// Return a list of locations connected to the given location, paired with the direction
    /// we would be coming from entering them.
    func connected(to location: Location) -> [(Location, Direction)] {
        var result: [(Location, Direction)] = []
        if let next = location.north(), [.S, .pipe, .F, .seven].contains(get(next)) {
            result.append((next, .south))
        }
        if let next = location.south(height), [.S, .pipe, .L, .J].contains(get(next)) {
            result.append((next, .north))
        }
        if let next = location.west(), [.S, .dash, .L, .F].contains(get(next)) {
            result.append((next, .east))
        }
        if let next = location.east(width), [.S, .dash, .J, .seven].contains(get(next)) {
            result.append((next, .west))
        }
        return result
    }

    /// Find a loop starting at and returning to the start location. Return the list of locations
    /// in the loop, or nil if no loop is found.
    func findLoop() -> [Location]? {
        let possibleStarts = connected(to: startLocation)
        for (next, direction) in possibleStarts {
            var visited: [Location] = [startLocation, next]
            var current = next
            var cameFrom = direction
            while let (next, direction) = nextLocation(iAmHere: current, andCameFrom: cameFrom) {
                visited.append(next)
                if next == startLocation {
                    return visited
                }
                current = next
                cameFrom = direction
            }
        }
        return nil
    }

    /// Construct a map from the input data. Data is assumed to be a sequence of fixed-width lines
    /// terminated by newline characters. The start location is assumed to be marked by an 'S' character.
    /// Pipes are represented by '|' (north/south), '-' (east/west), 'L' (north/east), 'F' (south/east),
    /// 'J' (north/west) and '7' (south/west). Tiles with no pipe are represented by the dot '.'.
    static func parse(_ lines: Data) -> Map? {
        guard let width = lines.firstIndex(of: UInt8(10)) else {
            return nil
        }
        let height = lines.count / (width + 1)
        guard let startpoint = lines.firstIndex(of: Tile.S.rawValue) else {
            return nil
        }
        let startLocation = Location(x: startpoint % (width + 1), y: startpoint / (width + 1))
        return Map(lines: lines, startLocation: startLocation, width: width, height: height)
    }
}

/// Given a closed path of points with integer coordinates, compute the number of points interior to the path.
/// THe last point on the path must be the same as the first point of the path.
func shoelace_with_picks_theorem(_ points: [Location]) -> Int {
    let n = points.count
    let area =
        abs(
            (0..<n - 1).reduce(0) {
                let i = $1
                let j = (i + 1)
                return $0 + (points[i].y + points[j].y) * (points[i].x - points[j].x)
            }) / 2

    // Pick's theorem: A = i + b/2 - 1, where i is the number of interior points,
    // A the area within the polygon and b is the number of boundary points.
    return area - (n - 1) / 2 + 1
}

/// AoC 2023 part 1: compute the distance to the tile in the loop farthest from the start location.
func p1(_ input: Data) -> Int {
    guard let map = Map.parse(input) else {
        print("Failed to parse input")
        return 0
    }
    return (map.findLoop()?.count ?? 0) / 2
}

func p2(_ input: Data) -> Int {
    guard let map = Map.parse(input) else {
        print("Failed to parse input")
        return 0
    }
    guard let loop = map.findLoop() else {
        print("No loop found")
        return 0
    }
    return shoelace_with_picks_theorem(loop)
}

func p2_from_file(filename: String) -> Int {
    guard let lines = try? Data(contentsOf: URL(fileURLWithPath: filename)) else {
        print("Failed to read file \(filename)")
        return 0
    }
    return p2(lines)
}

func p1_from_file(filename: String) -> Int {
    guard let lines = try? Data(contentsOf: URL(fileURLWithPath: filename)) else {
        print("Failed to read file \(filename)")
        return 0
    }
    return p1(lines)
}

// declare c abi to p1
@_cdecl("run_p1_swift")
public func p1Swift(_ a: UnsafeMutablePointer<UInt8>?, _ b: UInt64) -> Int {
    guard let a = a else {
        return 0
    }
    return p1(Data(bytesNoCopy: a, count: Int(b), deallocator: .none))
}

// declare c abi to p1
@_cdecl("run_p2_swift")
public func p2Swift(_ a: UnsafeMutablePointer<UInt8>?, _ b: UInt64) -> Int {
    guard let a = a else {
        return 0
    }
    return p2(Data(bytesNoCopy: a, count: Int(b), deallocator: .none))
}
