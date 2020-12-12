open Unix

let sock_addr = ADDR_UNIX "/tmp/comodoro.sock"

let create_and_accept () =
  let sock = socket PF_UNIX SOCK_STREAM 0 in
  let mutex = Mutex.create () in
  let conns : file_descr list ref = ref [] in
  let add_conn () =
    while true do
      let conn, _ = accept sock in
      Mutex.lock mutex;
      conns := conn :: !conns;
      Mutex.unlock mutex;
      print_endline "Connection added!"
    done
  in
  let broadcast str =
    let send_str conn =
      let out_ch = out_channel_of_descr conn in
      output_string out_ch (str ^ "\n");
      flush out_ch
    in
    Mutex.lock mutex;
    List.iter send_str !conns;
    Mutex.unlock mutex
  in
  bind sock sock_addr;
  listen sock 1024;
  print_endline "Socket open!";
  ignore @@ Thread.create add_conn ();
  broadcast

let connect_and_listen handler =
  let sock = socket PF_UNIX SOCK_STREAM 0 in
  connect sock sock_addr;
  let in_ch = in_channel_of_descr sock in
  while true do
    handler @@ input_line in_ch
  done
