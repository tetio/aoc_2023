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
    shape.append(currentPosition)
    var first = true
    while currentPosition != startingPosition || first {
      first = false
      currentPosition = try nextMove(position: currentPosition, res: shape)
      if currentPosition != (-1, -1){
        shape.append(currentPosition)
      } else {
        currentPosition = startingPosition
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
      return (-1, -1) 
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

  func isOutsider(positions: [(Int, Int)]) -> Bool {
    positions.filter { $0.0 == 0 || $0.1 == 0 || $0.0 == width - 1 || $0.1 == height - 1 }.count > 0
  }

  func findAllInsiders() -> [(Int, Int)] {
    var candidates: [(Int, Int)] = []
    var insiders: [(Int, Int)] = []
    for i in 0..<width {
      for j in 0..<height {
        if !shape.contains(where: { $0 == (i, j) }) {
          candidates.append((i, j))
        }
      }
    }
    //return candidates.map {floodFill(node: $0)}.filter {!isOutsider(positions: $0)}.flatMap{$0}.filter {$0}
    for candidate in candidates {
      let ff = floodFill(node: candidate)
      if !isOutsider(positions: ff) {
        for insider in ff {
          if !insiders.contains(where: { $0 == insider }) {
            insiders.append(insider)
            candidates.removeAll(where: { $0 == insider })
          }
        }
      } else {
        candidates = candidates.filter { candidate in
          return !ff.contains(where: { candidate == $0 })
        }
      }
    }
    return insiders
  }

  func nortRayHasOddIntersections(_ position: (Int, Int)) -> Bool {
    var candidates: [(Int, Int)] = []
    for y in 0...position.1 - 1 {
      candidates.append((position.0, y))
    }
    let ray =
      candidates
      .map { r in
        if shape.contains(where: { $0 == r }) {
          return tileAt(position: r)
        } else {
          return "."
        }
      }
    return countHits(ray: ray) % 2 != 0
  }

  func southRayOddIntersections(_ position: (Int, Int)) -> Bool {
    var candidates: [(Int, Int)] = []
    for y in position.1 + 1...height - 1 {
      candidates.append((position.0, y))
    }
    let ray =
      candidates
      .map { r in
        if shape.contains(where: { $0 == r }) {
          return tileAt(position: r)
        } else {
          return "."
        }
      }
    return countHits(ray: ray) % 2 != 0
  }

  func westRayOddIntersections(_ position: (Int, Int)) -> Bool {
    var candidates: [(Int, Int)] = []
    for x in 0...position.0 - 1 {
      candidates.append((x, position.1))
    }
    let ray =
      candidates
      .map { r in
        if shape.contains(where: { $0 == r }) {
          return tileAt(position: r)
        } else {
          return "."
        }
      }
    return countHits(ray: ray) % 2 != 0

  }

  func eastRayOddIntersections(_ position: (Int, Int)) -> Bool {
    var candidates: [(Int, Int)] = []
    for x in position.0 + 1...width - 1 {
      candidates.append((x, position.1))
    }
    let ray =
      candidates
      .map { r in
        if shape.contains(where: { $0 == r }) {
          return tileAt(position: r)
        } else {
          return "."
        }
      }
    return countHits(ray: ray) % 2 != 0
  }

  func countHits(ray: [Character]) -> Int {
    var hits = 0
    if ray.count > 0 {
      var i = 0
      while i < ray.count {
        if isPipe(ray[i]) {
          var j = i
          while j < ray.count && isPipe(ray[j]) {
            i += 1
            j += 1
          }
          hits += 1
        }
        i += 1
      }
    }
    return hits
  }

  func findAllInsidersRaycasting() -> [(Int, Int)] {
    var candidates: [(Int, Int)] = []
    var insiders: [(Int, Int)] = []
    for i in 0..<width {
      for j in 0..<height {
        if !shape.contains(where: { $0 == (i, j) }) {
          candidates.append((i, j))
        }
      }
    }
    //return candidates.map {floodFill(node: $0)}.filter {!isOutsider(positions: $0)}.flatMap{$0}.filter {$0}
    for candidate in candidates {

      let n = nortRayHasOddIntersections(candidate)
      let s = southRayOddIntersections(candidate)
      let w = westRayOddIntersections(candidate)
      let e = eastRayOddIntersections(candidate)
      /*
      if nortRayHasOddIntersections(candidate)
      && southRayOddIntersections(candidate)
      && westRayOddIntersections(candidate)
      && eastRayOddIntersections(candidate) {
*/
      if s && n && e && w {
        insiders.append(candidate)
      }
    }
    return insiders
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
    let insiders = day10.findAllInsidersRaycasting()
    let endTime = Date()
    let diff = endTime.timeIntervalSince1970 - initTime.timeIntervalSince1970
    print("Part2's result is \(insiders.count) and took \(diff) seconds")
  }

}
