// The Swift Programming Language
// https://docs.swift.org/swift-book
import Foundation

struct Location {
  private(set) var x: Int
  private(set) var y: Int
}

extension Location {
  func south() -> Location? {
    return Location(x: x, y: y + 1)
  }
  func north() -> Location? {
    return y > 0 ? Location(x: x, y: y - 1) : nil
  }
  func east() -> Location? {
    return Location(x: x + 1, y: y)
  }
  func west() -> Location? {
    return x > 0 ? Location(x: x - 1, y: y) : nil
  }
}

extension Location: Equatable {
  static func == (lhs: Location, rhs: Location) -> Bool {
    return lhs.x == rhs.x && lhs.y == rhs.y
  }
}

extension Location: Hashable {
  func hash(into hasher: inout Hasher) {
    hasher.combine(x)
    hasher.combine(y)
  }
}

enum Direction: CaseIterable {
  case north
  case east 
  case south
  case west 
}

struct Map {
  private(set) var lines: Data
  private(set) var startLocaton: Location
  private(set) var width: Int
  private(set) var height: Int
}

extension Map {
  func get(_ location: Location) -> Tile {
    return Tile(rawValue: lines[location.y * (width + 1) + location.x])!
  }
}

enum Tile: UInt8 {
    case pipe = 124
    case dash = 45
    case L = 76
    case J = 74
    case seven = 55
    case F = 70
    case S = 83
    case dot = 46
}

extension Map {
  func nextLocation(from: Location, cameFrom: Direction) -> (Location, Direction)? {
    let tile = get(from)
    var result: (Location, Direction)?
    switch tile {
    case .pipe where cameFrom == .north:
      if let next = from.south() {
        result = (next, .north)
      }
    case .pipe where cameFrom == .south:
      if let next = from.north() {
        result = (next, .south)
      }
    case .dash where cameFrom == .east:
      if let next = from.west() {
        result = (next, .east)
      }
    case .dash where cameFrom == .west:
      if let next = from.east() {
        result = (next, .west)
      }
    case .L where cameFrom == .north:
      if let next = from.east() {
        result = (next, .west)
      }
    case .L where cameFrom == .east:
      if let next = from.north() {
        result = (next, .south)
      }
    case .J where cameFrom == .north:
      if let next = from.west() {
        result = (next, .east)
      }
    case .J where cameFrom == .west:
      if let next = from.north() {
        result = (next, .south)
      }
    case .seven where cameFrom == .south:
      if let next = from.west() {
        result = (next, .east)
      }
    case .seven where cameFrom == .west:
      if let next = from.south() {
        result = (next, .north)
      }
    case .F where cameFrom == .south:
      if let next = from.east() {
        result = (next, .west)
      }
    case .F where cameFrom == .east:
      if let next = from.south() {
        result = (next, .north)
      }
    default:
      do {}
    }

    if let (next, direction) = result, next.x < width && next.y < height {
      switch direction {
      case .north where [.S, .pipe, .L, .J].contains(get(next)),
        .east where [.S, .dash, .L, .F].contains(get(next)),
        .south where [.S, .pipe, .seven, .F].contains(get(next)),
        .west where [.S, .dash, .seven, .J].contains(get(next)):
        return (next, direction)
      default:
        do {}
      }
    }
    return nil
  }

}

extension Map {
  func connected(to location: Location) -> [(Location, Direction)] {
    var result: [(Location, Direction)] = []
    if let next = location.north(), [.S, .pipe, .F, .seven].contains(get(next)) {
      result.append((next, .south))
    }
    if let next = location.south(), [.S, .pipe, .L, .J].contains(get(next)) {
      result.append((next, .north))
    }
    if let next = location.west(), [.S, .dash, .L, .F].contains(get(next)) {
      result.append((next, .east))
    }
    if let next = location.east(), [.S, .dash, .J, .seven].contains(get(next)) {
      result.append((next, .west))
    }
    return result
  }
}

extension Map {
    func findLoop() -> [Location]? {
        let possible_starts = connected(to: startLocaton)
        for (next, direction) in possible_starts {
            var visited: Array<Location> = [startLocaton, next]
            var current = next
            var cameFrom = direction
            while let (next, direction) = nextLocation(from: current, cameFrom: cameFrom) {
                if visited.contains(next) {
                    return visited
                }
                visited.append(next)
                current = next
                cameFrom = direction
            }
        }
        return nil
    }
}

func parse(_ lines: Data) -> Map {
  let width = lines.firstIndex(of: UInt8(10))!
  let height = lines.count / (width + 1)
  let startpoint = lines.firstIndex(of: UInt8(83))!
  let startLocaton = Location(x: startpoint % (width + 1), y: startpoint / (width + 1))
  return Map(lines: lines, startLocaton: startLocaton, width: width, height: height)
}

func p1(_ input: Data) -> Int {
  let map = parse(input)
  return (map.findLoop()?.count ?? 0 ) / 2
}

func p1_from_file(filename: String) -> Int {
  let lines = try! Data(contentsOf: URL(fileURLWithPath: filename))
  let map = parse(lines)
  return (map.findLoop()?.count ?? 0 ) / 2
}

// declare c abi to p1
@_cdecl("p1")
public func p1(_ a: UnsafeMutablePointer<UInt8>?, _ b: Int) -> Int {
  return p1(Data(bytesNoCopy: a!, count: b, deallocator: .none))
}   
