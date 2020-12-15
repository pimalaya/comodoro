let ( >>= ) = Option.bind

let ( <|> ) a b = match a with Some _ -> a | None -> b

let join paths =
  let with_dir_sep p p' = p ^ Filename.dir_sep ^ p' in
  Array.fold_left with_dir_sep "" paths

let tmp_join paths =
  let tmp_dir = Filename.get_temp_dir_name () in
  let paths = Array.append [| tmp_dir |] paths in
  join paths

let tmp_file name = tmp_join [| name |]

let xdg_file name =
  let xdg_path path = Some (join [| path; "comodoro" |]) in
  let from_xdg = Sys.getenv_opt "XDG_CONFIG_HOME" >>= xdg_path in
  let home_path path = Some (join [| path; ".config"; "comodoro" |]) in
  let from_home = Sys.getenv_opt "HOME" >>= home_path in
  let from_tmp = tmp_join [| "comodoro" |] in
  let xdg_dir = Option.value ~default:from_tmp (from_xdg <|> from_home) in
  join [| xdg_dir; name |]
