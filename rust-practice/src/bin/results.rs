fn main() {
  let age  = "23";
  let result = age.parse::<i32>();
  println!("{:?}" , result);

  is_number(age);
}
fn is_number(num : &str) {
  if let Ok(value) = num.parse::<i32>() {
   println!("Number")
  }else{
   println!("Not a number")
  }
}