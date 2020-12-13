open Unix

let start () =
  let fd_null = descr_of_out_channel @@ open_out "/dev/null" in
  let args = [| "comodoro"; "run" |] in
  let pid = create_process args.(0) args fd_null fd_null stderr in
  let out_ch = open_out "/tmp/comodoro.pid" in
  output_string out_ch @@ string_of_int pid;
  close_out out_ch;
  pid

let stop () =
  let in_ch = open_in "/tmp/comodoro.pid" in
  let pid = int_of_string @@ input_line in_ch in
  close_in in_ch;
  kill pid Sys.sigterm;
  Sys.remove "/tmp/comodoro.pid";
  Sys.remove "/tmp/comodoro.sock"
