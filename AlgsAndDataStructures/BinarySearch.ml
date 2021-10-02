let rec binary_search (lst: int array) (p: int) (q: int) (v: int) : int =
  let m = (p+q)/2
  in if lst.(m) = v then m
  else if q-p = 0 then -1
  else if v < lst.(m) then binary_search lst p m v
  else
  let n = m+1 in binary_search lst n q v
  
  (* main program *)
let run_bs (size: int) : (string * int * string * int * string * float) =
  let timeStart = Unix.gettimeofday ()
  in let biglist = (Array.make size 0)
  in for x=0 to size-1 do biglist.(x) <- x done;
  let res1 = binary_search biglist 0 (size-1) (size-1)
  in let res2 = binary_search biglist 0 (size-1) size
  in ("has last item?", res1, "has item after last?", res2, "time:", (Unix.gettimeofday ())-.timeStart) 
