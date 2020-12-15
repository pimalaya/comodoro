open Toml

type t = {
  exec_on_start : string array;
  exec_on_break : string array;
  exec_on_resume : string array;
  exec_on_stop : string array;
}

let empty =
  {
    exec_on_start = [||];
    exec_on_break = [||];
    exec_on_resume = [||];
    exec_on_stop = [||];
  }

let of_table key value config =
  let parse_cmds handle = function
    | TomlTypes.TArray (TomlTypes.NodeString lst) -> handle (Array.of_list lst)
    | _ -> config
  in

  match TomlTypes.Table.Key.to_string key with
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
    | `Ok table -> TomlTypes.Table.fold of_table table empty
    | `Error (_, _) -> empty
  with _ -> empty
