let string_of_timer timer =
  let mins = timer / 60 in
  let secs = timer mod 60 in
  Format.sprintf "%.2d:%.2d" mins secs

let rec run = function
  | 0 -> ()
  | timer ->
    print_endline (string_of_timer timer);
    Unix.sleep 1;
    run (timer - 1)
