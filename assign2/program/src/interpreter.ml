open Core
open Ast

type outcome =
  | Step of Expr.t
  | Val

exception RuntimeError of string
exception Unimplemented

(* You will implement the App, Binop, Pair, Project, Inject, and Case cases
   below. See the dynamics section for a specification on how the small step
   semantics should work. *)
let rec trystep e =
  match e with
  | Expr.Var _ -> raise (RuntimeError "Unreachable")
  | (Expr.Lam _ | Expr.Int _) -> Val
  | Expr.App (fn, arg) -> 
    (match trystep fn with
     | Step s -> Step(Expr.App(s, arg))
     | Val -> 
       (match fn with
        | Expr.Lam (x', _, body) -> Step(Expr.substitute x' arg body)
        | _ -> raise (RuntimeError "Unreachable")))
  | Expr.Binop (binop, left, right) -> 
    (match trystep left with
     | Step l -> Step(Expr.Binop(binop, l, right))
     | Val ->
       (match trystep right with
        | Step r -> Step(Expr.Binop(binop, left, r))
        | Val ->
          (match (binop, left, right) with
           | (Expr.Add, Expr.Int l, Expr.Int r) -> Step(Expr.Int(l + r))
           | (Expr.Sub, Expr.Int l, Expr.Int r) -> Step(Expr.Int(l - r))
           | (Expr.Mul, Expr.Int l, Expr.Int r) -> Step(Expr.Int(l * r))
           | (Expr.Div, Expr.Int _, Expr.Int r) when r = 0 -> Step(Expr.Int(0))
           | (Expr.Div, Expr.Int l, Expr.Int r) -> Step(Expr.Int(l / r))
           | _ -> raise (RuntimeError "Unreachable"))))
  | Expr.Pair (e1, e2) ->
    (match trystep e1 with
     | Step l -> Step(Expr.Pair(l, e2))
     | Val ->
       (match trystep e2 with
        | Step r -> Step(Expr.Pair(e1, r))
        | Val -> Val))
  | Expr.Project (e, dir) -> 
    (match trystep e with
     | Step e' -> Step(Expr.Project(e', dir))
     | Val -> 
       (match e with
        | Expr.Pair(e1, e2) -> if dir = Expr.Left then Step(e1) else Step(e2)
        | _ -> raise (RuntimeError "Unreachable")))
  | Expr.Inject (e, dir, tau) ->
    (match trystep e with
     | Step e' -> Step(Expr.Inject(e', dir, tau))
     | Val -> Val)
  | Expr.Case (e, (x1, e1), (x2, e2)) ->
    (match trystep e with
     | Step e' -> Step(Expr.Case(e', (x1, e1), (x2, e2)))
     | Val ->
       (match e with
        | Expr.Inject(e0, dir, _) -> 
          (if dir = Expr.Left then Step(Expr.substitute x1 e0 e1) else Step(Expr.substitute x2 e0 e2))
        | _ -> raise (RuntimeError "Unreachable")))

let rec eval e =
  match trystep e with
  | Step e' -> eval e'
  | Val -> Ok e

let inline_tests () =
  let e1 = Expr.Binop(Expr.Add, Expr.Int 2, Expr.Int 3) in
  assert (trystep e1 = Step(Expr.Int 5));

  let e2 = Expr.App(Expr.Lam("x", Type.Int, Expr.Var "x"), Expr.Int 3) in
  assert (trystep e2 = Step(Expr.Int 3))

(* Uncomment the line below when you want to run the inline tests. *)
let () = inline_tests ()
