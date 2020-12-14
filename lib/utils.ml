let uppercase_at idx str =
  let with_uppercase = function
    | i when i == idx -> Char.uppercase_ascii
    | _ -> Char.lowercase_ascii
  in
  String.mapi with_uppercase str
