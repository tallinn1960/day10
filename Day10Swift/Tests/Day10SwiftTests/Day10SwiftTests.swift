import XCTest

@testable import Day10Swift

final class Day10SwiftTests: XCTestCase {
  func test_next_location_dash() {
    let map = Map.parse(
        """
        .S-7.
        .|.|.
        .L-J.
        """.data(using: .utf8)!
    )!
    var (next, direction) = map.nextLocation(from: Location(x: 2, y: 0), cameFrom: .west)!
    XCTAssertEqual(next, Location(x: 3, y: 0))
    XCTAssertEqual(direction, .west)
    (next, direction) = map.nextLocation(from: Location(x: 2, y: 0), cameFrom: .east)!
    XCTAssertEqual(next, Location(x: 1, y: 0))
    XCTAssertEqual(direction, .east)
  }

  func test_next_location_J() {
    let map = Map.parse(
        """
        .S-7.
        .|.|.
        .L-J.
        """.data(using: .utf8)!
    )!
    var (next, direction) = map.nextLocation(from: Location(x: 3, y: 2), cameFrom: .north)!
    XCTAssertEqual(next, Location(x: 2, y: 2))
    XCTAssertEqual(direction, .east)
    (next, direction) = map.nextLocation(from: Location(x: 3, y: 2), cameFrom: .west)!
    XCTAssertEqual(next, Location(x: 3, y: 1))
    XCTAssertEqual(direction, .south)
  }

  func test_next_location_L() {
     let map = Map.parse(
        """
        .S-7.
        .|.|.
        .L-J.
        """.data(using: .utf8)!
    )!
    var (next, direction) = map.nextLocation(from: Location(x: 1, y: 2), cameFrom: .north)!
    XCTAssertEqual(next, Location(x: 2, y: 2))
    XCTAssertEqual(direction, .west)
    (next, direction) = map.nextLocation(from: Location(x: 1, y: 2), cameFrom: .east)!
    XCTAssertEqual(next, Location(x: 1, y: 1))
    XCTAssertEqual(direction, .south)
  }

  func test_next_location_pipe() {
    let map = Map.parse(
        """
        .S-7.
        .|.|.
        .L-J.
        """.data(using: .utf8)!
    )!
    var (next, direction) = map.nextLocation(from: Location(x: 1, y: 1), cameFrom: .north)!
    XCTAssertEqual(next, Location(x: 1, y: 2))
    XCTAssertEqual(direction, .north)
    (next, direction) = map.nextLocation(from: Location(x: 1, y: 1), cameFrom: .south)!
    XCTAssertEqual(next, Location(x: 1, y: 0))
    XCTAssertEqual(direction, .south)
  }

  func test_next_location_seven() {
    let map = Map.parse(
        """
        .S-7.
        .|.|.
        .L-J.
        """.data(using: .utf8)!
    )!
    var (next, direction) = map.nextLocation(from: Location(x: 3, y: 0), cameFrom: .west)!
    XCTAssertEqual(next, Location(x: 3, y: 1))
    XCTAssertEqual(direction, .north)
    (next, direction) = map.nextLocation(from: Location(x: 3, y: 0), cameFrom: .south)!
    XCTAssertEqual(next, Location(x: 2, y: 0))
    XCTAssertEqual(direction, .east)
  }

  func test_next_location_F() {
    let map = Map.parse(
        """
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
        """.data(using: .utf8)!
    )!
    var (next, direction) = map.nextLocation(from: Location(x: 2, y: 0), cameFrom: .south)!
    XCTAssertEqual(next, Location(x: 3, y: 0))
    XCTAssertEqual(direction, .west)
    (next, direction) = map.nextLocation(from: Location(x: 2, y: 0), cameFrom: .east)!
    XCTAssertEqual(next, Location(x: 2, y: 1))
    XCTAssertEqual(direction, .north)
  }

  func test_connected_to() {
    let map = Map.parse(
        """
        .S-7.
        .|.|.
        .L-J.
        """.data(using: .utf8)!
    )!
    let connected = map.connected(to: Location(x: 1, y: 0))
    XCTAssertEqual(connected.count, 2)
    XCTAssertTrue(connected.contains { $0.0 == Location(x: 1, y: 1) && $0.1 == .north })
    XCTAssertTrue(connected.contains { $0.0 == Location(x: 2, y: 0) && $0.1 == .west })
  }

  func test_find_loop() {
    let map = Map.parse(
        """
        .S-7.
        .|.|.
        .L-J.
        """.data(using: .utf8)!
    )!
    let loop = map.findLoop()
    XCTAssertEqual(loop?.count, 8)
    XCTAssertEqual(
      loop,
      [
        Location(x: 1, y: 0),
        Location(x: 1, y: 1),
        Location(x: 1, y: 2),
        Location(x: 2, y: 2),
        Location(x: 3, y: 2),
        Location(x: 3, y: 1),
        Location(x: 3, y: 0),
        Location(x: 2, y: 0),
      ])
  }

  func test_find_loop2() {
    let map = Map.parse(
        """
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
        """.data(using: .utf8)!
    )!
    let loop = map.findLoop()
    XCTAssertEqual(loop?.count, 16)
  }
  
  func test_part1() {
    let lines = try! Data(contentsOf: URL(fileURLWithPath: "../input.txt"))

    let map = Map.parse(lines)!
    let count = (map.findLoop()?.count ?? 0) / 2
    XCTAssertEqual(count, 6778)
  }
}

final class PerfomanceTests: XCTestCase {
  func test_part1() {
    let lines = try! Data(contentsOf: URL(fileURLWithPath: "../input.txt"))
    let map = Map.parse(lines)!
    measure(metrics: [XCTClockMetric()]) {
      let count = (map.findLoop()?.count ?? 0) / 2
      XCTAssertEqual(count, 6778)
    }
  }
}