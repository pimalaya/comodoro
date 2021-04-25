open Lib
open Core

let start () =
  Command.basic ~summary:"start a new pomodoro"
    (Command.Param.return (fun () ->
         let pid = Process.start () in
         Printf.printf "Timer started (pid=%d).\n" pid))

let run () =
  Command.basic ~summary:"start and watch a pomodoro"
    (Command.Param.return (fun () ->
         let config = Config.read_file () in
         let initial_timer = Timer.initial_timer config in
         let broadcaster = Socket.create_and_accept () in
         Timer.run config initial_timer broadcaster))

let watch () =
  Command.basic ~summary:"watch an existing pomodoro"
    (Command.Param.return (fun () ->
         let handler data = print_endline data in
         Socket.connect_and_listen handler))

let stop () =
  Command.basic ~summary:"stop a pomodoro"
    (Command.Param.return (fun () ->
         Process.stop ();
         print_endline "Timer stopped."))

let () =
  Command.run
    (Command.group
       ~summary:"Socket-based CLI timer following the Pomodoro principles"
       [
         ("start", start ());
         ("run", run ());
         ("watch", watch ());
         ("stop", stop ());
       ])
