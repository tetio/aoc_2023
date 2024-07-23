import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  let input = read_input("input.txt")
  let lines = calculate_dimensions(input)

  let height = list.length(lines)
  let width = list.length(result.unwrap(list.last(lines), []))

  io.println("Part1 \n" <> "Width: " <> int.to_string(width))
  io.println("Part1 \n" <> "Height" <> int.to_string(height))

  io.debug("Original map")
  list.map(string.split(input, "\n"), fn(row) { io.println(row) })

  let adjusted_map =
    list.map(string.split(input, "\n"), fn(row) {
      case string.contains(does: row, contain: "#") {
        True -> [row]
        False -> [row, row]
      }
    })
    |> list.flatten
  let adjusted_height = list.length(adjusted_map)
  io.println("Part1: " <> "Adjusted height" <> int.to_string(adjusted_height))

  // io.debug(
  //   "Adjusted map has " <> int.to_string(list.length(adjusted_map)) <> "rows",
  // )

  // io.debug("adjusted_map\n")
  // list.map(adjusted_map, fn(e) { io.debug(e) })

  let tiles =
    list.flatten(list.map(adjusted_map, fn(row) { string.to_graphemes(row) }))

  // let column0 = get_column(tiles, width, 0)
  // io.debug("Col0 is")
  // io.debug(list.fold(column0, "", fn(a, b) { a <> b }))

  let reversed_tiles = make_columns(tiles, width, 0)
  // io.debug("Reversed tiles\n")
  // list.map(reversed_tiles, fn(e) {
  //   io.debug("" <> list.fold(e, "", fn(a, b) { a <> b }))
  // })
  let adjusted_reversed_tiles =
    list.map(reversed_tiles, fn(column) {
      case list.count(column, where: fn(e) { e == "#" }) != 0 {
        True -> [column]
        False -> [column, column]
      }
    })
    |> list.flatten
  let adjusted_width = list.length(adjusted_reversed_tiles)
  io.println("Part1: " <> "Adjusted width" <> int.to_string(adjusted_width))

  io.debug("adjusted_reversed_tiles tiles\n")
  list.map(adjusted_reversed_tiles, fn(e) {
    io.println("" <> list.fold(e, "", fn(a, b) { a <> b }))
  })

  // let adjusted_tiles = make_rows(adjusted_reversed_tiles, width, height, 0)
  // io.debug("adjusted_tiles:\n")
  // list.map(adjusted_tiles, fn(e) {
  //   io.debug("" <> list.fold(e, "", fn(a, b) { a <> b }))
  // })

  rotate_tiles(adjusted_reversed_tiles, adjusted_width)
}

fn get_column(tiles, width, column) -> List(String) {
  // i - column % with  == 0
  list.filter(
    list.index_map(tiles, fn(e, i) {
      let p = { i - column } % width
      case p {
        0 -> e
        _ -> ""
      }
    }),
    fn(n) { n != "" },
  )
}

fn make_columns(tiles, width, column) -> List(List(String)) {
  let p = column >= width
  case p {
    True -> []
    False ->
      list.prepend(
        make_columns(tiles, width, column + 1),
        get_column(tiles, width, column),
      )
  }
}

fn get_row(tiles: List(List(String)), height, row) -> List(String) {
  // i - row % with  == 0
  list.filter(
    list.index_map(tiles, fn(e, i) {
      let p = { i - row } % height
      case p {
        0 -> e
        _ -> []
      }
    }),
    fn(n) { n != [] },
  )
  |> list.flatten
}

fn make_rows(
  tiles: List(List(String)),
  width,
  height,
  row,
) -> List(List(String)) {
  case row >= width {
    True -> []
    False ->
      list.prepend(
        make_rows(tiles, height, height, row + 1),
        get_row(tiles, height, row),
      )
  }
}

fn string_sliced_as_array(input, width) -> List(String) {
  case string.is_empty(input) {
    False ->
      list.append(
        [string.slice(input, 0, width)],
        string_sliced_as_array(
          string.drop_left(from: input, up_to: width),
          width,
        )
      )
    True -> []
  }
}

fn rotate_tiles(tiles: List(List(String)), width) {
  let tiles = list.map(tiles, fn(l) { string.concat(l) }) |> string.concat
  io.debug("Tiles:" <> tiles)
  let r = string_sliced_as_array(tiles, width)
  io.println("r:")
  io.println(string.join(r, "\n"))
  r
}

fn read_input(file_name) -> String {
  let result = simplifile.read(file_name)
  case result {
    Ok(a) -> a
    Error(e) -> panic as simplifile.describe_error(e)
  }
}

fn calculate_dimensions(input) -> List(List(String)) {
  let row = string.split(input, "\n")
  list.map(row, fn(s) { string.to_graphemes(s) })
}
