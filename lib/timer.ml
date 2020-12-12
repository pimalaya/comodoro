type t = Work of int * int | ShortBreak of int * int | LongBreak of int * int

let min = 60

let workTime = 25 * min

let shortBreakTime = 5 * min

let longBreakTime = 15 * min

let int_of_timer = function
  | Work (n, _) -> n
  | ShortBreak (n, _) -> n
  | LongBreak (n, _) -> n

let to_string timer =
  let mins = int_of_timer timer / 60 in
  let secs = int_of_timer timer mod 60 in
  let symbol =
    match timer with
    | Work _ -> "WO"
    | ShortBreak _ -> "SB"
    | LongBreak _ -> "LB"
  in
  Format.sprintf "%.2d:%.2d [%s]" mins secs symbol

let next = function
  | Work (1, n) when n mod 6 == 4 -> LongBreak (shortBreakTime, n + 1)
  | Work (1, n) -> ShortBreak (shortBreakTime, n + 1)
  | Work (t, n) -> Work (t - 1, n)
  | ShortBreak (1, n) -> Work (workTime, n + 1)
  | ShortBreak (t, n) -> ShortBreak (t - 1, n)
  | LongBreak (1, n) -> Work (workTime, n + 1)
  | LongBreak (t, n) -> LongBreak (t - 1, n)

let rec run handler timer =
  let timer_str = to_string timer in
  handler timer_str;
  print_endline timer_str;
  Unix.sleep 1;
  run handler @@ next timer

let start handler = run handler @@ Work (workTime, 0)
