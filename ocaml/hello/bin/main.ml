let () = print_endline "Hello, Ocaml!"
let () = Printf.printf "%s!\n" Hello.Es.v
let year = read_int ()

let () = print_endline (string_of_int year)

let is_leap_year year =
  if year mod 4 = 0 then
    if year mod 100 = 0 then
      year mod 400 = 0
    else
      true
  else
    false;;

is_leap_year 1890;;

