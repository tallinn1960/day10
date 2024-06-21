// The Swift Programming Language
// https://docs.swift.org/swift-book
import Foundation

struct Location: Equatable, Hashable {
  let x: Int
  let y: Int
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
  static func == (lhs: Location, rhs: Location) -> Bool {
    return lhs.x == rhs.x && lhs.y == rhs.y
  }
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
  let lines: Data
  let startLocaton: Location
  let width: Int
  let height: Int

  private func get(_ location: Location) -> Tile? {
    guard location.x < width && location.y < height else {
      return nil
    }
    return Tile(rawValue: lines[location.y * (width + 1) + location.x])
  }

  func nextLocation(from: Location, cameFrom: Direction) -> (Location, Direction)? {
    guard let tile = get(from) else {
      return nil
    }
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

  func findLoop() -> [Location]? {
    let possible_starts = connected(to: startLocaton)
    for (next, direction) in possible_starts {
      var visited: [Location] = [startLocaton, next]
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

func parse(_ lines: Data) -> Map? {
  guard let width = lines.firstIndex(of: UInt8(10)) else {
    return nil
  }
  let height = lines.count / (width + 1)
  guard let startpoint = lines.firstIndex(of: UInt8(83)) else {
    return nil
  }
  let startLocaton = Location(x: startpoint % (width + 1), y: startpoint / (width + 1))
  return Map(lines: lines, startLocaton: startLocaton, width: width, height: height)
}

func p1(_ input: Data) -> Int {
  guard let map = parse(input) else {
    print("Failed to parse input")
    return 0
  }
  return (map.findLoop()?.count ?? 0) / 2
}

func p1_from_file(filename: String) -> Int {
  guard let lines = try? Data(contentsOf: URL(fileURLWithPath: filename)) else {
    print("Failed to read file \(filename)")
    return 0
  }
  guard let map = parse(lines) else {
    print("Failed to parse file \(filename)")
    return 0
  }
  return (map.findLoop()?.count ?? 0) / 2
}

// declare c abi to p1
@_cdecl("p1")
public func p1(_ a: UnsafeMutablePointer<UInt8>?, _ b: Int) -> Int {
  guard let a = a else {
    return 0
  }
  return p1(Data(bytesNoCopy: a, count: b, deallocator: .none))
}
