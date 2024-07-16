import XCTest

@testable import day_10

final class day_10Tests: XCTestCase {
  var day10: Day10!

  override func setUp() async throws {
    day10 = try Day10(path: "test1.txt")
  }

  func testProcessed001() throws {
    let processed = day10.floodFill(node: (0, 0))
    XCTAssertEqual(processed.count, 16, "Unexpected result")
  }

  func testExample() throws {
    // XCTest Documentation
    // https://developer.apple.com/documentation/xctest

    // Defining Test Cases and Test Methods
    // https://developer.apple.com/documentation/xctest/defining_test_cases_and_test_methods
  }
}
