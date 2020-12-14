open Lib

let start () =
  let pid = Process.start () in
  Printf.printf "Timer started (pid=%d).\n" pid

let stop () =
  Process.stop ();
  print_endline "Timer stopped."

let run () =
  let config = Config.read_file () in
  let broadcast_str = Socket.create_and_accept () in
  Timer.run config broadcast_str

let watch () =
  let handler data = print_endline data in
  Socket.connect_and_listen handler

let () =
  let argv_last_idx = Array.length Sys.argv - 1 in
  let arg_list = Array.to_list @@ Array.sub Sys.argv 1 argv_last_idx in
  match arg_list with
  | "start" :: _ -> start ()
  | "stop" :: _ -> stop ()
  | "run" :: _ -> run ()
  | "watch" :: _ -> watch ()
  | _ -> print_endline "Command not found"
