type t = Work of int * int | ShortBreak of int * int | LongBreak of int * int

let initial_timer (config : Config.t) = Work (config.work_time, 0)

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

let next (config : Config.t) = function
  | Work (1, n) when n mod 6 == 4 -> LongBreak (config.long_break_time, n + 1)
  | Work (1, n) -> ShortBreak (config.short_break_time, n + 1)
  | Work (t, n) -> Work (t - 1, n)
  | ShortBreak (1, n) -> Work (config.work_time, n + 1)
  | ShortBreak (t, n) -> ShortBreak (t - 1, n)
  | LongBreak (1, n) -> Work (config.work_time, n + 1)
  | LongBreak (t, n) -> LongBreak (t - 1, n)

let exec_hooks (config : Config.t) = function
  | Work (t, n) when t == config.work_time && n > 0 ->
      Process.exec_silent_all config.exec_on_resume
  | ShortBreak (t, _) when t == config.short_break_time ->
      Process.exec_silent_all config.exec_on_break
  | LongBreak (t, _) when t == config.long_break_time ->
      Process.exec_silent_all config.exec_on_break
  | _ -> ()

let rec run config timer handle =
  let timer_str = to_string timer in
  let next_timer = next config timer in
  handle timer_str;
  print_endline timer_str;
  exec_hooks config timer;
  Unix.sleep 1;
  run config next_timer handle
