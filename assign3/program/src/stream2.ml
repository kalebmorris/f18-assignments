open Core
open List2.MyList

exception Unimplemented

module type Stream2 = sig
  type 'a stream = Stream of (unit -> 'a * 'a stream)
  val head : 'a stream -> 'a
  val tail : 'a stream -> 'a stream
  val take : 'a stream -> int -> 'a list * 'a stream
  val zip : 'a stream -> 'b stream -> ('a * 'b) stream
  val enumerate : 'a stream -> (int * 'a) stream
  val windows : 'a stream -> int -> ('a list) stream
end

module MyStream : Stream2 = struct
  type 'a stream = Stream of (unit -> 'a * 'a stream)

  let head (Stream f) =
    match (f ()) with
    | (value, _) -> value

  let tail (Stream f) =
    match (f ()) with
    | (_, stream) -> stream

  let rec take s n =
    if n = 0 
      then (Nil, s)
    else 
      match (take (tail s) (n - 1)) with
      | (l, s') -> (Cons(head s, l), s')

  let rec zip (Stream a) (Stream b) =
    Stream (fun () -> ((head (Stream a), head (Stream b)), (zip (tail (Stream a)) (tail (Stream b)))))

  (* let enumerate s =
    let rec indices (base : int) : int stream =
      Stream (fun () -> (base, indices(base + 1)))
    in
    zip (indices 0) s
  *)

  let enumerate s =
    let rec enumerate_rec n (Stream s') =
      Stream (fun () -> ((n, head (Stream s')), enumerate_rec (n + 1) (tail (Stream s'))))
    in
    enumerate_rec 0 s


  let rec windows s n =
    let rec window s n =
      if n = 0 then Nil
      else Cons(head s, window (tail s) (n - 1))
    in
    Stream (fun () -> ((window s n), (windows (tail s) n)))

end

module StreamTests(S : Stream2) = struct
  open S ;;

  let rec repeat (n : int) : int stream =
    Stream (fun () -> (n, repeat (n)))
  ;;
  let s = enumerate (repeat 1) in 
  assert (head s = (0, 1));
  assert (head (tail s) = (1, 1));
  assert (head (tail (tail s)) = (2, 1));
  assert (head (tail (tail (tail s))) = (3, 1));

  let s = zip (repeat 1) (repeat 2) in
  assert (head s = (1, 2));
  assert (head (tail s) = (1, 2));
  assert (head (tail (tail s)) = (1, 2));

  let s = enumerate (repeat 5) in
  let (l, s) = take s 2 in
  assert (l = Cons((0, 5), Cons((1, 5), Nil)));
  assert (head s = (2, 5));

  let s = windows (repeat 4) 3 in
  assert (head s = (Cons(4, Cons(4, Cons(4, Nil)))));
  assert (head (tail s) = (Cons(4, Cons(4, Cons(4, Nil)))))
  ;;
  
  let rec upfrom (n : int) : int stream =
    Stream (fun () -> (n, upfrom (n + 1)))
  ;;

  let s = upfrom 5 in
  assert (head s = 5);
  assert (head (tail s) = 6);

  let s = enumerate (upfrom 5) in
  let (l, s) = take s 2 in
  assert (l = Cons((0, 5), Cons((1, 6), Nil)));
  assert (head s = (2, 7));

  let s = windows (upfrom 4) 3 in
  assert (head s = (Cons(4, Cons(5, Cons(6, Nil)))));
  assert (head (tail s) = (Cons(5, Cons(6, Cons(7, Nil)))))
  ;;

  let fib () : int stream =
    let rec fib_rec (n : int) (m : int) : int stream =
      Stream (fun () -> (n, fib_rec (m) (n + m)))
    in
    fib_rec 0 1
  ;;

  let s = fib () in
  assert (head s = 0);
  assert (head (tail s) = 1);
  assert (head (tail (tail s)) = 1);
  assert (head (tail (tail (tail s))) = 2);
 
  let s = zip (fib ()) (repeat 2) in
  assert (head s = (0, 2));
  assert (head (tail s) = (1, 2));
  assert (head (tail (tail s)) = (1, 2));
  assert (head (tail (tail (tail s))) = (2,2));

end

module MyStreamTests = StreamTests(MyStream)
