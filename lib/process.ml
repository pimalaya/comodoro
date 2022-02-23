open Unix

let exec_silent cmd = Sys.command (cmd ^ "&") |> ignore

let exec_silent_all = Array.iter exec_silent

let kill_if_exists pid_file_path =
  try
    let in_ch = open_in pid_file_path in
    let pid = int_of_string (input_line in_ch) in
    close_in in_ch;
    kill pid Sys.sigterm
  with _ -> ()

let remove_if_exists file_path =
  try Sys.remove file_path with Sys_error _ -> ()

let start () =
  let config = Config.read_file () in
  let out_null = open_out Filename.null in
  let fd_null = descr_of_out_channel out_null in
  let args = [| "/bin/bash"; "-c"; Sys.argv.(0) ^ " run" |] in
  let pid = create_process args.(0) args fd_null fd_null stderr in
  let out_ch = open_out @@ Path.tmp_file "comodoro.pid" in
  output_string out_ch @@ string_of_int pid;
  close_out out_ch;
  close_out out_null;
  exec_silent_all config.exec_on_start;
  pid

let stop () =
  let config = Config.read_file ()
  and pid_file_path = Path.tmp_file "comodoro.pid"
  and sock_file_path = Path.tmp_file "comodoro.sock" in

  kill_if_exists pid_file_path;
  exec_silent_all config.exec_on_stop;
  remove_if_exists pid_file_path;
  remove_if_exists sock_file_path
