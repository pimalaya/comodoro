open Unix

let exec_silent cmd = Sys.command (cmd ^ "&") |> ignore

let start () =
  let out_null = open_out Filename.null in
  let fd_null = descr_of_out_channel out_null in
  let args = [| "/bin/bash"; "-c"; "comodoro run" |] in
  let pid = create_process args.(0) args fd_null fd_null stderr in
  let out_ch = open_out @@ Path.tmp_file "comodoro.pid" in
  output_string out_ch @@ string_of_int pid;
  close_out out_ch;
  close_out out_null;
  pid

let stop () =
  try
    let pid_file_path = Path.tmp_file "comodoro.pid"
    and sock_file_path = Path.tmp_file "comodoro.sock" in
    let in_ch = open_in pid_file_path in
    let pid = int_of_string (input_line in_ch) in
    close_in in_ch;
    kill pid Sys.sigterm;
    Sys.remove pid_file_path;
    Sys.remove sock_file_path
  with
  | Sys_error _ -> ()
  | err -> raise err
