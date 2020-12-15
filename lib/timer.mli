type t = Work of int * int | ShortBreak of int * int | LongBreak of int * int

val run : ?timer:t -> Config.t -> (string -> unit) -> unit
