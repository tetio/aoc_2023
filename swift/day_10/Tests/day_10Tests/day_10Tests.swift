import XCTest

@testable import day_10

final class day_10Tests: XCTestCase {

  override func setUp() async throws {
    
  }

  func testProcessed001() throws {
    let day10 = try Day10(path: "test1.txt")
    let processed = day10.floodFill(node: (0, 0))
    XCTAssertEqual(processed.count, 16, "Unexpected result")
  }


}
