fn greet(name:&str) {
 println!("Hello i am {}" , name);
}
fn add(x : i32 , y : i32) {
 println!("Sum : {} ",x+y)
}
fn is_pass(pass:&str) -> bool {
  if pass == "parampreet" {
   return true
  }
  false
}
fn main() {
 greet("Param");
 add(2,3);
 let flag = is_pass("prampreet");
 println!("{}" , flag)

}