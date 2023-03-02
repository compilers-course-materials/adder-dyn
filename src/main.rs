use sexp::*;
use sexp::Atom::*;

enum Expr {
  Num(i64),
  Add1(Box<Expr>),
  Sub1(Box<Expr>)
}

fn parse_expr(s : &Sexp) -> Expr {
  match s {
    Sexp::Atom(I(n)) => Expr::Num(*n),
    Sexp::List(vec) =>
    match &vec[..] {
      [Sexp::Atom(S(op)), e] if op == "add1" => Expr::Add1(Box::new(parse_expr(e))),
      [Sexp::Atom(S(op)), e] if op == "sub1" => Expr::Sub1(Box::new(parse_expr(e))),
      _ => panic!("parse error")
    },
    _ => panic!("parse error")
  }
}

fn compile_expr(e : &Expr, cmds : &mut Vec<String>) {
  match e {
    Expr::Num(n) => cmds.push("mov rax, {n}"),
    Expr::Add1(subexpr) => {
      compile_expr(&subexpr, cmds);
      cmds.push("add rax, 1");
    },
    Expr::Sub1(subexpr) => {
      compile_expr(&subexpr, cmds);
      cmds.push("sub rax, 1");
    }
    _ => panic!("unknown")
  }
}

fn compile(e : &Expr) -> Vec<&str> {
  let mut v : Vec<&str> = Vec::new();
  compile_expr(e, &mut v);
  v
}

fn main() {
  let s : Sexp = parse("99").unwrap();
  if let Sexp::Atom(I(n)) = s {
    println!("{n}")
  };

  let expr = parse_expr(&parse("(add1 5)").unwrap());
  let result = compile(&expr);

  println!("{}", result.iter().map(|e| { e.to_string() }).collect::<Vec<_>>().join("\n"));
 
}
