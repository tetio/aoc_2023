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
  let EXIT_POSITION = (-1, -1)

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
    shape.append(currentPosition)
    var first = true
    shape.append(startingPosition)
    while (currentPosition != startingPosition || first) && currentPosition != EXIT_POSITION {
      first = false
      currentPosition = try nextMove(position: currentPosition, res: shape)
      if currentPosition != EXIT_POSITION {
        shape.append(currentPosition)
      }
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
    if validDestination.isEmpty {
      return EXIT_POSITION
    }
    let direction = try getDirection(orientation: validDestination[0])
    return addDirection(position: position, direction: direction)
  }

  func tileAt(position: (Int, Int)) -> Character {
    return tiles[position.1][position.0]
  }

  func whereIsStartPoint() throws -> (Int, Int) {
    for j in 0..<height {
      for i in 0..<width {
        if tileAt(position: (i, j)) == "S" {
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

  func isPipe(_ tile: Character) -> Bool {
    switch tile {
    case "|": return true
    case "-": return true
    case "L": return true
    case "J": return true
    case "7": return true
    case "F": return true
    case "S": return true
    default: return false
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

  static func main() throws {
    let day10A = try Day10(path: "test1.txt")
    let outside = day10A.floodFill(node: (0, 0))
    print("flood fill outside result has \(outside.count) tiles")
    print("flood fill outside result is \(outside) ")
    let inside = day10A.floodFill(node: (2, 2))
    print("flood fill inside result has \(inside.count) tiles")
    print("flood fill inside result is \(inside) ")

    let day10 = try Day10(path: "input.txt")
    let initTime = Date()
    //let insiders = day10.findAllInsiders()
    let distance = day10.part1()
    let endTime = Date()
    let diff = endTime.timeIntervalSince1970 - initTime.timeIntervalSince1970
    print("Part1's result is \(distance) and took \(diff) seconds")
  }

}
