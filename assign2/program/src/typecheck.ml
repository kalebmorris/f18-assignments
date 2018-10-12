open Core
open Result.Monad_infix
open Ast

exception Unimplemented

(* You need to implement the statics for the three remaining cases below,
 * Var, Lam, and App. We have provided you with an implementation for Int
 * and Binop that you may refer to.*)
let rec typecheck_term (env : Type.t String.Map.t) (e : Expr.t) : (Type.t, string) Result.t =
  match e with
  | Expr.Int _ -> Ok Type.Int
  | Expr.Binop (_, e1, e2) ->
    typecheck_term env e1
    >>= fun tau1 ->
    typecheck_term env e2
    >>= fun tau2 ->
    (match (tau1, tau2) with
     | (Type.Int, Type.Int) -> Ok Type.Int
     | _ -> Error ("One of the binop operands is not an int"))
  | Expr.Var x -> 
    (match String.Map.find env x with
     | None -> Error ("Unbound variable cannot be typechecked")
     | Some v -> Ok v)
  | Expr.Lam(x, arg_tau, e') ->
    let env1 = String.Map.set ~key:x ~data:arg_tau env in
    (match typecheck_term env1 e' with
     | Ok t -> Ok(Type.Fn(arg_tau, t))
     | _ -> Error ("Lambda function body not well-typed"))
  | Expr.App (fn, arg) ->
    (match (typecheck_term env fn, typecheck_term env arg) with
     | (Ok Type.Fn(t1, t2), Ok t3) when t1 = t3 -> Ok t2 
     | _ -> Error ("Function application with incorrect types"))
  | Expr.Pair (e1, e2) ->
    (match (typecheck_term env e1, typecheck_term env e2) with
     | (Ok t1, Ok t2) -> Ok(Type.Product(t1, t2))
     | _ -> Error ("One of the pair elements is not well-typed"))
  | Expr.Project (e, d) -> 
    (match typecheck_term env e with
     | Ok Type.Product(t1, t2) -> if d = Expr.Left then Ok t1 else Ok t2
     | _ -> Error ("Expression not well-typed"))
  | Expr.Inject (e, d, tau) -> 
    (match (typecheck_term env e, tau) with
     | (Ok t, Type.Sum(t1, t2)) ->
       (if (d = Expr.Left && t = t1) || (d = Expr.Right && t = t2)
        then Ok tau
        else Error ("Expression doesn't match either possible type"))
     | _ -> Error ("Injection expression not well-typed"))
  | Expr.Case (e, (x1, e1), (x2, e2)) -> 
    (match typecheck_term env e with
     | Ok Type.Sum(t1, t2) ->
       let env1 = String.Map.set ~key:x1 ~data:t1 env in
       let env2 = String.Map.set ~key:x2 ~data:t2 env in
       (match (typecheck_term env1 e1, typecheck_term env2 e2) with
        | (Ok t1', Ok t2') when t1' = t2' -> Ok t1'
        | _ -> Error ("Expressions within case statement don't match have same type"))
     | _ -> Error ("Case expression not well-typed"))

let typecheck t = typecheck_term String.Map.empty t

let inline_tests () =
  let e1 = Expr.Lam ("x", Type.Int, Expr.Var "x") in
  assert (typecheck e1 = Ok(Type.Fn(Type.Int, Type.Int)));

  let e2 = Expr.Lam ("x", Type.Int, Expr.Var "y") in
  assert (Result.is_error (typecheck e2));

  let t3 = Expr.App (e1, Expr.Int 3) in
  assert (typecheck t3 = Ok(Type.Int));

  let t4 = Expr.App (t3, Expr.Int 3) in
  assert (Result.is_error (typecheck t4));

  let t5 = Expr.Binop (Expr.Add, Expr.Int 0, e1) in
  assert (Result.is_error (typecheck t5))

(* Uncomment the line below when you want to run the inline tests. *)
let () = inline_tests ()
