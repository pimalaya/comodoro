open Toml

type t = { exec_on_break : string array; exec_on_resume : string array }

let empty = { exec_on_break = [||]; exec_on_resume = [||] }

let of_table key value config =
  match TomlTypes.Table.Key.to_string key with
  | "exec-on-break" -> (
      match value with
      | TomlTypes.TArray (TomlTypes.NodeString arr) ->
          { config with exec_on_break = Array.of_list arr }
      | _ -> config)
  | "exec-on-resume" -> (
      match value with
      | TomlTypes.TArray (TomlTypes.NodeString arr) ->
          { config with exec_on_resume = Array.of_list arr }
      | _ -> config)
  | _ -> config

let read_file () =
  try
    match Parser.from_filename @@ Path.xdg_file "config.toml" with
    | `Ok table -> TomlTypes.Table.fold of_table table empty
    | `Error (_, _) -> empty
  with _ -> empty
