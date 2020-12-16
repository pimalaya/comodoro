type t = Work of int * int | ShortBreak of int * int | LongBreak of int * int

val initial_timer : Config.t -> t

val run : Config.t -> t -> (string -> unit) -> unit
