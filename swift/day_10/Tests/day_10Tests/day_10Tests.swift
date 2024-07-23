import XCTest

@testable import day_10

final class day_10Tests: XCTestCase {

  override func setUp() async throws {
    
  }

  func testFloodFill001() throws {
    let day10 = try Day10(path: "test1.txt")
    let processed = day10.floodFill(node: (0, 0))
    XCTAssertEqual(processed.count, 16, "Unexpected result")
  }

  func testPart1_001() throws {
    let day10 = try Day10(path: "test3.txt")
    let distance = day10.part1()
    XCTAssertEqual(distance, 14, "Unexpected result")
  }
  //shoelaceAlgorithm
  func testShoelaceAlgorithm_001() throws {
    let day10 = try Day10(path: "test3.txt")
    let distance = day10.shoelaceAlgorithm()
    XCTAssertEqual(distance, 4, "Unexpected result")
  }

    func testShoelaceAlgorithm_002() throws {
    let day10 = try Day10(path: "test2.txt")
    let distance = day10.shoelaceAlgorithm()
    XCTAssertEqual(distance, 4, "Unexpected result")
  }
}
