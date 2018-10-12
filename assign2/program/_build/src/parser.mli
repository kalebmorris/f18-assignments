
(* The type of tokens. *)

type token = 
  | VAR of (string)
  | TY_INT
  | SUB
  | RPAREN
  | RIGHT
  | RBRACE
  | PLUS
  | MUL
  | LPAREN
  | LEFT
  | LBRACE
  | INT of (int)
  | INJECT
  | FN
  | FATARROW
  | EQUAL
  | EOF
  | DOT
  | DIV
  | COMMA
  | COLON
  | CASE
  | BAR
  | AS
  | ARROW

(* This exception is raised by the monolithic API functions. *)

exception Error

(* The monolithic API. *)

val main: (Lexing.lexbuf -> token) -> Lexing.lexbuf -> (Ast.Expr.t)
