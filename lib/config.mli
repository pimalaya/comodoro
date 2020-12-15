type t = {
  exec_on_start : string array;
  exec_on_break : string array;
  exec_on_resume : string array;
  exec_on_stop : string array;
}

val read_file : unit -> t
