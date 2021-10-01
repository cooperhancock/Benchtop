let rec binary_search (lst: int list) (p: int) (q: int) (v: int) : int =
let m = (p+q)/2
in if List.nth lst m = v then m
else if q-p = 0 then -1
else if v < List.nth lst m then binary_search lst p m v
else
let n = m+1 in binary_search lst n q v

let rec create_list (lst: int list) (x: int) (len: int) : int list =
if x = len then lst else let y = x+1 in x :: create_list lst y len

(* main program *)
let run_bs (size: int) : (string * int * string * int * string * float) =
let timeStart = Unix.gettimeofday ()
in let biglist = create_list [] 0 size
in let res1 = binary_search biglist 0 (size-1) (size-1)
in let res2 = binary_search biglist 0 (size-1) size
in ("has last item?", res1, "has item after last?", res2, "time:", (Unix.gettimeofday ())-.timeStart) 
