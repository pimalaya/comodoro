type t = {
  work_time : int;
  short_break_time : int;
  long_break_time : int;
  exec_on_start : string array;
  exec_on_break : string array;
  exec_on_resume : string array;
  exec_on_stop : string array;
}

val read_file : unit -> t
