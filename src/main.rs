use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};

use std::{io, mem, slice};
use std::io::Write;


use sexp::*;
use sexp::Atom::*;

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

fn libcall() {
    let mut ops = dynasmrt::x64::Assembler::new().unwrap();
    let string = "Hello World!";

    dynasm!(ops
        ; .arch x64
        ; ->hello:
        ; .bytes string.as_bytes()
    );

    let hello = ops.offset();
    dynasm!(ops
        ; .arch x64
        ; lea rcx, [->hello]
        ; xor edx, edx
        ; mov dl, BYTE string.len() as _
        ; mov rax, QWORD print as _
        ; sub rsp, BYTE 0x28
        ; call rax
        ; add rsp, BYTE 0x28
        ; ret
    );

    println!("{:?}", ops);


    let buf = ops.finalize().unwrap();

    let hello_fn: extern "Rust" fn() -> bool = unsafe { mem::transmute(buf.ptr(hello)) };

    assert!(hello_fn());
}

pub extern "Rust" fn print(buffer: *const u8, length: u64) -> bool {
    io::stdout()
        .write_all(unsafe { slice::from_raw_parts(buffer, length as usize) })
        .is_ok()
}


fn compile_expr(e : &Expr, cmds : &mut Vec<String>) {
  match e {
    Expr::Num(n) => cmds.push(format!("mov rax, {n}")),
    Expr::Add1(subexpr) => {
      compile_expr(&subexpr, cmds);
      cmds.push(String::from("add rax, 1"));
    },
    Expr::Sub1(subexpr) => {
      compile_expr(&subexpr, cmds);
      cmds.push(String::from("sub rax, 1"));
    }
  }
}

fn compile(e : &Expr) -> Vec<String> {
  let mut v : Vec<String> = Vec::new();
  compile_expr(e, &mut v);
  v
}

fn main() {
  let expr = parse_expr(&parse("(add1 (sub1 (add1 73)))").unwrap());
  let result = compile(&expr);

  println!("{}", result.iter().map(|e| { e.to_string() }).collect::<Vec<_>>().join("\n"));

  let mut ops = dynasmrt::x64::Assembler::new().unwrap();
  let start = ops.offset();

  libcall();

/*
  jit_expr(&expr, &mut ops);
  dynasm!(ops; ret);
  let buf = ops.finalize().unwrap();
  let jitted_fn : extern fn() -> i32 = unsafe { mem::transmute(buf.ptr(start)) };

  println!("{}", jitted_fn());
  */
 
}
