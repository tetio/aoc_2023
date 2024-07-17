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

  func testProcessed002() throws {
    let day10 = try Day10(path: "test1.txt")
    let processed = day10.findAllInsidersRaycasting()
    XCTAssertEqual(processed.count, 1, "Unexpected result")
  }

  func testProcessed003() throws {
    let day10 = try Day10(path: "test2.txt")
    let processed = day10.findAllInsidersRaycasting()
    XCTAssertEqual(processed.count, 4, "Unexpected result")
  }

  func testProcessed004() throws {
    let day10 = try Day10(path: "test3.txt")
    let processed = day10.findAllInsidersRaycasting()
    XCTAssertEqual(processed.count, 4, "Unexpected result")
  }

  func testProcessed0041() throws {
    let day10 = try Day10(path: "test4.txt")
    let processed = day10.findAllInsidersRaycasting()
    XCTAssertEqual(processed.count, 4, "Unexpected result")
  }



  func testProcessed005() throws {
    let day10 = try Day10(path: "input2.txt")
    let processed = day10.findAllInsidersRaycasting()
    XCTAssertEqual(processed.count, 10, "Unexpected result")
  }


  func testProcessed005b() throws {
    let day10 = try Day10(path: "input2.txt")
    //let processed = day10.findAllInsidersRaycasting()
      let candidate = (14,3)
      let n = day10.nortRayHasOddIntersections(candidate) 
      let s = day10.southRayOddIntersections(candidate)
      let w = day10.westRayOddIntersections(candidate)
      let e = day10.eastRayOddIntersections(candidate)
    XCTAssertEqual(10, 10, "Unexpected result")
  }


  func testCountHits001() throws {
    let day10 = try Day10(path: "input2.txt")
    let hits = day10.countHits(ray: [".", ".", ".", "L", "."])
    XCTAssertEqual(hits, 1, "Unexpected result")
  }

  func testCountHits002() throws {
    let day10 = try Day10(path: "input2.txt")
    let hits = day10.countHits(ray: [".", ".", "J","L", "."])
    XCTAssertEqual(hits, 1, "Unexpected result")
  }


  func testCountHits003() throws {
    let day10 = try Day10(path: "input2.txt")
    let hits = day10.countHits(ray: ["F", ".", "J","L", ".", "L", "."])
    XCTAssertEqual(hits, 3, "Unexpected result")
  }


  func testCountHits004() throws {
    let day10 = try Day10(path: "input2.txt")
    let hits = day10.countHits(ray: ["F", ".", "J","L", ".", "L", ".", "7"])
    XCTAssertEqual(hits, 4, "Unexpected result")
  }


  func testCountHits005() throws {
    let day10 = try Day10(path: "input2.txt")
    let hits = day10.countHits(ray: ["F", ".", "J","L", ".", "L", ".", "7", "L"])
    XCTAssertEqual(hits, 4, "Unexpected result")
  }


  func testCountHits006() throws {
    let day10 = try Day10(path: "input2.txt")
    let hits = day10.countHits(ray: ["F", "J","L", "L", "7", "L"])
    XCTAssertEqual(hits, 1, "Unexpected result")
  }


  func testCountHits007() throws {
    let day10 = try Day10(path: "input2.txt")
    let hits = day10.countHits(ray: ["F", "J", ".", "L", "7", "L"])
    XCTAssertEqual(hits, 2, "Unexpected result")
  }

}
