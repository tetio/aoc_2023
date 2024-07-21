import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  let input = read_input("input.txt")
  let lines = calculate_dimensions(input)
  io.println(
    "Part1 \n"
    <> "Width: "
    <> int.to_string(list.length(result.unwrap(list.last(lines), []))),
  )
  io.println("Part1 \n" <> "Height" <> int.to_string(list.length(lines)))

  let map = list.map(lines, fn(s) { string.to_graphemes(s) })
  let adjusted_map = list.map(map, fn(row) {
    case string.contains(does: row, contain: "#") {
      True -> [row]
      False -> [row, row]
    }
    }) |> list.flatten
    io.debug("Adjusted map has " <> int.to_string(list.length(adjusted_map)) <> "rows")
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

