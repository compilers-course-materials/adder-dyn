use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};

use std::mem;

use sexp::*;
use sexp::Atom::*;

enum Val {
  Reg(Reg),
  Imm(i32),
}

use Val::*;

enum Reg {
  RAX
}

use Reg::*;

enum Instr {
  IMov(Val, Val),
  IAdd(Val, Val),
  ISub(Val, Val),
}

enum Expr {
  Num(i32),
  Add1(Box<Expr>),
  Sub1(Box<Expr>)
}

fn parse_expr(s : &Sexp) -> Expr {
  match s {
    Sexp::Atom(I(n)) => Expr::Num(i32::try_from(*n).unwrap()),
    Sexp::List(vec) =>
    match &vec[..] {
      [Sexp::Atom(S(op)), e] if op == "add1" => Expr::Add1(Box::new(parse_expr(e))),
      [Sexp::Atom(S(op)), e] if op == "sub1" => Expr::Sub1(Box::new(parse_expr(e))),
      _ => panic!("parse error")
    },
    _ => panic!("parse error")
  }
}

fn val_to_str(v : &Val) -> String {
  match v {
    Reg(RAX) => String::from("RAX"),
    Imm(n) => format!("DWORD {n}")
  }
}

fn reg_to_index(r : &Reg) -> u8 {
  match r {
    RAX => 0
  }
}

fn instr_to_str(i : &Instr) -> String {
  match i {
    Instr::IMov(v1, v2) => {
      return format!("mov {}, {}", val_to_str(&v1), val_to_str(&v2));
    },
    Instr::ISub(v1, v2) => {
      return format!("sub {}, {}", val_to_str(&v1), val_to_str(&v2));
    }
    Instr::IAdd(v1, v2) => {
      return format!("add {}, {}", val_to_str(&v1), val_to_str(&v2));
    }
  }
}

fn instrs_to_str(cmds : &Vec<Instr>) -> String {
  cmds.iter().map(|c| { instr_to_str(c) }).collect::<Vec<_>>().join("\n")
}

fn instr_to_asm(i : &Instr, ops : &mut dynasmrt::x64::Assembler) {
  match i {
    Instr::IMov(Reg(r), Imm(n)) => {
      dynasm!(ops ; .arch x64 ; mov Rq(reg_to_index(r)), *n);
    },
    Instr::IAdd(Reg(r), Imm(n)) => {
      dynasm!(ops ; .arch x64 ; add Rq(reg_to_index(r)), *n);
    }
    Instr::ISub(Reg(r), Imm(n)) => {
      dynasm!(ops ; .arch x64 ; sub Rq(reg_to_index(r)), *n);
    }
    _ => {
      panic!("Unknown instruction format")
    }
  }
}

fn instrs_to_asm(cmds : &Vec<Instr>, ops : &mut dynasmrt::x64::Assembler) {
  cmds.iter().for_each(|c| { instr_to_asm(c, ops) })
}

fn jit_expr(e : &Expr, ops : &mut dynasmrt::x64::Assembler) {
  match e {
    Expr::Num(n) => {
      dynasm!(ops
        ; .arch x64
        ; mov rax, DWORD *n
      );
    }
    Expr::Add1(subexpr) => {
      jit_expr(&subexpr, ops);
      dynasm!(ops
        ; .arch x64
        ; add rax, DWORD 1
      );
    },
    Expr::Sub1(subexpr) => {
      jit_expr(&subexpr, ops);
      dynasm!(ops
        ; .arch x64
        ; sub rax, DWORD 1
      );
    }
  }
}

fn compile_expr(e : &Expr, cmds : &mut Vec<Instr>) {
  match e {
    Expr::Num(n) => cmds.push(Instr::IMov(Reg(RAX), Imm(*n))),
    Expr::Add1(subexpr) => {
      compile_expr(&subexpr, cmds);
      cmds.push(Instr::IAdd(Reg(RAX), Imm(1)))
    },
    Expr::Sub1(subexpr) => {
      compile_expr(&subexpr, cmds);
      cmds.push(Instr::ISub(Reg(RAX), Imm(1)))
    }
  }
}

fn compile_to_instrs(e : &Expr) -> Vec<Instr> {
  let mut v : Vec<Instr> = Vec::new();
  compile_expr(e, &mut v);
  return v;
}

fn compile(e : &Expr) -> String {
  let mut v : Vec<Instr> = Vec::new();
  compile_expr(e, &mut v);
  return instrs_to_str(&v)
}

fn main() {
  let expr = parse_expr(&parse("(add1 (sub1 (add1 73)))").unwrap());
  let result = compile(&expr);

  println!("{}", result);

  let instrs = compile_to_instrs(&expr);

  let mut ops = dynasmrt::x64::Assembler::new().unwrap();
  let start = ops.offset();

  instrs_to_asm(&instrs, &mut ops);

  dynasm!(ops
    ; .arch x64
    ; ret);
  let buf = ops.finalize().unwrap();
  let jitted_fn : extern fn() -> i32 = unsafe { mem::transmute(buf.ptr(start)) };

  println!("{}", jitted_fn());
 
}
