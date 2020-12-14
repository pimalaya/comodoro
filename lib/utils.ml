let lift_str str =
  let last_char = str.[String.length str - 1]
  and sub_str = String.sub str 0 (String.length str - 1) in
  Printf.sprintf "%c%s" last_char sub_str
