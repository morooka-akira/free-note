open Stdio

let line = In_channel.input_line In_channel.stdin
let () = match line with
  | Some s -> printf "Total: %s\n" s
  | None -> ()
