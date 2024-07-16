import Foundation

enum Day10Error: Error {
  case invalidTile
  case invalidDirection
  case invalidInputFile
  case noStartingTileFound
  case invalidPosition
  case invalidReversePosition
}

@main
struct Day10 {
  private var tiles: [[Character]]!
  private var shape: [(Int, Int)]!
  private var width: Int!
  private var height: Int!

  init(path: String) throws {
    let contents = try readDataFile(path: path)
    self.tiles = contents.components(separatedBy: "\n").map { Array($0) }
    self.width = self.tiles[0].count
    self.height = self.tiles.count
    self.shape = try calculateShape(input: contents)
  }

  public func part1() -> Int {
    self.shape.count / 2
  }

  func readDataFile(path: String) throws -> String {
    do {
      let contents = try String(contentsOfFile: path, encoding: .utf8)
      return contents
    } catch {
      throw Day10Error.invalidInputFile
    }
  }

  func calculateShape(input: String) throws -> [(Int, Int)] {
    let startingPosition = try whereIsStartPoint()
    var currentPosition = startingPosition
    var shape: [(Int, Int)] = []
    var first = true
    while currentPosition != startingPosition || first {
      first = false
      currentPosition = try nextMove(position: currentPosition, res: shape)
      shape.append(currentPosition)
    }
    return shape
  }

  func nextMove(position: (Int, Int), res: [(Int, Int)]) throws -> (Int, Int) {
    let currentTile = tileAt(position: position)
    let destinations = try getConnections(tile: currentTile)
    let validDestination =
      try destinations
      .filter {
        try isValidMove(orientation: $0, currentPosition: position)
      }
      .filter({
        let direction = try getDirection(orientation: $0)
        let nextPosition = addDirection(position: position, direction: direction)
        return !res.contains(where: { $0 == nextPosition })
      })
    let direction = try getDirection(orientation: validDestination[0])
    return addDirection(position: position, direction: direction)
  }

  func tileAt(position: (Int, Int)) -> Character {
    return tiles[position.1][position.0]
  }

  func whereIsStartPoint() throws -> (Int, Int) {
    let width = self.tiles[0].count
    let height = self.tiles.count
    for j in 0..<height {
      for i in 0..<width {
        if self.tiles[j][i] == "S" {
          return (i, j)
        }
      }
    }
    throw Day10Error.noStartingTileFound
  }

  func getConnections(tile: Character) throws -> [Character] {
    switch tile {
    case "|": return ["n", "s"]
    case "-": return ["e", "w"]
    case "L": return ["e", "n"]
    case "J": return ["w", "n"]
    case "7": return ["w", "s"]
    case "F": return ["e", "s"]
    case "S": return ["n", "w", "e", "s"]
    default: throw Day10Error.invalidTile
    }
  }
  func addDirection(position: (Int, Int), direction: (Int, Int)) -> (Int, Int) {
    return (position.0 + direction.0, position.1 + direction.1)
  }

  func isValidMove(orientation: Character, currentPosition: (Int, Int)) throws -> Bool {
    let dir = try getDirection(orientation: orientation)
    if isValidPosition(currentPosition: currentPosition, direction: dir) {
      return try getConnections(
        tile: tileAt(position: addDirection(position: currentPosition, direction: dir))
      )
      .filter { try reverseConnection(connection: $0) == orientation }.count > 0
    }
    return false
  }

  func getDirection(orientation: Character) throws -> (Int, Int) {
    switch orientation {
    case "n": return (0, -1)
    case "s": return (0, 1)
    case "e": return (1, 0)
    case "w": return (-1, 0)
    default: throw Day10Error.invalidDirection
    }
  }

  func isValidPosition(currentPosition: (Int, Int), direction: (Int, Int)) -> Bool {
    let newPosition = addDirection(position: currentPosition, direction: direction)
    return isPositionInsideMap(position: newPosition)
  }

  func isPositionInsideMap(position: (Int, Int)) -> Bool {
    return position.0 >= 0 && position.0 < width && position.1 >= 0 && position.1 < height
  }

  func reverseConnection(connection: Character) throws -> Character {
    switch connection {
    case "n": return "s"
    case "s": return "n"
    case "e": return "w"
    case "w": return "e"
    default: throw Day10Error.invalidReversePosition
    }
  }

  func floodFill(node: (Int, Int)) -> [(Int, Int)] {
    var queue: [(Int, Int)] = []
    var processed: [(Int, Int)] = []
    queue.append(node)
    while !queue.isEmpty {
      let n = queue.popLast()!
      if !processed.contains(where: { $0 == n }) && !self.shape.contains(where: { $0 == n }) {
        processed.append(n)
        if n.1 > 0 && !shape.contains(where: { $0 == (n.0, n.1 - 1) }) {
          queue.append((n.0, n.1 - 1))
        }
        if n.0 > 0 && !shape.contains(where: { $0 == (n.0 - 1, n.1) }) {
          queue.append((n.0 - 1, n.1))
        }
        if n.1 < height - 1 && !shape.contains(where: { $0 == (n.0, n.1 + 1) }) {
          queue.append((n.0, n.1 + 1))
        }
        if n.0 < width - 1 && !shape.contains(where: { $0 == (n.0 + 1, n.1) }) {
          queue.append((n.0 + 1, n.1))
        }
      }
    }
    return processed
  }

  func isOutsider(positions: [(Int, Int)]) -> Bool {
    positions.filter { $0.0 == 0 || $0.1 == 0 || $0.0 == width - 1 || $0.1 == height - 1 }.count > 0
  }

  //   func findAllInsiders() -> [(Int, Int)] {
  //     var candidates: [(Int, Int)] = []
  //     for i in 0..<width {
  //       for j in 0..<height {
  //         if !shape.contains(where: {$0 == (i, j)}) {
  //           candidates.append((i, j))
  //         }
  //       }
  //     }
  //     let res = candidates.map {!isOutsider(candidates: $0)}
  //     (0, 0)
  //   }

  static func main() throws {
    // let day10 = try Day10(path: "input.txt")

    // var numbers = [1, 2, 3, 4, 34, 67, 5, 23, 12]
    // numbers.append(contentsOf: [100, 101, 102])
    // numbers.append(contentsOf: [1020, 1012, 1022])
    // let even = numbers.filter { $0 % 2 == 0 }
    // let odd = numbers.filter { $0 % 2 != 0 }
    // let sum = numbers.reduce(0, +)
    // print("Total numbers = \(numbers.count)")
    // print("Total even numbers = \(even.count)")
    // print("Total odd numbers = \(odd.count)")
    // print("Value of sum = \(sum)")
    // print("Where is the starting point: \(try day10.whereIsStartPoint())")
    // print("Part1 result is \(day10.part1())")

    let day10A = try Day10(path: "test1.txt")
    let outside = day10A.floodFill(node: (0, 0))
    print("flood fill outside result has \(outside.count) tiles")
    print("flood fill outside result is \(outside) ")
    let inside = day10A.floodFill(node: (2, 2))
    print("flood fill inside result has \(inside.count) tiles")
    print("flood fill inside result is \(inside) ")

  }

}
