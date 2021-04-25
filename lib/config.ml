open Toml

type t = {
  work_time : int;
  short_break_time : int;
  long_break_time : int;
  exec_on_start : string array;
  exec_on_break : string array;
  exec_on_resume : string array;
  exec_on_stop : string array;
}

let empty =
  {
    work_time = 25 * 60;
    short_break_time = 5 * 60;
    long_break_time = 15 * 60;
    exec_on_start = [||];
    exec_on_break = [||];
    exec_on_resume = [||];
    exec_on_stop = [||];
  }

let of_table key value config =
  let parse_int handle = function
    | Types.TInt n -> handle n
    | _ -> config
  in

  let parse_cmds handle = function
    | Types.TArray (Types.NodeString lst) -> handle (Array.of_list lst)
    | _ -> config
  in

  match Types.Table.Key.to_string key with
  | "work-time" ->
      let handle n = { config with work_time = n * 60 } in
      parse_int handle value
  | "short-break-time" ->
      let handle n = { config with short_break_time = n * 60 } in
      parse_int handle value
  | "long-break-time" ->
      let handle n = { config with long_break_time = n * 60 } in
      parse_int handle value
  | "exec-on-start" ->
      let handle cmds = { config with exec_on_start = cmds } in
      parse_cmds handle value
  | "exec-on-break" ->
      let handle cmds = { config with exec_on_break = cmds } in
      parse_cmds handle value
  | "exec-on-resume" ->
      let handle cmds = { config with exec_on_resume = cmds } in
      parse_cmds handle value
  | "exec-on-stop" ->
      let handle cmds = { config with exec_on_stop = cmds } in
      parse_cmds handle value
  | _ -> config

let read_file () =
  try
    match Parser.from_filename @@ Path.xdg_file "config.toml" with
    | `Ok table -> Types.Table.fold of_table table empty
    | `Error (_, _) -> empty
  with _ -> empty
