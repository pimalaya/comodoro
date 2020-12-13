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

let exec_hooks (config : Config.t) timer =
  let exec = Array.iter Process.exec_silent in
  match timer with
  | Work (t, n) when t == workTime && n > 0 -> exec config.exec_on_resume
  | ShortBreak (t, _) when t == shortBreakTime -> exec config.exec_on_break
  | LongBreak (t, _) when t == shortBreakTime -> exec config.exec_on_break
  | _ -> ()

let rec run handle timer =
  let config = Config.read_file () in
  let timer_str = to_string timer in
  handle timer_str;
  print_endline timer_str;
  exec_hooks config timer;
  Unix.sleep 1;
  run handle @@ next timer

let start handler = run handler @@ Work (workTime, 0)
