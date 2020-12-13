open Unix

let sock_addr = ADDR_UNIX "/tmp/comodoro.sock"

let create_and_accept () =
  let sock = socket PF_UNIX SOCK_STREAM 0
  and mutex = Mutex.create ()
  and conns = ref [] in

  let handle_conn conn =
    let in_ch = in_channel_of_descr conn in
    while true do
      try input_line in_ch |> ignore
      with End_of_file ->
        Mutex.lock mutex;
        conns := List.filter (( <> ) conn) !conns;
        Mutex.unlock mutex;
        close_in in_ch;
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
      let out_ch = out_channel_of_descr conn in
      output_string out_ch @@ data ^ "\n";
      flush out_ch
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
  try
    let sock = socket PF_UNIX SOCK_STREAM 0 in
    let in_ch = in_channel_of_descr sock in
    connect sock sock_addr;

    while true do
      handle @@ input_line in_ch
    done
  with _ ->
    print_endline "Comodoro";
    sleep 1;
    connect_and_listen handle