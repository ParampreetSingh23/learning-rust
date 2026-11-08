use std::io::{self, Read};
fn main(){
 println!("{}",getRandom());
  let mut name = String::new();
  io::stdin().read_line(&mut name).unwrap();
  println!("{}", name);
}
fn getRandom()->u32{
  let num = rand::random_range(1..=10);
  return num
 

}