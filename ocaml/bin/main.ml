open Base
open Stdlib
open Str

let get_input : string list =
let read_file (filename: string) : string list =
  try
    let channel = open_in filename in
    try
      let rec read_lines acc =
        try
          let line = input_line channel in
          read_lines (line :: acc)
        with End_of_file ->
          List.rev acc
      in
      let lines = read_lines [] in
      close_in channel;
      lines
    with e ->
      close_in channel;
      raise e
  with Sys_error msg ->
    prerr_endline ("Cannot open file: " ^ msg);
    []
  in
  let filename = "input.txt" in
  read_file filename
  

let () =
  let reg = regexp {|\d|} in
  print_endline "haiiii";
  Printf.printf "%d\n" @@ search_forward reg "abc1" 0;
  print_endline "byeeee";
  let first_digit s = int_of_char s.[search_forward reg s 0] - int_of_char '0' in 
  let last_digit s = int_of_char s.[search_backward reg s (String.length s - 1)] - int_of_char '0' in 
  let input = get_input in 
  let lines = List.map (fun line -> (first_digit line * 10) + last_digit line) input in 
  let sum = List.fold_left (fun sum x -> sum + x) 0 lines in 
  Printf.printf "%d\n" sum;