#[link(name = "our_code")]
extern "C" {
    fn our_code_starts_here() -> i64;
}

fn main() {
  let i : i64 = unsafe {
    our_code_starts_here()
  };
  println!("{i}");
}
