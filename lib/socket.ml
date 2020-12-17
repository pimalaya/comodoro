open Unix

let get_sock_addr () =
  let path = Path.tmp_file "comodoro.sock" in
  ADDR_UNIX path

let create_and_accept () =
  let sock = socket PF_UNIX SOCK_STREAM 0
  and sock_addr = get_sock_addr ()
  and mutex = Mutex.create ()
  and conns = ref [] in

  let handle_conn conn =
    let chan_in = in_channel_of_descr conn in

    while true do
      try input_line chan_in |> ignore
      with End_of_file ->
        Mutex.lock mutex;
        conns := List.filter (( <> ) conn) !conns;
        Mutex.unlock mutex;
        close_in chan_in;
        Thread.exit ()
    done
  in

  let handle_conns () =
    while true do
      let conn, _ = accept sock in
      Mutex.lock mutex;
      conns := conn :: !conns;
      Mutex.unlock mutex;
      Thread.create handle_conn conn |> ignore
    done
  in

  let broadcast data =
    let send_data conn =
      let chan_out = out_channel_of_descr conn in
      output_string chan_out @@ data ^ "\n";
      flush chan_out
    in

    Mutex.lock mutex;
    List.iter send_data !conns;
    Mutex.unlock mutex
  in

  bind sock sock_addr;
  listen sock 8;
  Thread.create handle_conns () |> ignore;
  broadcast

let rec connect_and_listen handle =
  let sock = socket PF_UNIX SOCK_STREAM 0 in
  let sock_addr = get_sock_addr () in
  let chan_in = in_channel_of_descr sock in

  try
    connect sock sock_addr;
    while true do
      handle @@ input_line chan_in
    done
  with
  | End_of_file ->
      close_in_noerr chan_in;
      connect_and_listen handle
  | Unix.Unix_error (Unix.ENOENT, "connect", "") ->
      print_endline "Comodoro OFF";
      close sock;
      sleep 1;
      connect_and_listen handle
  | err -> raise err
