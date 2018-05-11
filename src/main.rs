mod error;

#[macro_use]
extern crate text_io;

use error::{
  ResultC,
  ConcurrencyError
};

use std::{
  ops::Add,
  error::Error,
  collections::BTreeMap,
  io::{Write, stdout}
};

macro_rules! scan {
  () => ( { stdout().flush().expect("Failed to flush"); read!() } );
}

struct ConcurrencyControl<T> where T: Clone + Add<Output = T> {
  vars: BTreeMap<*const T ,T>
}

impl<T> ConcurrencyControl<T> where T: Clone + Add<Output = T> {
  fn new() -> Self {
    ConcurrencyControl { vars: BTreeMap::new() }
  }

  fn read_var(&mut self, x: &T) -> ResultC<()> {
    if self.vars.contains_key(&(x as *const T)) {
      return Err(ConcurrencyError::new("Trying to read a variable that already was readed!"))
    }

    self.vars.insert(x as *const T, x.clone());
    Ok(())
  }

  fn write_var(&mut self, x: &mut T) -> ResultC<()> {
    if !self.vars.contains_key(&(x as *const T)) {
      return Err(ConcurrencyError::new("Variable was not read in the first place!"));
    }

    let val = self.vars.get(&(x as *const T)).unwrap().clone();
    *x = val;
    self.vars.remove(&(x as *const T));
    Ok(())
  }

  #[allow(dead_code)]
  fn assign_val(&mut self, x: &T, val: T) -> ResultC<()> {
    if !self.vars.contains_key(&(x as *const T)) {
      return Err(ConcurrencyError::new("Variable was not read in the first place!"));
    }

    let var = self.vars.get_mut(&(x as *const T)).unwrap();
    *var = val;
    Ok(())
  }

  fn add_val(&mut self, x: &T, val: T) -> ResultC<()> {
    if !self.vars.contains_key(&(x as *const T)) {
      return Err(ConcurrencyError::new("Variable was not read in the first place!"));
    }

    let var = self.vars.get_mut(&(x as *const T)).unwrap();
    *var = var.clone() + val;
    Ok(())
  }
}

fn main() -> Result<(), Box<Error>> {
  let mut cc = ConcurrencyControl::new();
  eprint!("Initial values for X and Y: ");
  let mut x: isize = scan!();
  let mut y: isize = scan!();

  eprint!("Values for M and N: ");
  let m: isize = scan!();
  let n: isize = scan!();

  eprintln!("Transaction 1:");
  eprintln!("a) read_item(X)");
  eprintln!("b) X := X - N");
  eprintln!("c) write_item(X)");
  eprintln!("d) read_item(Y)");
  eprintln!("e) Y := Y + N");
  eprintln!("f) write_item(Y)");
  eprintln!("Transaction 2:");
  eprintln!("g) read_item(X)");
  eprintln!("h) X := X + M");
  eprintln!("i) write_item(X)");

  let mut order = Vec::new();
  for _ in 0..9 {
    let temp: char = scan!();
    if !(temp >= 'a' && temp <= 'i') {
      panic!("Unknown option");
    }
    order.push(temp);
  }

  for i in &order {
    match i {
      'a' => cc.read_var(&x)?,
      'b' => cc.add_val(&mut x, -n)?,
      'c' => cc.write_var(&mut x)?,
      'd' => cc.read_var(&y)?,
      'e' => cc.add_val(&mut y, n)?,
      'f' => cc.write_var(&mut y)?,
      'g' => cc.read_var(&x)?,
      'h' => cc.add_val(&mut x, m)?,
      'i' => cc.write_var(&mut x)?,
       _  => { panic!("Get an unknown operation!"); }
    };
  }

  println!("\nResults:");
  println!("X: {}, Y: {}", x, y);
  Ok(())
}
