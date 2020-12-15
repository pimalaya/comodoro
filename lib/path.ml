let join paths =
  let with_dir_sep p p' = p ^ Filename.dir_sep ^ p' in
  Array.fold_left with_dir_sep "" paths

let tmp_join paths =
  let tmp_dir = Filename.get_temp_dir_name () in
  let paths = Array.append [| tmp_dir |] paths in
  join paths

let tmp_file name = tmp_join [| name |]
